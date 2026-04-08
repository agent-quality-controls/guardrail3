/// Public ingestion entry point.
use g3rs_cargo_types::{
    G3RsCargoAstChecksInput, G3RsCargoConfigChecksInput, G3RsCargoFileTreeChecksInput,
};
use g3rs_workspace_crawl::G3RsWorkspaceCrawl;

/// Re-export of `G3RsCargoConfigIngestionError` so the facade can reach it.
pub use g3rs_cargo_config_ingestion_types::G3RsCargoConfigIngestionError as IngestionError;

/// Ingest the root `Cargo.toml` from a workspace crawl into a config checks input.
///
/// # Errors
///
/// Returns an error if the `Cargo.toml` is missing, unreadable, or unparseable.
pub fn ingest_for_config_checks(
    crawl: &G3RsWorkspaceCrawl,
) -> Result<G3RsCargoConfigChecksInput, IngestionError> {
    let entry = crate::select::select_root_cargo_toml(crawl)
        .ok_or(IngestionError::CargoTomlNotFound)?;

    if !entry.readable {
        return Err(IngestionError::Unreadable {
            path: entry.path.abs_path.clone(),
            reason: "file is not readable".to_owned(),
        });
    }

    let cargo = crate::parse::parse_cargo_toml(&entry.path.abs_path)?;
    let cargo_rel_path = entry.path.rel_path.clone();
    Ok(crate::ingest::assemble(cargo_rel_path, cargo))
}

/// Stub AST ingestion entry point for the Cargo family.
pub fn ingest_for_ast_checks(_crawl: &G3RsWorkspaceCrawl) -> Result<G3RsCargoAstChecksInput, IngestionError> {
    Err(IngestionError::AstIngestionNotImplemented)
}

/// Stub file-tree ingestion entry point for the Cargo family.
pub fn ingest_for_file_tree_checks(
    _crawl: &G3RsWorkspaceCrawl,
) -> Result<G3RsCargoFileTreeChecksInput, IngestionError> {
    Err(IngestionError::FileTreeIngestionNotImplemented)
}
