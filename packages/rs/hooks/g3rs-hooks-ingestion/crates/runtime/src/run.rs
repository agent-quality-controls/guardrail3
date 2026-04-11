use g3rs_hooks_ingestion_types::{
    G3RsHookScriptKind, G3RsHooksConfigChecksInput, G3RsHooksFileTreeChecksInput,
    G3RsHooksIngestionError as IngestionError, G3RsHooksSourceChecksInput,
};
use g3rs_workspace_crawl::{G3RsWorkspaceCrawl, G3RsWorkspaceEntryKind};

pub fn ingest_for_config_checks(
    _crawl: &G3RsWorkspaceCrawl,
) -> Result<G3RsHooksConfigChecksInput, IngestionError> {
    Err(IngestionError::ConfigIngestionNotImplemented)
}

pub fn ingest_for_source_checks(
    crawl: &G3RsWorkspaceCrawl,
) -> Result<Vec<G3RsHooksSourceChecksInput>, IngestionError> {
    let has_modular_dir = crawl.entries.iter().any(|entry| {
        entry.kind == G3RsWorkspaceEntryKind::Directory && entry.path.rel_path == ".githooks/pre-commit.d"
    });
    let mut inputs = Vec::new();
    let mut is_workspace_project = false;

    for rel_path in [".githooks/pre-commit", "hooks/pre-commit"] {
        if let Some(entry) = crawl.entry(rel_path) {
            if !entry.readable {
                return Err(IngestionError::Unreadable {
                    path: entry.path.abs_path.clone(),
                    reason: "file is not readable".to_owned(),
                });
            }
            let content = read_entry(entry.path.abs_path.as_path())?;
            is_workspace_project = root_is_workspace_project(crawl)?;
            inputs.push(G3RsHooksSourceChecksInput {
                rel_path: rel_path.to_owned(),
                kind: G3RsHookScriptKind::PreCommit,
                content,
                has_modular_dir,
                is_workspace_project,
            });
            break;
        }
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
            has_modular_dir,
            is_workspace_project,
        });
    }

    Ok(inputs)
}

pub fn ingest_for_file_tree_checks(
    _crawl: &G3RsWorkspaceCrawl,
) -> Result<G3RsHooksFileTreeChecksInput, IngestionError> {
    Err(IngestionError::FileTreeIngestionNotImplemented)
}

fn is_direct_modular_script(rel_path: &str) -> bool {
    let Some(suffix) = rel_path.strip_prefix(".githooks/pre-commit.d/") else {
        return false;
    };
    !suffix.is_empty() && !suffix.contains('/')
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
