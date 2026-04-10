/// Public ingestion entry point.
use g3rs_release_types::{
    G3RsReleaseSourceChecksInput, G3RsReleaseConfigChecksInput, G3RsReleaseFileTreeChecksInput,
};
use g3rs_workspace_crawl::G3RsWorkspaceCrawl;

/// Re-export of `G3RsReleaseIngestionError` so the facade can reach it.
pub use g3rs_release_ingestion_types::G3RsReleaseIngestionError as IngestionError;

/// Ingest release config from a workspace crawl into a config checks input.
///
/// Parses `Cargo.toml` (required), `release-plz.toml` / `.release-plz.toml`
/// (optional), and `cliff.toml` (optional) from the workspace root.
///
/// # Errors
///
/// Returns an error if `Cargo.toml` is missing, unreadable, or unparseable.
/// Optional files may be absent, but if present they must be readable and
/// parseable.
pub fn ingest_for_config_checks(
    crawl: &G3RsWorkspaceCrawl,
) -> Result<G3RsReleaseConfigChecksInput, IngestionError> {
    // --- Cargo.toml (required) ---
    let cargo_entry = crate::select::select_cargo_toml(crawl)
        .ok_or(IngestionError::CargoTomlNotFound)?;

    if !cargo_entry.readable {
        return Err(IngestionError::Unreadable {
            path: cargo_entry.path.abs_path.clone(),
            reason: "file is not readable".to_owned(),
        });
    }

    let cargo = crate::parse::parse_cargo_toml(&cargo_entry.path.abs_path)?;
    let cargo_rel_path = cargo_entry.path.rel_path.clone();

    // --- release-plz.toml (optional) ---
    let (release_plz_rel_path, release_plz) = if let Some(entry) = crate::select::select_release_plz_toml(crawl) {
        if !entry.readable {
            return Err(IngestionError::Unreadable {
                path: entry.path.abs_path.clone(),
                reason: "file is not readable".to_owned(),
            });
        }
        let parsed = crate::parse::parse_release_plz_toml(&entry.path.abs_path)?;
        (Some(entry.path.rel_path.clone()), Some(parsed))
    } else {
        (None, None)
    };

    // --- cliff.toml (optional) ---
    let (cliff_rel_path, cliff) = if let Some(entry) = crate::select::select_cliff_toml(crawl) {
        if !entry.readable {
            return Err(IngestionError::Unreadable {
                path: entry.path.abs_path.clone(),
                reason: "file is not readable".to_owned(),
            });
        }
        let parsed = crate::parse::parse_cliff_toml(&entry.path.abs_path)?;
        (Some(entry.path.rel_path.clone()), Some(parsed))
    } else {
        (None, None)
    };

    Ok(crate::ingest::assemble(
        cargo_rel_path,
        cargo,
        release_plz_rel_path,
        release_plz,
        cliff_rel_path,
        cliff,
    ))
}

/// Stub source ingestion entry point for the release family.
pub fn ingest_for_source_checks(
    _crawl: &G3RsWorkspaceCrawl,
) -> Result<G3RsReleaseSourceChecksInput, IngestionError> {
    Err(IngestionError::SourceIngestionNotImplemented)
}

/// Stub file-tree ingestion entry point for the release family.
pub fn ingest_for_file_tree_checks(
    _crawl: &G3RsWorkspaceCrawl,
) -> Result<G3RsReleaseFileTreeChecksInput, IngestionError> {
    Err(IngestionError::FileTreeIngestionNotImplemented)
}
