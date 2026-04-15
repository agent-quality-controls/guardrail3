use std::ffi::OsStr;

use g3rs_release_types::G3RsReleaseConfigRepo;
use g3rs_release_types::{
    G3RsReleaseConfigChecksInput, G3RsReleaseFileTreeChecksInput, G3RsReleaseSourceChecksInput,
};
use g3rs_workspace_crawl::G3RsWorkspaceCrawl;

pub use g3rs_release_ingestion_types::G3RsReleaseIngestionError as IngestionError;

pub fn ingest_for_config_checks(
    crawl: &G3RsWorkspaceCrawl,
) -> Result<G3RsReleaseConfigChecksInput, IngestionError> {
    require_pointed_workspace_root(crawl)?;
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
    require_pointed_workspace_root(crawl)?;
    Ok(crate::ingest::collect(crawl, std::env::var_os("PATH").as_deref()).source)
}

pub fn ingest_for_file_tree_checks(
    crawl: &G3RsWorkspaceCrawl,
) -> Result<G3RsReleaseFileTreeChecksInput, IngestionError> {
    require_pointed_workspace_root(crawl)?;
    Ok(crate::ingest::collect(crawl, std::env::var_os("PATH").as_deref()).filetree)
}

pub fn ingest_for_repo_root_checks(
    _crawl: &G3RsWorkspaceCrawl,
) -> Result<G3RsReleaseConfigRepo, IngestionError> {
    Err(IngestionError::RepoRootChecksNotImplemented)
}

fn require_pointed_workspace_root(crawl: &G3RsWorkspaceCrawl) -> Result<(), IngestionError> {
    let Some(entry) = crawl.entry("Cargo.toml") else {
        return Err(IngestionError::CargoTomlNotFound);
    };
    if !entry.readable {
        return Err(IngestionError::Unreadable {
            path: entry.path.abs_path.clone(),
            reason: "file is not readable".to_owned(),
        });
    }
    let content = crate::parse::read_to_string(&entry.path.abs_path)?;
    let cargo = crate::parse::parse_cargo_toml(&content, &entry.path.abs_path)?;
    if cargo.workspace.is_none() {
        return Err(IngestionError::NormalizationFailed {
            path: entry.path.abs_path.clone(),
            reason: "root Cargo.toml must declare a [workspace] table".to_owned(),
        });
    }
    Ok(())
}
