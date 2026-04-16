use std::collections::BTreeSet;
use std::ffi::OsStr;
use std::path::Path;

use g3rs_deps_types::{
    G3RsDepsConfigChecksInput, G3RsDepsConfigInputScope, G3RsDepsFileTreeChecksInput,
    G3RsDepsSourceChecksInput,
};
use g3rs_workspace_crawl::G3RsWorkspaceCrawl;
use glob::Pattern;
use guardrail3_rs_toml_parser::RustProfile;

/// Re-export of `G3RsDepsIngestionError` so the facade can reach it.
pub use g3rs_deps_ingestion_types::G3RsDepsIngestionError as IngestionError;

/// Ingest workspace deps config from a workspace crawl into per-crate checks inputs.
pub fn ingest_for_config_checks(
    crawl: &G3RsWorkspaceCrawl,
) -> Result<Vec<G3RsDepsConfigChecksInput>, IngestionError> {
    ingest_for_config_checks_with_path(crawl, std::env::var_os("PATH").as_deref())
}

pub(crate) fn ingest_for_config_checks_with_path(
    crawl: &G3RsWorkspaceCrawl,
    path_env: Option<&OsStr>,
) -> Result<Vec<G3RsDepsConfigChecksInput>, IngestionError> {
    let workspace_cargo_entry = crate::select::select_workspace_cargo_toml(crawl)
        .ok_or(IngestionError::CargoTomlNotFound)?;
    if !workspace_cargo_entry.readable {
        return Err(IngestionError::Unreadable {
            path: workspace_cargo_entry.path.abs_path.clone(),
            reason: "file is not readable".to_owned(),
        });
    }

    let guardrail_entry = crate::select::select_workspace_guardrail3_rs_toml(crawl)
        .ok_or(IngestionError::Guardrail3RsTomlNotFound)?;
    if !guardrail_entry.readable {
        return Err(IngestionError::Unreadable {
            path: guardrail_entry.path.abs_path.clone(),
            reason: "file is not readable".to_owned(),
        });
    }

    let workspace_cargo = crate::parse::parse_cargo_toml(&workspace_cargo_entry.path.abs_path)?;
    let guardrail = crate::parse::parse_guardrail3_rs_toml(&guardrail_entry.path.abs_path)?;
    if guardrail
        .config
        .allowed_deps
        .iter()
        .any(|dependency| dependency.trim().is_empty())
    {
        return Err(IngestionError::NormalizationFailed {
            path: guardrail_entry.path.abs_path.clone(),
            reason: "allowed_deps must not contain empty dependency names".to_owned(),
        });
    }

    let member_entries = crate::select::select_member_cargo_tomls(crawl, &workspace_cargo)
        .map_err(|reason| IngestionError::NormalizationFailed {
            path: workspace_cargo_entry.path.abs_path.clone(),
            reason,
        })?;
    let workspace_root_abs_dir = workspace_cargo_entry
        .path
        .abs_path
        .parent()
        .ok_or_else(|| IngestionError::NormalizationFailed {
            path: workspace_cargo_entry.path.abs_path.clone(),
            reason: "workspace root Cargo.toml has no parent directory".to_owned(),
        })?;
    let workspace_member_dirs = member_entries
        .iter()
        .filter_map(|entry| crate::select::member_dir_from_manifest_path(&entry.path.rel_path))
        .map(str::to_owned)
        .collect::<BTreeSet<_>>();

    let mut inputs = Vec::new();
    inputs.push(G3RsDepsConfigChecksInput {
        scope: G3RsDepsConfigInputScope::WorkspaceTooling,
        crate_cargo_rel_path: workspace_cargo_entry.path.rel_path.clone(),
        crate_name: "workspace".to_owned(),
        profile: None,
        allowlist_present: false,
        allowed_deps: Vec::new(),
        dependencies: Vec::new(),
        installed_tools: discover_installed_tools(path_env),
    });
    for member_entry in member_entries {
        if !member_entry.readable {
            return Err(IngestionError::Unreadable {
                path: member_entry.path.abs_path.clone(),
                reason: "file is not readable".to_owned(),
            });
        }

        let member_cargo = crate::parse::parse_cargo_toml(&member_entry.path.abs_path)?;
        let input = crate::ingest::assemble(
            member_entry.path.rel_path.clone(),
            &member_cargo,
            &workspace_cargo,
            &guardrail.config,
            guardrail.allowlist_present,
            &workspace_member_dirs,
            workspace_root_abs_dir,
        )
        .map_err(|reason| IngestionError::NormalizationFailed {
            path: member_entry.path.abs_path.clone(),
            reason,
        })?;
        inputs.push(input);
    }

    Ok(inputs)
}

/// Stub source ingestion entry point for the deps family.
pub fn ingest_for_source_checks(
    _crawl: &G3RsWorkspaceCrawl,
) -> Result<G3RsDepsSourceChecksInput, IngestionError> {
    Err(IngestionError::SourceIngestionNotImplemented)
}

pub fn ingest_for_file_tree_checks(
    crawl: &G3RsWorkspaceCrawl,
) -> Result<G3RsDepsFileTreeChecksInput, IngestionError> {
    let cargo_lock_rel_path = "Cargo.lock".to_owned();
    let cargo_lock_exists =
        g3rs_workspace_crawl::root_file(crawl, cargo_lock_rel_path.as_str()).is_some();
    let profile = read_root_profile(crawl)?;
    let (cargo_lock_ignored, gitignore_rel_path) = read_lockfile_ignore_state(crawl)?;

    Ok(G3RsDepsFileTreeChecksInput {
        profile,
        cargo_lock_rel_path,
        cargo_lock_exists,
        cargo_lock_ignored,
        gitignore_rel_path,
    })
}

fn read_root_profile(crawl: &G3RsWorkspaceCrawl) -> Result<Option<RustProfile>, IngestionError> {
    let Some(guardrail_entry) = crate::select::select_workspace_guardrail3_rs_toml(crawl) else {
        return Ok(None);
    };
    if !guardrail_entry.readable {
        return Err(IngestionError::Unreadable {
            path: guardrail_entry.path.abs_path.clone(),
            reason: "file is not readable".to_owned(),
        });
    }

    let guardrail = crate::parse::parse_guardrail3_rs_toml(&guardrail_entry.path.abs_path)?;
    Ok(guardrail.config.profile)
}

fn read_lockfile_ignore_state(
    crawl: &G3RsWorkspaceCrawl,
) -> Result<(bool, Option<String>), IngestionError> {
    let Some(gitignore_entry) = g3rs_workspace_crawl::root_file(crawl, ".gitignore") else {
        return Ok((false, None));
    };
    if !gitignore_entry.readable {
        return Err(IngestionError::Unreadable {
            path: gitignore_entry.path.abs_path.clone(),
            reason: "file is not readable".to_owned(),
        });
    }

    let content = crate::fs::read_to_string(&gitignore_entry.path.abs_path).map_err(|err| {
        IngestionError::Unreadable {
            path: gitignore_entry.path.abs_path.clone(),
            reason: err.to_string(),
        }
    })?;

    let mut ignored = false;
    for line in content.lines() {
        if let Some(next_ignored) = cargo_lock_ignore_match(line) {
            ignored = next_ignored;
        }
    }

    Ok((
        ignored,
        ignored.then(|| gitignore_entry.path.rel_path.clone()),
    ))
}

fn cargo_lock_ignore_match(line: &str) -> Option<bool> {
    let trimmed = line.trim();
    if trimmed.is_empty() || trimmed.starts_with('#') {
        return None;
    }

    let (ignored, pattern_text) = if let Some(pattern) = trimmed.strip_prefix('!') {
        (false, pattern)
    } else {
        (true, trimmed)
    };
    let anchored = pattern_text.starts_with('/');
    let normalized = pattern_text.trim_start_matches('/');
    if normalized.is_empty() {
        return None;
    }

    let matched = if normalized == "Cargo.lock" {
        true
    } else if normalized.contains('/') {
        Pattern::new(normalized)
            .ok()
            .is_some_and(|pattern| pattern.matches("Cargo.lock"))
    } else {
        Pattern::new(normalized).ok().is_some_and(|pattern| {
            if anchored {
                pattern.matches("Cargo.lock")
            } else {
                pattern.matches("Cargo.lock")
            }
        })
    };

    matched.then_some(ignored)
}

fn discover_installed_tools(path_env: Option<&OsStr>) -> Vec<String> {
    let mut installed = BTreeSet::new();
    for tool in ["cargo-deny", "cargo-machete", "cargo-dupes", "gitleaks"] {
        if tool_is_available(tool, path_env) {
            let _ = installed.insert(tool.to_owned());
        }
    }
    installed.into_iter().collect()
}

fn tool_is_available(tool: &str, path_env: Option<&OsStr>) -> bool {
    let Some(path_env) = path_env else {
        return false;
    };

    std::env::split_paths(path_env).any(|dir| candidate_is_executable(&dir.join(tool)))
}

fn candidate_is_executable(path: &Path) -> bool {
    let Ok(metadata) = crate::fs::metadata(path) else {
        return false;
    };
    if !metadata.is_file() {
        return false;
    }

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt as _;
        metadata.permissions().mode() & 0o111 != 0
    }
    #[cfg(not(unix))]
    {
        true
    }
}
