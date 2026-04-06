/// Public ingestion entry point.
use g3rs_release_config_checks::G3RsReleaseConfigChecksInput;
use g3rs_workspace_crawl::G3RsWorkspaceCrawl;

/// Re-export of `G3RsReleaseConfigIngestionError` so the facade can reach it.
pub use g3rs_release_config_ingestion_types::G3RsReleaseConfigIngestionError as IngestionError;

/// Ingest release config from a workspace crawl into a checks input.
///
/// Parses `Cargo.toml` (required), `release-plz.toml` / `.release-plz.toml`
/// (optional), and `cliff.toml` (optional) from the workspace root.
///
/// # Errors
///
/// Returns an error if `Cargo.toml` is missing, unreadable, or unparseable.
/// Optional files that are missing, unreadable, or unparseable are silently
/// treated as `None`.
pub fn ingest(
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
    let (release_plz_rel_path, release_plz) = crate::select::select_release_plz_toml(crawl)
        .and_then(|entry| {
            if !entry.readable {
                return None;
            }
            crate::parse::parse_release_plz_toml(&entry.path.abs_path)
                .ok()
                .map(|parsed| (entry.path.rel_path.clone(), parsed))
        })
        .map_or((None, None), |(path, toml)| (Some(path), Some(toml)));

    // --- cliff.toml (optional) ---
    let (cliff_rel_path, cliff) = crate::select::select_cliff_toml(crawl)
        .and_then(|entry| {
            if !entry.readable {
                return None;
            }
            crate::parse::parse_cliff_toml(&entry.path.abs_path)
                .ok()
                .map(|parsed| (entry.path.rel_path.clone(), parsed))
        })
        .map_or((None, None), |(path, toml)| (Some(path), Some(toml)));

    Ok(crate::ingest::assemble(
        cargo_rel_path,
        cargo,
        release_plz_rel_path,
        release_plz,
        cliff_rel_path,
        cliff,
    ))
}
