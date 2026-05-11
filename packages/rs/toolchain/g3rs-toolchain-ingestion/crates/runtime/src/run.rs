use g3_workspace_crawl::G3WorkspaceCrawl;
/// Public ingestion entry point.
use g3rs_toolchain_types::{
    G3RsToolchainConfigChecksInput, G3RsToolchainFileTreeChecksInput,
    G3RsToolchainSourceChecksInput,
};

/// Re-export of `G3RsToolchainIngestionError` so the facade can reach it.
pub use g3rs_toolchain_ingestion_types::G3RsToolchainIngestionError as IngestionError;

/// Ingest toolchain config from a workspace crawl into a config checks input.
///
/// Parses `rust-toolchain.toml` (required) and `Cargo.toml` (optional) from the
/// workspace root, returning the checks input directly.
///
/// # Errors
///
/// Returns an error if `rust-toolchain.toml` is missing, unreadable, or unparseable.
/// `Cargo.toml` may be absent, but if present it must be readable and parseable.
pub fn ingest_for_config_checks(
    crawl: &G3WorkspaceCrawl,
) -> Result<G3RsToolchainConfigChecksInput, IngestionError> {
    let toolchain_entry = g3_workspace_crawl::root_file(crawl, "rust-toolchain.toml")
        .ok_or(IngestionError::ToolchainTomlNotFound)?;

    if !toolchain_entry.readable {
        return Err(IngestionError::Unreadable {
            path: toolchain_entry.path.abs_path.clone(),
            reason: "file is not readable".to_owned(),
        });
    }

    let toolchain_toml = crate::parse::parse_toolchain_toml(&toolchain_entry.path.abs_path)?;
    let toolchain_rel_path = toolchain_entry.path.rel_path.clone();

    let (cargo_rel_path, cargo_toml) =
        if let Some(cargo_entry) = g3_workspace_crawl::root_file(crawl, "Cargo.toml") {
            if !cargo_entry.readable {
                return Err(IngestionError::Unreadable {
                    path: cargo_entry.path.abs_path.clone(),
                    reason: "file is not readable".to_owned(),
                });
            }
            let cargo = crate::parse::parse_cargo_toml(&cargo_entry.path.abs_path)?;
            (Some(cargo_entry.path.rel_path.clone()), Some(cargo))
        } else {
            (None, None)
        };

    Ok(crate::ingest::assemble(
        toolchain_rel_path,
        toolchain_toml,
        cargo_rel_path,
        cargo_toml,
    ))
}

/// Stub source ingestion entry point for the toolchain family.
///
/// # Errors
///
/// Always returns [`IngestionError::SourceIngestionNotImplemented`] until source
/// ingestion is supported by the toolchain family.
pub const fn ingest_for_source_checks(
    _crawl: &G3WorkspaceCrawl,
) -> Result<G3RsToolchainSourceChecksInput, IngestionError> {
    Err(IngestionError::SourceIngestionNotImplemented)
}

/// Build the toolchain file-tree checks input from a workspace crawl.
///
/// # Errors
///
/// Currently never returns an error; the [`Result`] return type is reserved for
/// future ingestion failures and matches the family's other entry points.
#[allow(
    clippy::unnecessary_wraps,
    reason = "shape parity across `ingest_for_*` entry points; future failures will populate the Err arm"
)]
pub fn ingest_for_file_tree_checks(
    crawl: &G3WorkspaceCrawl,
) -> Result<G3RsToolchainFileTreeChecksInput, IngestionError> {
    Ok(G3RsToolchainFileTreeChecksInput {
        toolchain_toml_rel_path: g3_workspace_crawl::root_file(crawl, "rust-toolchain.toml")
            .map(|entry| entry.path.rel_path.clone()),
        legacy_toolchain_rel_path: g3_workspace_crawl::root_file(crawl, "rust-toolchain")
            .map(|entry| entry.path.rel_path.clone()),
    })
}

#[cfg(test)]
#[path = "run_tests/mod.rs"]
mod run_tests;
