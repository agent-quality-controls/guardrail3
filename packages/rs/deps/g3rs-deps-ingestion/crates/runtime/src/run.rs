use std::collections::BTreeSet;
use std::ffi::OsStr;
use std::path::Path;

use g3_workspace_crawl::G3WorkspaceCrawl;
use g3rs_deps_types::{
    G3RsDepsConfigChecksInput, G3RsDepsConfigInputScope, G3RsDepsFileTreeChecksInput,
    G3RsDepsSourceChecksInput,
};
use g3rs_toml_parser::types::RustProfile;
use glob::Pattern;

/// Re-export of `G3RsDepsIngestionError` so the facade can reach it.
pub use g3rs_deps_ingestion_types::G3RsDepsIngestionError as IngestionError;

/// List of `G3RsDepsConfigChecksInput` records produced by `ingest_for_config_checks`.
type ConfigChecksInputs = Vec<G3RsDepsConfigChecksInput>;

/// Ingest workspace deps config from a workspace crawl into per-crate checks inputs.
///
/// # Errors
/// Returns an error when the underlying operation fails.
pub fn ingest_for_config_checks(
    crawl: &G3WorkspaceCrawl,
) -> Result<ConfigChecksInputs, IngestionError> {
    #[expect(
        clippy::disallowed_methods,
        reason = "the deps-ingestion entry point legitimately needs the host PATH to enumerate installed tools; tests use ingest_for_config_checks_with_path which takes the env explicitly"
    )]
    let path_env = std::env::var_os("PATH");
    ingest_for_config_checks_with_path(crawl, path_env.as_deref())
}

/// Implements `ingest for config checks with path`.
pub(crate) fn ingest_for_config_checks_with_path(
    crawl: &G3WorkspaceCrawl,
    path_env: Option<&OsStr>,
) -> Result<ConfigChecksInputs, IngestionError> {
    let workspace_cargo_entry = read_workspace_cargo_entry(crawl)?;
    let guardrail_entry = read_guardrail_entry(crawl)?;

    let workspace_cargo = crate::parse::parse_cargo_toml(&workspace_cargo_entry.path.abs_path)?;
    let guardrail = crate::parse::parse_guardrail3_rs_toml(&guardrail_entry.path.abs_path)?;
    validate_allowed_deps(&guardrail, &guardrail_entry.path.abs_path)?;

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

/// Resolves the workspace-root `Cargo.toml` entry, returning a readability error if necessary.
fn read_workspace_cargo_entry(
    crawl: &G3WorkspaceCrawl,
) -> Result<&g3_workspace_crawl::G3WorkspaceEntry, IngestionError> {
    let workspace_cargo_entry = crate::select::select_workspace_cargo_toml(crawl)
        .ok_or(IngestionError::CargoTomlNotFound)?;
    if !workspace_cargo_entry.readable {
        return Err(IngestionError::Unreadable {
            path: workspace_cargo_entry.path.abs_path.clone(),
            reason: "file is not readable".to_owned(),
        });
    }
    Ok(workspace_cargo_entry)
}

/// Resolves the workspace-root `guardrail3-rs.toml` entry, returning a readability error if necessary.
fn read_guardrail_entry(
    crawl: &G3WorkspaceCrawl,
) -> Result<&g3_workspace_crawl::G3WorkspaceEntry, IngestionError> {
    let guardrail_entry = crate::select::select_workspace_guardrail3_rs_toml(crawl)
        .ok_or(IngestionError::Guardrail3RsTomlNotFound)?;
    if !guardrail_entry.readable {
        return Err(IngestionError::Unreadable {
            path: guardrail_entry.path.abs_path.clone(),
            reason: "file is not readable".to_owned(),
        });
    }
    Ok(guardrail_entry)
}

/// Rejects allowlists that contain empty dependency names.
fn validate_allowed_deps(
    guardrail: &crate::parse::ParsedGuardrail3RsToml,
    abs_path: &Path,
) -> Result<(), IngestionError> {
    if guardrail
        .config
        .allowed_deps
        .iter()
        .any(|dependency| dependency.trim().is_empty())
    {
        return Err(IngestionError::NormalizationFailed {
            path: abs_path.to_path_buf(),
            reason: "allowed_deps must not contain empty dependency names".to_owned(),
        });
    }
    Ok(())
}

/// Stub source ingestion entry point for the deps family.
///
/// # Errors
/// Returns an error when the underlying operation fails.
pub const fn ingest_for_source_checks(
    _crawl: &G3WorkspaceCrawl,
) -> Result<G3RsDepsSourceChecksInput, IngestionError> {
    Err(IngestionError::SourceIngestionNotImplemented)
}

/// Implements this item.
///
/// # Errors
/// Returns an error when the underlying operation fails.
pub fn ingest_for_file_tree_checks(
    crawl: &G3WorkspaceCrawl,
) -> Result<G3RsDepsFileTreeChecksInput, IngestionError> {
    let cargo_lock_rel_path = "Cargo.lock".to_owned();
    let cargo_lock_exists =
        g3_workspace_crawl::root_file(crawl, cargo_lock_rel_path.as_str()).is_some();
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

/// Pair of (`Cargo.lock` ignored?, gitignore file path) returned by
/// `read_lockfile_ignore_state`.
type LockfileIgnoreState = (bool, Option<String>);

/// Optional profile read from `guardrail3-rs.toml`. `None` means no policy file
/// is present; `Some(None)` means the policy file exists but does not declare a profile.
type ReadRootProfile = Result<Option<RustProfile>, IngestionError>;

/// Implements `read root profile`.
///
/// # Errors
/// Returns an error when the underlying operation fails.
fn read_root_profile(crawl: &G3WorkspaceCrawl) -> ReadRootProfile {
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

/// Implements `read lockfile ignore state`.
fn read_lockfile_ignore_state(
    crawl: &G3WorkspaceCrawl,
) -> Result<LockfileIgnoreState, IngestionError> {
    let Some(gitignore_entry) = g3_workspace_crawl::root_file(crawl, ".gitignore") else {
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

/// Implements `cargo lock ignore match`.
fn cargo_lock_ignore_match(line: &str) -> Option<bool> {
    let trimmed = line.trim();
    if trimmed.is_empty() || trimmed.starts_with('#') {
        return None;
    }

    let (ignored, pattern_text) = trimmed
        .strip_prefix('!')
        .map_or((true, trimmed), |pattern| (false, pattern));
    let normalized = pattern_text.trim_start_matches('/');
    if normalized.is_empty() {
        return None;
    }

    // For unanchored, slash-free patterns, gitignore semantics also match a top-level
    // `Cargo.lock`, so all three branches collapse to the same `pattern.matches("Cargo.lock")`
    // call once the pattern parses.
    let matched = normalized == "Cargo.lock"
        || Pattern::new(normalized)
            .ok()
            .is_some_and(|pattern| pattern.matches("Cargo.lock"));

    matched.then_some(ignored)
}

/// Implements `discover installed tools`.
fn discover_installed_tools(path_env: Option<&OsStr>) -> Vec<String> {
    let mut installed = BTreeSet::new();
    for tool in ["cargo-deny", "cargo-machete", "cargo-dupes", "gitleaks"] {
        if tool_is_available(tool, path_env) {
            let _ = installed.insert(tool.to_owned());
        }
    }
    installed.into_iter().collect()
}

/// Implements `tool is available`.
fn tool_is_available(tool: &str, path_env: Option<&OsStr>) -> bool {
    let Some(path_env) = path_env else {
        return false;
    };

    std::env::split_paths(path_env).any(|dir| candidate_is_executable(&dir.join(tool)))
}

/// Implements `candidate is executable`.
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
