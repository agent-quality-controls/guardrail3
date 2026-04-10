/// Public ingestion entry point.
use g3rs_clippy_types::{
    G3RsClippySourceChecksInput, G3RsClippyConfigChecksInput, G3RsClippyFileTreeChecksInput,
};
use g3rs_workspace_crawl::G3RsWorkspaceCrawl;

/// Re-export of `G3RsClippyIngestionError` so the facade can reach it.
pub use g3rs_clippy_ingestion_types::G3RsClippyIngestionError as IngestionError;

/// Ingest the root clippy config from a workspace crawl into a config checks input.
///
/// Prefers `clippy.toml` over `.clippy.toml` when both exist.
///
/// # Errors
///
/// Returns an error if the clippy config is missing, unreadable, or unparseable.
pub fn ingest_for_config_checks(
    crawl: &G3RsWorkspaceCrawl,
) -> Result<G3RsClippyConfigChecksInput, IngestionError> {
    let entry = crate::select::select_clippy_toml(crawl)
        .ok_or(IngestionError::ClippyTomlNotFound)?;

    if !entry.readable {
        return Err(IngestionError::Unreadable {
            path: entry.path.abs_path.clone(),
            reason: "file is not readable".to_owned(),
        });
    }

    let clippy = crate::parse::parse_clippy_toml(&entry.path.abs_path)?;
    let clippy_rel_path = entry.path.rel_path.clone();
    Ok(crate::ingest::assemble(clippy_rel_path, clippy))
}

/// Stub source ingestion entry point for the clippy family.
pub fn ingest_for_source_checks(
    _crawl: &G3RsWorkspaceCrawl,
) -> Result<G3RsClippySourceChecksInput, IngestionError> {
    Err(IngestionError::SourceIngestionNotImplemented)
}

/// Stub file-tree ingestion entry point for the clippy family.
pub fn ingest_for_file_tree_checks(
    _crawl: &G3RsWorkspaceCrawl,
) -> Result<G3RsClippyFileTreeChecksInput, IngestionError> {
    Err(IngestionError::FileTreeIngestionNotImplemented)
}
