use g3rs_hooks_rs_ingestion_types::{
    G3RsHooksRsConfigChecksInput, G3RsHooksRsFileTreeChecksInput,
    G3RsHooksRsIngestionError as IngestionError, G3RsHooksRsSourceChecksInput,
};
use g3rs_workspace_crawl::G3RsWorkspaceCrawl;

pub fn ingest_for_config_checks(
    _crawl: &G3RsWorkspaceCrawl,
) -> Result<G3RsHooksRsConfigChecksInput, IngestionError> {
    Err(IngestionError::ConfigIngestionNotImplemented)
}

pub fn ingest_for_source_checks(
    crawl: &G3RsWorkspaceCrawl,
) -> Result<Vec<G3RsHooksRsSourceChecksInput>, IngestionError> {
    for rel_path in [".githooks/pre-commit", "hooks/pre-commit"] {
        if let Some(entry) = crawl.entry(rel_path) {
            let is_workspace_project = root_is_workspace_project(crawl)?;
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
            return Ok(vec![G3RsHooksRsSourceChecksInput {
                rel_path: rel_path.to_owned(),
                content,
                is_workspace_project,
            }]);
        }
    }

    Ok(Vec::new())
}

pub fn ingest_for_file_tree_checks(
    _crawl: &G3RsWorkspaceCrawl,
) -> Result<G3RsHooksRsFileTreeChecksInput, IngestionError> {
    Err(IngestionError::FileTreeIngestionNotImplemented)
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

    let content = std::fs::read_to_string(&entry.path.abs_path).map_err(|err| IngestionError::Unreadable {
        path: entry.path.abs_path.clone(),
        reason: err.to_string(),
    })?;
    let cargo = cargo_toml_parser::parse(&content).map_err(|err| IngestionError::ParseFailed {
        path: entry.path.abs_path.clone(),
        reason: err.to_string(),
    })?;
    Ok(cargo.workspace.is_some())
}
