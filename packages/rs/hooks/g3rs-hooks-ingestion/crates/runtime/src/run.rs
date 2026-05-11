#![expect(
    clippy::missing_errors_doc,
    reason = "structural code pattern (parser/assertion helper) where lint conflicts with module architecture"
)]
#![expect(
    clippy::type_complexity,
    reason = "structural code pattern (parser/assertion helper) where lint conflicts with module architecture"
)]
#![expect(
    clippy::unnecessary_wraps,
    reason = "structural code pattern (parser/assertion helper) where lint conflicts with module architecture"
)]
use std::borrow::Cow;
use std::collections::BTreeSet;
use std::ffi::OsStr;
use std::path::Path;

use g3_workspace_crawl::{G3WorkspaceCrawl, G3WorkspaceEntry, G3WorkspaceEntryKind};
use g3rs_hooks_config_checks_types::{G3RsHooksConfigChecksInput, G3RsHooksSelectedHookConfigFact};
use g3rs_hooks_file_tree_checks_types::{G3RsHooksFileTreeChecksInput, G3RsHooksScriptFileFact};
use g3rs_hooks_ingestion_types::G3RsHooksIngestionError as IngestionError;
use g3rs_hooks_types::{G3RsHookScriptKind, G3RsHooksSourceChecksInput};
use hook_shell_parser::parse_script;

use crate::upward;

/// `SelectedHookSurface` struct.
struct SelectedHookSurface<'a> {
    /// `entry` item.
    entry: Cow<'a, G3WorkspaceEntry>,
}

/// Ingest hooks config-checks input by reading the host PATH from the environment.
///
/// # Errors
///
/// Propagates ingestion failures from selection and parsing.
#[expect(
    clippy::disallowed_methods,
    reason = "this is the single sanctioned site that reads the host PATH for hook-existence resolution; downstream callers go through this entry point."
)]
pub fn ingest_for_config_checks(
    crawl: &G3WorkspaceCrawl,
) -> Result<G3RsHooksConfigChecksInput, IngestionError> {
    ingest_for_config_checks_with_path(crawl, std::env::var_os("PATH").as_deref())
}

pub fn ingest_for_source_checks(
    crawl: &G3WorkspaceCrawl,
) -> Result<Vec<G3RsHooksSourceChecksInput>, IngestionError> {
    let mut inputs = Vec::new();
    if !hooks_scope_is_active(crawl.root_abs_path.as_path())? {
        return Ok(inputs);
    }

    let is_workspace_project = root_is_workspace_project(crawl)?;

    if let Some(entry) = upward::find_file_entry(crawl, ".githooks/pre-commit") {
        if !entry.readable {
            return Err(IngestionError::Unreadable {
                path: entry.path.abs_path.clone(),
                reason: "file is not readable".to_owned(),
            });
        }
        let content = read_entry(entry.path.abs_path.as_path())?;
        inputs.push(G3RsHooksSourceChecksInput {
            rel_path: entry.path.rel_path.clone(),
            kind: G3RsHookScriptKind::PreCommit,
            exists: true,
            parsed: parse_script(&content),
            has_modular_dir: false,
            is_workspace_project,
            requirements: Vec::new(),
        });
    }

    match upward::find_file_entry(crawl, "scripts/g3rs/verify") {
        Some(entry) => {
            if !entry.readable {
                return Err(IngestionError::Unreadable {
                    path: entry.path.abs_path.clone(),
                    reason: "file is not readable".to_owned(),
                });
            }
            let content = read_entry(entry.path.abs_path.as_path())?;
            inputs.push(G3RsHooksSourceChecksInput {
                rel_path: entry.path.rel_path.clone(),
                kind: G3RsHookScriptKind::G3RsVerifier,
                exists: true,
                parsed: parse_script(&content),
                has_modular_dir: false,
                is_workspace_project,
                requirements: Vec::new(),
            });
        }
        None => {
            inputs.push(G3RsHooksSourceChecksInput {
                rel_path: "scripts/g3rs/verify".to_owned(),
                kind: G3RsHookScriptKind::G3RsVerifier,
                exists: false,
                parsed: parse_script(""),
                has_modular_dir: false,
                is_workspace_project,
                requirements: Vec::new(),
            });
        }
    }

    Ok(inputs)
}

pub fn ingest_for_file_tree_checks(
    crawl: &G3WorkspaceCrawl,
) -> Result<G3RsHooksFileTreeChecksInput, IngestionError> {
    let active = hooks_scope_is_active(crawl.root_abs_path.as_path())?;
    if !active {
        return Ok(G3RsHooksFileTreeChecksInput {
            active,
            pre_commit: None,
            has_modular_dir: false,
            modular_scripts: Vec::new(),
            local_override_scripts: Vec::new(),
            hooks_path: None,
            trust_risks: Vec::new(),
        });
    }

    let hooks_path = read_hooks_path(crawl.root_abs_path.as_path())?;
    let selected = select_pre_commit_surface(crawl, hooks_path.as_deref());
    let pre_commit = selected
        .as_ref()
        .map(|selected| read_script_file_fact(selected.entry.as_ref()))
        .transpose()?;
    let modular_dir_entry = upward::find_dir_entry(crawl, ".githooks/pre-commit.d");
    let has_modular_dir = modular_dir_entry.is_some();

    Ok(G3RsHooksFileTreeChecksInput {
        active,
        pre_commit,
        has_modular_dir,
        modular_scripts: match modular_dir_entry.as_deref() {
            Some(dir) => collect_direct_script_facts(crawl, dir, ".githooks/pre-commit.d/")?,
            None => Vec::new(),
        },
        local_override_scripts: collect_direct_file_names(
            crawl,
            ".guardrail3/overrides/pre-commit.d/",
        ),
        hooks_path: hooks_path.clone(),
        trust_risks: collect_trust_risks(crawl, hooks_path.as_deref()),
    })
}

/// `ingest_for_config_checks_with_path` function.
pub(crate) fn ingest_for_config_checks_with_path(
    crawl: &G3WorkspaceCrawl,
    path_env: Option<&OsStr>,
) -> Result<G3RsHooksConfigChecksInput, IngestionError> {
    let active = hooks_scope_is_active(crawl.root_abs_path.as_path())?;
    if !active {
        return Ok(G3RsHooksConfigChecksInput {
            active,
            selected_hook: None,
            installed_tools: Vec::new(),
            requirements: Vec::new(),
        });
    }

    let hooks_path = read_hooks_path(crawl.root_abs_path.as_path())?;
    Ok(G3RsHooksConfigChecksInput {
        active,
        selected_hook: select_pre_commit_surface(crawl, hooks_path.as_deref())
            .map(|selected| read_selected_hook_config_fact(selected.entry.as_ref()))
            .transpose()?,
        installed_tools: discover_installed_tools(path_env),
        requirements: Vec::new(),
    })
}

/// `select_pre_commit_surface` function.
fn select_pre_commit_surface<'a>(
    crawl: &'a G3WorkspaceCrawl,
    hooks_path: Option<&str>,
) -> Option<SelectedHookSurface<'a>> {
    let entry = match normalized_hooks_path(hooks_path) {
        Some(".githooks") => upward::find_file_entry(crawl, ".githooks/pre-commit"),
        Some("hooks") => upward::find_file_entry(crawl, "hooks/pre-commit"),
        Some(_) => None,
        None => upward::find_file_entry(crawl, ".githooks/pre-commit")
            .or_else(|| upward::find_file_entry(crawl, "hooks/pre-commit")),
    }?;

    Some(SelectedHookSurface { entry })
}

/// `normalized_hooks_path` function.
fn normalized_hooks_path(hooks_path: Option<&str>) -> Option<&str> {
    let hooks_path = hooks_path?;
    let hooks_path = hooks_path.trim_end_matches('/');
    Some(hooks_path.strip_prefix("./").unwrap_or(hooks_path))
}

/// `collect_direct_script_facts` function.
fn collect_direct_script_facts(
    crawl: &G3WorkspaceCrawl,
    dir_entry: &G3WorkspaceEntry,
    rel_path_prefix: &str,
) -> Result<Vec<G3RsHooksScriptFileFact>, IngestionError> {
    let from_workspace = g3_workspace_crawl::entry(crawl, dir_entry.path.rel_path.as_str())
        .is_some_and(|entry| {
            entry.kind == G3WorkspaceEntryKind::Directory
                && entry.path.abs_path == dir_entry.path.abs_path
        });

    let mut scripts = if from_workspace {
        crawl
            .entries
            .iter()
            .filter(|entry| entry.kind == G3WorkspaceEntryKind::File)
            .filter(|entry| {
                let Some(suffix) = entry.path.rel_path.strip_prefix(rel_path_prefix) else {
                    return false;
                };
                !suffix.is_empty() && !suffix.contains('/')
            })
            .map(read_script_file_fact)
            .collect::<Result<Vec<_>, _>>()?
    } else {
        let mut facts = Vec::new();
        for abs_path in upward::read_direct_files(dir_entry.path.abs_path.as_path()) {
            let file_name = abs_path
                .file_name()
                .map(|name| name.to_string_lossy().into_owned())
                .unwrap_or_default();
            if file_name.is_empty() {
                continue;
            }
            let rel_path = format!("{rel_path_prefix}{file_name}");
            let synth = G3WorkspaceEntry {
                path: g3_workspace_crawl::G3WorkspacePath { rel_path, abs_path },
                kind: G3WorkspaceEntryKind::File,
                ignore_state: g3_workspace_crawl::G3WorkspaceIgnoreState::Included,
                readable: true,
            };
            facts.push(read_script_file_fact(&synth)?);
        }
        facts
    };
    scripts.sort_by(|left, right| left.rel_path.cmp(&right.rel_path));
    Ok(scripts)
}

/// `collect_direct_file_names` function.
fn collect_direct_file_names(crawl: &G3WorkspaceCrawl, prefix: &str) -> Vec<String> {
    let mut paths = crawl
        .entries
        .iter()
        .filter(|entry| entry.kind == G3WorkspaceEntryKind::File)
        .filter_map(|entry| {
            let suffix = entry.path.rel_path.strip_prefix(prefix)?;
            (!suffix.is_empty() && !suffix.contains('/')).then(|| suffix.to_owned())
        })
        .collect::<Vec<_>>();
    paths.sort();
    paths
}

/// `root_is_workspace_project` function.
fn root_is_workspace_project(crawl: &G3WorkspaceCrawl) -> Result<bool, IngestionError> {
    let Some(entry) = g3_workspace_crawl::entry(crawl, "Cargo.toml") else {
        return Ok(false);
    };
    if !entry.readable {
        return Err(IngestionError::Unreadable {
            path: entry.path.abs_path.clone(),
            reason: "file is not readable".to_owned(),
        });
    }

    let content = read_entry(entry.path.abs_path.as_path())?;
    let cargo = cargo_toml_parser::parse(&content).map_err(|err| IngestionError::ParseFailed {
        path: entry.path.abs_path.clone(),
        reason: err.to_string(),
    })?;
    Ok(cargo.workspace.is_some())
}

/// `read_entry` function.
fn read_entry(path: &std::path::Path) -> Result<String, IngestionError> {
    crate::fs::read_to_string(path).map_err(|err| IngestionError::Unreadable {
        path: path.to_path_buf(),
        reason: err.to_string(),
    })
}

/// `read_selected_hook_config_fact` function.
fn read_selected_hook_config_fact(
    entry: &g3_workspace_crawl::G3WorkspaceEntry,
) -> Result<G3RsHooksSelectedHookConfigFact, IngestionError> {
    if !entry.readable {
        return Err(IngestionError::Unreadable {
            path: entry.path.abs_path.clone(),
            reason: "file is not readable".to_owned(),
        });
    }

    Ok(G3RsHooksSelectedHookConfigFact {
        rel_path: entry.path.rel_path.clone(),
        parsed: parse_script(&read_entry(entry.path.abs_path.as_path())?),
    })
}

/// `read_script_file_fact` function.
fn read_script_file_fact(
    entry: &g3_workspace_crawl::G3WorkspaceEntry,
) -> Result<G3RsHooksScriptFileFact, IngestionError> {
    if !entry.readable {
        return Err(IngestionError::Unreadable {
            path: entry.path.abs_path.clone(),
            reason: "file is not readable".to_owned(),
        });
    }

    let content = read_entry(entry.path.abs_path.as_path())?;
    Ok(G3RsHooksScriptFileFact {
        rel_path: entry.path.rel_path.clone(),
        line_count: content.lines().count(),
        byte_count: content.len(),
        executable: executable_bit(entry.path.abs_path.as_path()),
    })
}

/// `discover_installed_tools` function.
fn discover_installed_tools(path_env: Option<&OsStr>) -> Vec<String> {
    let mut installed = BTreeSet::new();
    for tool in [
        "gitleaks",
        "cargo-deny",
        "cargo-machete",
        "g3rs",
        "cargo-dupes",
    ] {
        if tool_is_available(tool, path_env) {
            let _ = installed.insert(tool.to_owned());
        }
    }
    installed.into_iter().collect()
}

/// `tool_is_available` function.
fn tool_is_available(tool: &str, path_env: Option<&OsStr>) -> bool {
    let Some(path_env) = path_env else {
        return false;
    };

    std::env::split_paths(path_env).any(|dir| candidate_is_executable(&dir.join(tool)))
}

/// `candidate_is_executable` function.
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

/// `executable_bit` function.
fn executable_bit(path: &Path) -> Option<bool> {
    let metadata = crate::fs::metadata(path).ok()?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt as _;
        Some(metadata.permissions().mode() & 0o111 != 0)
    }
    #[cfg(not(unix))]
    {
        let _ = metadata;
        None
    }
}

/// `read_hooks_path` function.
fn read_hooks_path(root: &Path) -> Result<Option<String>, IngestionError> {
    if !crate::fs::path_exists(root.join(".git").as_path()) && upward::find_git_root(root).is_none()
    {
        return Ok(None);
    }

    let config_path = git_path(root, "config").unwrap_or_else(|| root.join(".git/config"));
    #[allow(clippy::disallowed_methods)]
    let output = std::process::Command::new("git")
        .args(["config", "core.hooksPath"])
        .current_dir(root)
        .output()
        .map_err(|err| IngestionError::Unreadable {
            path: config_path.clone(),
            reason: err.to_string(),
        })?;
    if !output.status.success() {
        if output.status.code() == Some(1) {
            return Ok(None);
        }
        let reason = String::from_utf8_lossy(&output.stderr)
            .trim_end_matches(['\n', '\r'])
            .to_owned();
        return Err(IngestionError::ParseFailed {
            path: config_path,
            reason: if reason.is_empty() {
                format!("git config core.hooksPath exited with {}", output.status)
            } else {
                reason
            },
        });
    }

    Ok(Some(
        String::from_utf8_lossy(&output.stdout)
            .trim_end_matches(['\n', '\r'])
            .to_owned(),
    ))
}

/// `hooks_scope_is_active` function.
fn hooks_scope_is_active(root: &Path) -> Result<bool, IngestionError> {
    if crate::fs::path_exists(root.join(".git").as_path()) {
        return Ok(true);
    }
    if upward::find_git_root(root).is_some() {
        return Ok(true);
    }

    Ok(false)
}

/// `collect_trust_risks` function.
fn collect_trust_risks(crawl: &G3WorkspaceCrawl, hooks_path: Option<&str>) -> Vec<String> {
    let mut risks = Vec::new();

    if g3_workspace_crawl::entry(crawl, ".husky/pre-commit").is_some() {
        risks.push(".husky/pre-commit".to_owned());
    }
    for rel_path in [
        "lefthook.yml",
        "lefthook.yaml",
        ".lefthook.yml",
        ".lefthook.yaml",
    ] {
        if g3_workspace_crawl::entry(crawl, rel_path).is_some() {
            risks.push(rel_path.to_owned());
        }
    }

    if git_hook_pre_commit_exists(crawl.root_abs_path.as_path())
        && !matches!(
            normalized_hooks_path(hooks_path),
            Some(".githooks" | "hooks")
        )
    {
        risks.push(".git/hooks/pre-commit".to_owned());
    }

    risks.sort();
    risks
}

/// `git_path` function.
fn git_path(root: &Path, rel: &str) -> Option<std::path::PathBuf> {
    #[allow(clippy::disallowed_methods)]
    let output = std::process::Command::new("git")
        .args(["rev-parse", "--git-path", rel])
        .current_dir(root)
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }

    let raw = String::from_utf8_lossy(&output.stdout)
        .trim_end_matches(['\n', '\r'])
        .to_owned();
    if raw.is_empty() {
        return None;
    }

    let path = Path::new(&raw);
    Some(if path.is_absolute() {
        path.to_path_buf()
    } else {
        root.join(path)
    })
}

/// `git_hook_pre_commit_exists` function.
fn git_hook_pre_commit_exists(root: &Path) -> bool {
    let Some(abs) = git_path(root, "hooks/pre-commit") else {
        return false;
    };
    crate::fs::path_exists(abs.as_path())
}

#[cfg(test)]
#[path = "run_tests/mod.rs"] // reason: owned sidecar tests for file module.
mod run_tests;
