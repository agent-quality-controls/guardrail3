use std::collections::BTreeSet;
use std::ffi::OsStr;
use std::path::Path;

use g3rs_hooks_config_checks_types::{
    G3RsHooksConfigChecksInput, G3RsHooksSelectedHookConfigFact,
};
use g3rs_hooks_file_tree_checks_types::{G3RsHooksFileTreeChecksInput, G3RsHooksScriptFileFact};
use g3rs_hooks_ingestion_types::{
    G3RsHookScriptKind, G3RsHooksIngestionError as IngestionError, G3RsHooksSourceChecksInput,
};
use g3rs_workspace_crawl::{G3RsWorkspaceCrawl, G3RsWorkspaceEntryKind};

struct SelectedHookSurface<'a> {
    entry: &'a g3rs_workspace_crawl::G3RsWorkspaceEntry,
    has_modular_dir: bool,
}

pub fn ingest_for_config_checks(
    crawl: &G3RsWorkspaceCrawl,
) -> Result<G3RsHooksConfigChecksInput, IngestionError> {
    ingest_for_config_checks_with_path(crawl, std::env::var_os("PATH").as_deref())
}

pub fn ingest_for_source_checks(
    crawl: &G3RsWorkspaceCrawl,
) -> Result<Vec<G3RsHooksSourceChecksInput>, IngestionError> {
    let mut inputs = Vec::new();
    let hooks_path = read_hooks_path(crawl.root_abs_path.as_path())?;
    let Some(selected) = select_pre_commit_surface(crawl, hooks_path.as_deref()) else {
        return Ok(inputs);
    };
    let pre_commit_entry = selected.entry;

    if !pre_commit_entry.readable {
        return Err(IngestionError::Unreadable {
            path: pre_commit_entry.path.abs_path.clone(),
            reason: "file is not readable".to_owned(),
        });
    }

    let selected_rel_path = pre_commit_entry.path.rel_path.as_str();
    let content = read_entry(pre_commit_entry.path.abs_path.as_path())?;
    let is_workspace_project = root_is_workspace_project(crawl)?;
    inputs.push(G3RsHooksSourceChecksInput {
        rel_path: selected_rel_path.to_owned(),
        kind: G3RsHookScriptKind::PreCommit,
        content,
        has_modular_dir: selected.has_modular_dir,
        is_workspace_project,
    });

    if !selected.has_modular_dir {
        return Ok(inputs);
    }

    for entry in &crawl.entries {
        if entry.kind != G3RsWorkspaceEntryKind::File || !is_direct_modular_script(&entry.path.rel_path) {
            continue;
        }
        if !entry.readable {
            return Err(IngestionError::Unreadable {
                path: entry.path.abs_path.clone(),
                reason: "file is not readable".to_owned(),
            });
        }
        let content = read_entry(entry.path.abs_path.as_path())?;
        inputs.push(G3RsHooksSourceChecksInput {
            rel_path: entry.path.rel_path.clone(),
            kind: G3RsHookScriptKind::Modular,
            content,
            has_modular_dir: selected.has_modular_dir,
            is_workspace_project,
        });
    }

    Ok(inputs)
}

pub fn ingest_for_file_tree_checks(
    crawl: &G3RsWorkspaceCrawl,
) -> Result<G3RsHooksFileTreeChecksInput, IngestionError> {
    let hooks_path = read_hooks_path(crawl.root_abs_path.as_path())?;
    let selected = select_pre_commit_surface(crawl, hooks_path.as_deref());
    let pre_commit = selected.as_ref().map(|selected| selected.entry).map(read_script_file_fact).transpose()?;
    let has_modular_dir = crawl
        .entry(".githooks/pre-commit.d")
        .is_some_and(|entry| entry.kind == G3RsWorkspaceEntryKind::Directory);

    Ok(G3RsHooksFileTreeChecksInput {
        pre_commit,
        has_modular_dir,
        modular_scripts: if has_modular_dir {
            collect_direct_script_facts(crawl, ".githooks/pre-commit.d/")?
        } else {
            Vec::new()
        },
        local_override_scripts: collect_direct_file_names(crawl, ".guardrail3/overrides/pre-commit.d/"),
        hooks_path: hooks_path.clone(),
        trust_risks: collect_trust_risks(crawl, hooks_path.as_deref()),
    })
}

pub(crate) fn ingest_for_config_checks_with_path(
    crawl: &G3RsWorkspaceCrawl,
    path_env: Option<&OsStr>,
) -> Result<G3RsHooksConfigChecksInput, IngestionError> {
    let hooks_path = read_hooks_path(crawl.root_abs_path.as_path())?;
    Ok(G3RsHooksConfigChecksInput {
        selected_hook: select_pre_commit_surface(crawl, hooks_path.as_deref())
            .map(|selected| selected.entry)
            .map(read_selected_hook_config_fact)
            .transpose()?,
        installed_tools: discover_installed_tools(path_env),
    })
}

fn is_direct_modular_script(rel_path: &str) -> bool {
    let Some(suffix) = rel_path.strip_prefix(".githooks/pre-commit.d/") else {
        return false;
    };
    !suffix.is_empty() && !suffix.contains('/')
}

fn select_pre_commit_surface<'a>(
    crawl: &'a G3RsWorkspaceCrawl,
    hooks_path: Option<&str>,
) -> Option<SelectedHookSurface<'a>> {
    let entry = match normalized_hooks_path(hooks_path) {
        Some(".githooks") => crawl.entry(".githooks/pre-commit"),
        Some("hooks") => crawl.entry("hooks/pre-commit"),
        Some(_) => None,
        None => crawl
            .entry(".githooks/pre-commit")
            .or_else(|| crawl.entry("hooks/pre-commit")),
    }?;

    Some(SelectedHookSurface {
        has_modular_dir: entry.path.rel_path == ".githooks/pre-commit"
            && crawl
                .entry(".githooks/pre-commit.d")
                .is_some_and(|entry| entry.kind == G3RsWorkspaceEntryKind::Directory),
        entry,
    })
}

fn normalized_hooks_path(hooks_path: Option<&str>) -> Option<&str> {
    let hooks_path = hooks_path?;
    let hooks_path = hooks_path.trim_end_matches('/');
    Some(hooks_path.strip_prefix("./").unwrap_or(hooks_path))
}

fn collect_direct_script_facts(
    crawl: &G3RsWorkspaceCrawl,
    prefix: &str,
) -> Result<Vec<G3RsHooksScriptFileFact>, IngestionError> {
    let mut scripts = crawl
        .entries
        .iter()
        .filter(|entry| entry.kind == G3RsWorkspaceEntryKind::File)
        .filter(|entry| {
            let Some(suffix) = entry.path.rel_path.strip_prefix(prefix) else {
                return false;
            };
            !suffix.is_empty() && !suffix.contains('/')
        })
        .map(read_script_file_fact)
        .collect::<Result<Vec<_>, _>>()?;
    scripts.sort_by(|left, right| left.rel_path.cmp(&right.rel_path));
    Ok(scripts)
}

fn collect_direct_file_names(crawl: &G3RsWorkspaceCrawl, prefix: &str) -> Vec<String> {
    let mut paths = crawl
        .entries
        .iter()
        .filter(|entry| entry.kind == G3RsWorkspaceEntryKind::File)
        .filter_map(|entry| {
            let suffix = entry.path.rel_path.strip_prefix(prefix)?;
            (!suffix.is_empty() && !suffix.contains('/')).then(|| suffix.to_owned())
        })
        .collect::<Vec<_>>();
    paths.sort();
    paths
}

fn root_is_workspace_project(crawl: &G3RsWorkspaceCrawl) -> Result<bool, IngestionError> {
    let Some(entry) = crawl.entry("Cargo.toml") else {
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

fn read_entry(path: &std::path::Path) -> Result<String, IngestionError> {
    std::fs::read_to_string(path).map_err(|err| IngestionError::Unreadable {
        path: path.to_path_buf(),
        reason: err.to_string(),
    })
}

fn read_selected_hook_config_fact(
    entry: &g3rs_workspace_crawl::G3RsWorkspaceEntry,
) -> Result<G3RsHooksSelectedHookConfigFact, IngestionError> {
    if !entry.readable {
        return Err(IngestionError::Unreadable {
            path: entry.path.abs_path.clone(),
            reason: "file is not readable".to_owned(),
        });
    }

    Ok(G3RsHooksSelectedHookConfigFact {
        rel_path: entry.path.rel_path.clone(),
        content: read_entry(entry.path.abs_path.as_path())?,
    })
}

fn read_script_file_fact(
    entry: &g3rs_workspace_crawl::G3RsWorkspaceEntry,
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

fn discover_installed_tools(path_env: Option<&OsStr>) -> Vec<String> {
    let mut installed = BTreeSet::new();
    for tool in ["gitleaks", "cargo-deny", "cargo-machete", "g3rs", "cargo-dupes"] {
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
    let Ok(metadata) = std::fs::metadata(path) else {
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

fn executable_bit(path: &Path) -> Option<bool> {
    let metadata = std::fs::metadata(path).ok()?;

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

fn read_hooks_path(root: &Path) -> Result<Option<String>, IngestionError> {
    if std::fs::metadata(root.join(".git")).is_err() {
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

fn collect_trust_risks(crawl: &G3RsWorkspaceCrawl, hooks_path: Option<&str>) -> Vec<String> {
    let mut risks = Vec::new();

    if crawl.entry(".husky/pre-commit").is_some() {
        risks.push(".husky/pre-commit".to_owned());
    }
    for rel_path in [
        "lefthook.yml",
        "lefthook.yaml",
        ".lefthook.yml",
        ".lefthook.yaml",
    ] {
        if crawl.entry(rel_path).is_some() {
            risks.push(rel_path.to_owned());
        }
    }

    if git_hook_pre_commit_exists(crawl.root_abs_path.as_path())
        && !matches!(normalized_hooks_path(hooks_path), Some(".githooks" | "hooks"))
    {
        risks.push(".git/hooks/pre-commit".to_owned());
    }

    risks.sort();
    risks
}

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

fn git_hook_pre_commit_exists(root: &Path) -> bool {
    let Some(abs) = git_path(root, "hooks/pre-commit") else {
        return false;
    };
    std::fs::metadata(abs).is_ok()
}
