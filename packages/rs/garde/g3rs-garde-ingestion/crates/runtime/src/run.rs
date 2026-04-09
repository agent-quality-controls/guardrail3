/// Public ingestion entry point.
use g3rs_garde_ast_checks_types::G3RsGardeAstChecksInput;
use g3rs_garde_types::{G3RsGardeConfigChecksInput, G3RsGardeFileTreeChecksInput};
use g3rs_workspace_crawl::G3RsWorkspaceCrawl;

/// Re-export of `G3RsGardeIngestionError` so the facade can reach it.
pub use g3rs_garde_ingestion_types::G3RsGardeIngestionError as IngestionError;

/// Ingest garde config from a workspace crawl into a config checks input.
///
/// Cargo.toml is required. Clippy config is optional — if absent,
/// the clippy fields will be `None` in the result and clippy ban
/// checks will be skipped by the checks package.
///
/// # Errors
///
/// Returns an error if Cargo.toml is missing, unreadable, or unparseable.
/// Clippy config errors are silently treated as absent.
pub fn ingest_for_config_checks(
    crawl: &G3RsWorkspaceCrawl,
) -> Result<G3RsGardeConfigChecksInput, IngestionError> {
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

    // 2. Select and parse clippy config (optional)
    let (clippy_rel_path, clippy) = crate::select::select_clippy_toml(crawl)
        .filter(|entry| entry.readable)
        .and_then(|entry| {
            crate::parse::parse_clippy_toml(&entry.path.abs_path)
                .ok()
                .map(|parsed| (entry.path.rel_path.clone(), parsed))
        })
        .map_or((None, None), |(path, parsed)| (Some(path), Some(parsed)));

    Ok(crate::ingest::assemble(
        cargo_entry.path.rel_path.clone(),
        cargo,
        clippy_rel_path,
        clippy,
    ))
}

/// Stub AST ingestion entry point for the garde family.
pub fn ingest_for_ast_checks(_crawl: &G3RsWorkspaceCrawl) -> Result<G3RsGardeAstChecksInput, IngestionError> {
    Err(IngestionError::AstIngestionNotImplemented)
}

/// Stub file-tree ingestion entry point for the garde family.
pub fn ingest_for_file_tree_checks(
    _crawl: &G3RsWorkspaceCrawl,
) -> Result<G3RsGardeFileTreeChecksInput, IngestionError> {
    Err(IngestionError::FileTreeIngestionNotImplemented)
}
