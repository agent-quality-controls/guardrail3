/// Public ingestion entry point.
use g3rs_fmt_types::{G3RsFmtAstChecksInput, G3RsFmtConfigChecksInput, G3RsFmtFileTreeChecksInput};
use g3rs_workspace_crawl::G3RsWorkspaceCrawl;

/// Re-export of `G3RsFmtIngestionError` so the facade can reach it.
pub use g3rs_fmt_ingestion_types::G3RsFmtIngestionError as IngestionError;

/// Ingest the root fmt config from a workspace crawl into a config checks input.
///
/// Requires all three files at the workspace root:
/// - `rustfmt.toml` (the dot-prefixed `.rustfmt.toml` is NOT accepted)
/// - `Cargo.toml`
/// - `rust-toolchain.toml`
///
/// # Errors
///
/// Returns an error if any of the three files is missing, unreadable, or
/// unparseable.
pub fn ingest_for_config_checks(
    crawl: &G3RsWorkspaceCrawl,
) -> Result<G3RsFmtConfigChecksInput, IngestionError> {
    // 1. Select rustfmt config (required — only rustfmt.toml, no dot variant).
    let rustfmt_entry = crate::select::select_rustfmt_toml(crawl)
        .ok_or(IngestionError::RustfmtTomlNotFound)?;
    if !rustfmt_entry.readable {
        return Err(IngestionError::Unreadable {
            path: rustfmt_entry.path.abs_path.clone(),
            reason: "file is not readable".to_owned(),
        });
    }

    // 2. Select Cargo.toml (required).
    let cargo_entry = crate::select::select_cargo_toml(crawl)
        .ok_or(IngestionError::CargoTomlNotFound)?;
    if !cargo_entry.readable {
        return Err(IngestionError::Unreadable {
            path: cargo_entry.path.abs_path.clone(),
            reason: "file is not readable".to_owned(),
        });
    }

    // 3. Select rust-toolchain.toml (required).
    let toolchain_entry = crate::select::select_toolchain_toml(crawl)
        .ok_or(IngestionError::ToolchainTomlNotFound)?;
    if !toolchain_entry.readable {
        return Err(IngestionError::Unreadable {
            path: toolchain_entry.path.abs_path.clone(),
            reason: "file is not readable".to_owned(),
        });
    }

    // 4. Parse all three.
    let rustfmt = crate::parse::parse_rustfmt_toml(&rustfmt_entry.path.abs_path)?;
    let cargo = crate::parse::parse_cargo_toml(&cargo_entry.path.abs_path)?;
    let toolchain = crate::parse::parse_toolchain_toml(&toolchain_entry.path.abs_path)?;

    // 5. Assemble.
    Ok(crate::ingest::assemble(
        rustfmt_entry.path.rel_path.clone(),
        rustfmt,
        cargo_entry.path.rel_path.clone(),
        cargo,
        toolchain_entry.path.rel_path.clone(),
        toolchain,
    ))
}

/// Stub AST ingestion entry point for the fmt family.
pub fn ingest_for_ast_checks(_crawl: &G3RsWorkspaceCrawl) -> Result<G3RsFmtAstChecksInput, IngestionError> {
    Err(IngestionError::AstIngestionNotImplemented)
}

/// Stub file-tree ingestion entry point for the fmt family.
pub fn ingest_for_file_tree_checks(
    _crawl: &G3RsWorkspaceCrawl,
) -> Result<G3RsFmtFileTreeChecksInput, IngestionError> {
    Err(IngestionError::FileTreeIngestionNotImplemented)
}
