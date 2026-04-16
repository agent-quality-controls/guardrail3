use g3rs_arch_types::{
    G3RsArchConfigChecksInput, G3RsArchFileTreeChecksInput, G3RsArchSourceChecksInput,
};
use g3rs_workspace_crawl::G3RsWorkspaceCrawl;

pub use crate::error::G3RsArchIngestionError;

pub fn ingest_for_source_checks(
    crawl: &G3RsWorkspaceCrawl,
) -> Result<Vec<G3RsArchSourceChecksInput>, G3RsArchIngestionError> {
    crate::source::ingest_for_source_checks(crawl)
}

pub fn ingest_for_config_checks(
    crawl: &G3RsWorkspaceCrawl,
) -> Result<Vec<G3RsArchConfigChecksInput>, G3RsArchIngestionError> {
    crate::config::ingest_for_config_checks(crawl)
}

pub fn ingest_for_file_tree_checks(
    crawl: &G3RsWorkspaceCrawl,
) -> Result<G3RsArchFileTreeChecksInput, G3RsArchIngestionError> {
    crate::file_tree::ingest_for_file_tree_checks(crawl)
}
