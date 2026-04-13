use std::ffi::OsStr;

use g3rs_release_types::{
    G3RsReleaseConfigChecksInput, G3RsReleaseFileTreeChecksInput, G3RsReleaseSourceChecksInput,
};
use g3rs_workspace_crawl::G3RsWorkspaceCrawl;

pub use g3rs_release_ingestion_types::G3RsReleaseIngestionError as IngestionError;

pub fn ingest_for_config_checks(
    crawl: &G3RsWorkspaceCrawl,
) -> Result<G3RsReleaseConfigChecksInput, IngestionError> {
    Ok(ingest_for_config_checks_with_path(
        crawl,
        std::env::var_os("PATH").as_deref(),
    ))
}

pub(crate) fn ingest_for_config_checks_with_path(
    crawl: &G3RsWorkspaceCrawl,
    path_env: Option<&OsStr>,
) -> G3RsReleaseConfigChecksInput {
    crate::ingest::collect(crawl, path_env).config
}

pub fn ingest_for_source_checks(
    crawl: &G3RsWorkspaceCrawl,
) -> Result<G3RsReleaseSourceChecksInput, IngestionError> {
    Ok(crate::ingest::collect(crawl, std::env::var_os("PATH").as_deref()).source)
}

pub fn ingest_for_file_tree_checks(
    crawl: &G3RsWorkspaceCrawl,
) -> Result<G3RsReleaseFileTreeChecksInput, IngestionError> {
    Ok(crate::ingest::collect(crawl, std::env::var_os("PATH").as_deref()).filetree)
}
