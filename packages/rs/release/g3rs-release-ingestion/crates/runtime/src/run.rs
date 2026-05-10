use g3rs_release_types::G3RsReleaseConfigRepo;
use g3rs_release_types::{
    G3RsReleaseConfigChecksInput, G3RsReleaseFileTreeChecksInput, G3RsReleaseSourceChecksInput,
};
use g3rs_workspace_crawl::G3RsWorkspaceCrawl;

pub use g3rs_release_ingestion_types::G3RsReleaseIngestionError as IngestionError;

/// Build config-checks input by crawling release artifacts.
///
/// # Errors
///
/// Returns `IngestionError` when the workspace root is missing, unreadable, or fails to parse.
pub fn ingest_for_config_checks(
    crawl: &G3RsWorkspaceCrawl,
) -> Result<G3RsReleaseConfigChecksInput, IngestionError> {
    crate::ingest::config_result(crawl)
}

/// Build source-checks input by reading per-crate README files.
///
/// # Errors
///
/// Returns `IngestionError` when the workspace root is missing, unreadable, or fails to parse.
pub fn ingest_for_source_checks(
    crawl: &G3RsWorkspaceCrawl,
) -> Result<G3RsReleaseSourceChecksInput, IngestionError> {
    crate::ingest::source_result(crawl)
}

/// Build file-tree-checks input from publish-relevant artifacts.
///
/// # Errors
///
/// Returns `IngestionError` when the workspace root is missing, unreadable, or fails to parse.
pub fn ingest_for_file_tree_checks(
    crawl: &G3RsWorkspaceCrawl,
) -> Result<G3RsReleaseFileTreeChecksInput, IngestionError> {
    crate::ingest::filetree_result(crawl)
}

/// Build repo-root checks input from release configuration files.
///
/// # Errors
///
/// Returns `IngestionError` when the workspace root is missing or the repo-root checks input is not yet implemented.
pub fn ingest_for_repo_root_checks(
    crawl: &G3RsWorkspaceCrawl,
) -> Result<G3RsReleaseConfigRepo, IngestionError> {
    crate::ingest::repo_root_result(crawl)
}
