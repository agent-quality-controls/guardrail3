use g3rs_hooks_shared_ingestion_types::{
    G3RsHookScriptKind, G3RsHooksSharedConfigChecksInput, G3RsHooksSharedFileTreeChecksInput,
    G3RsHooksSharedIngestionError as IngestionError, G3RsHooksSharedSourceChecksInput,
};
use g3rs_workspace_crawl::{G3RsWorkspaceCrawl, G3RsWorkspaceEntryKind};

pub fn ingest_for_config_checks(
    _crawl: &G3RsWorkspaceCrawl,
) -> Result<G3RsHooksSharedConfigChecksInput, IngestionError> {
    Err(IngestionError::ConfigIngestionNotImplemented)
}

pub fn ingest_for_source_checks(
    crawl: &G3RsWorkspaceCrawl,
) -> Result<Vec<G3RsHooksSharedSourceChecksInput>, IngestionError> {
    let has_modular_dir = crawl.entries.iter().any(|entry| {
        entry.kind == G3RsWorkspaceEntryKind::Directory && entry.path.rel_path == ".githooks/pre-commit.d"
    });
    let mut inputs = Vec::new();

    for rel_path in [".githooks/pre-commit", "hooks/pre-commit"] {
        if let Some(entry) = crawl.entry(rel_path) {
            if !entry.readable {
                return Err(IngestionError::Unreadable {
                    path: entry.path.abs_path.clone(),
                    reason: "file is not readable".to_owned(),
                });
            }
            let content = std::fs::read_to_string(&entry.path.abs_path).map_err(|err| IngestionError::Unreadable {
                path: entry.path.abs_path.clone(),
                reason: err.to_string(),
            })?;
            inputs.push(G3RsHooksSharedSourceChecksInput {
                rel_path: rel_path.to_owned(),
                kind: G3RsHookScriptKind::PreCommit,
                content,
                has_modular_dir,
            });
            break;
        }
    }

    for entry in &crawl.entries {
        if entry.kind != G3RsWorkspaceEntryKind::File
            || !entry.path.rel_path.starts_with(".githooks/pre-commit.d/")
        {
            continue;
        }
        if !entry.readable {
            return Err(IngestionError::Unreadable {
                path: entry.path.abs_path.clone(),
                reason: "file is not readable".to_owned(),
            });
        }
        let content = std::fs::read_to_string(&entry.path.abs_path).map_err(|err| IngestionError::Unreadable {
            path: entry.path.abs_path.clone(),
            reason: err.to_string(),
        })?;
        inputs.push(G3RsHooksSharedSourceChecksInput {
            rel_path: entry.path.rel_path.clone(),
            kind: G3RsHookScriptKind::Modular,
            content,
            has_modular_dir,
        });
    }

    Ok(inputs)
}

pub fn ingest_for_file_tree_checks(
    _crawl: &G3RsWorkspaceCrawl,
) -> Result<G3RsHooksSharedFileTreeChecksInput, IngestionError> {
    Err(IngestionError::FileTreeIngestionNotImplemented)
}
