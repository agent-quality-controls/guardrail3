use g3rs_release_types::G3RsReleaseConfigRepo;
use g3rs_release_types::{
    G3RsReleaseConfigChecksInput, G3RsReleaseFileTreeChecksInput, G3RsReleaseSourceChecksInput,
};
use g3rs_workspace_crawl::G3RsWorkspaceCrawl;

pub use g3rs_release_ingestion_types::G3RsReleaseIngestionError as IngestionError;

pub fn ingest_for_config_checks(
    crawl: &G3RsWorkspaceCrawl,
) -> Result<G3RsReleaseConfigChecksInput, IngestionError> {
    crate::ingest::config_result(crawl)
}

pub fn ingest_for_source_checks(
    crawl: &G3RsWorkspaceCrawl,
) -> Result<G3RsReleaseSourceChecksInput, IngestionError> {
    crate::ingest::source_result(crawl)
}

pub fn ingest_for_file_tree_checks(
    crawl: &G3RsWorkspaceCrawl,
) -> Result<G3RsReleaseFileTreeChecksInput, IngestionError> {
    crate::ingest::filetree_result(crawl)
}

pub fn ingest_for_repo_root_checks(
    crawl: &G3RsWorkspaceCrawl,
) -> Result<G3RsReleaseConfigRepo, IngestionError> {
    crate::ingest::repo_root_result(crawl)
}
