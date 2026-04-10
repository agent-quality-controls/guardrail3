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
