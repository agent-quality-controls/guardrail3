/// Public ingestion entry point.
use g3rs_clippy_types::{
    G3RsClippyAstChecksInput, G3RsClippyConfigChecksInput, G3RsClippyFileTreeChecksInput,
};
use g3rs_workspace_crawl::G3RsWorkspaceCrawl;

/// Re-export of `G3RsClippyConfigIngestionError` so the facade can reach it.
pub use g3rs_clippy_config_ingestion_types::G3RsClippyConfigIngestionError as IngestionError;

/// Ingest the root clippy config from a workspace crawl into a config checks input.
///
/// Prefers `clippy.toml` over `.clippy.toml` when both exist.
///
/// # Errors
///
/// Returns an error if the clippy config is missing, unreadable, or unparseable.
pub fn ingest_config(
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

/// Stub AST ingestion entry point for the clippy family.
pub fn ingest_ast(
    _crawl: &G3RsWorkspaceCrawl,
) -> Result<G3RsClippyAstChecksInput, IngestionError> {
    Err(IngestionError::AstIngestionNotImplemented)
}

/// Stub file-tree ingestion entry point for the clippy family.
pub fn ingest_file_tree(
    _crawl: &G3RsWorkspaceCrawl,
) -> Result<G3RsClippyFileTreeChecksInput, IngestionError> {
    Err(IngestionError::FileTreeIngestionNotImplemented)
}
