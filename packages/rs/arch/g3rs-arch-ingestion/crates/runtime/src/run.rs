use g3_workspace_crawl::G3WorkspaceCrawl;
use g3rs_arch_types::{
    G3RsArchConfigChecksInput, G3RsArchFileTreeChecksInput, G3RsArchSourceChecksInput,
};

pub use crate::error::G3RsArchIngestionError;

/// Result alias for fallible arch ingestion entry points.
pub(crate) type IngestResult<T> = Result<T, G3RsArchIngestionError>;

/// Ingest arch source-checks input from a workspace crawl.
///
/// # Errors
///
/// Returns an error when any required input cannot be located or parsed.
pub fn ingest_for_source_checks(
    crawl: &G3WorkspaceCrawl,
) -> IngestResult<Vec<G3RsArchSourceChecksInput>> {
    crate::source::ingest_for_source_checks(crawl)
}

/// Ingest arch config-checks input from a workspace crawl.
///
/// # Errors
///
/// Returns an error when any required input cannot be located or parsed.
pub fn ingest_for_config_checks(
    crawl: &G3WorkspaceCrawl,
) -> IngestResult<Vec<G3RsArchConfigChecksInput>> {
    crate::config::ingest_for_config_checks(crawl)
}

/// Ingest arch file-tree-checks input from a workspace crawl.
///
/// # Errors
///
/// Returns an error when any required input cannot be located or parsed.
pub fn ingest_for_file_tree_checks(
    crawl: &G3WorkspaceCrawl,
) -> IngestResult<G3RsArchFileTreeChecksInput> {
    crate::file_tree::ingest_for_file_tree_checks(crawl)
}
