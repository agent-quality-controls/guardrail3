/// Public ingestion entry point.
use g3rs_garde_config_ingestion_types::G3RsGardeConfigIngestionResult;
use g3rs_workspace_crawl::G3RsWorkspaceCrawl;

/// Re-export of `G3RsGardeConfigIngestionError` so the facade can reach it.
pub use g3rs_garde_config_ingestion_types::G3RsGardeConfigIngestionError as IngestionError;

/// Ingest garde config from a workspace crawl.
///
/// Cargo.toml is required. Clippy config is optional — if absent,
/// `clippy_bans` will be `None` in the result.
///
/// # Errors
///
/// Returns an error if Cargo.toml is missing, unreadable, or unparseable.
/// Clippy config errors are silently treated as absent.
pub fn ingest(
    crawl: &G3RsWorkspaceCrawl,
) -> Result<G3RsGardeConfigIngestionResult, IngestionError> {
    // 1. Select and parse Cargo.toml (required)
    let cargo_entry = crate::select::select_cargo_toml(crawl)
        .ok_or(IngestionError::CargoTomlNotFound)?;

    if !cargo_entry.readable {
        return Err(IngestionError::Unreadable {
            path: cargo_entry.path.abs_path.clone(),
            reason: "file is not readable".to_owned(),
        });
    }

    let cargo = crate::parse::parse_cargo_toml(&cargo_entry.path.abs_path)?;
    let dependency = crate::ingest::assemble_dependency(
        cargo_entry.path.rel_path.clone(),
        cargo,
    );

    // 2. Select and parse clippy config (optional)
    let clippy_bans = crate::select::select_clippy_toml(crawl)
        .filter(|entry| entry.readable)
        .and_then(|entry| {
            crate::parse::parse_clippy_toml(&entry.path.abs_path)
                .ok()
                .map(|clippy| {
                    crate::ingest::assemble_clippy_bans(
                        entry.path.rel_path.clone(),
                        clippy,
                    )
                })
        });

    Ok(G3RsGardeConfigIngestionResult {
        dependency,
        clippy_bans,
    })
}
