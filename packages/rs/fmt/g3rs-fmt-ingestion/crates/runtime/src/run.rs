use g3_workspace_crawl::G3WorkspaceCrawl;
/// Public ingestion entry point.
use g3rs_fmt_types::{
    G3RsFmtCargoState, G3RsFmtConfigChecksInput, G3RsFmtFileTreeChecksInput,
    G3RsFmtRustPolicyState, G3RsFmtRustfmtConfigState, G3RsFmtSourceChecksInput,
    G3RsFmtToolchainState, G3RsFmtWaiver,
};

/// Re-export of `G3RsFmtIngestionError` so the facade can reach it.
pub use g3rs_fmt_ingestion_types::G3RsFmtIngestionError as IngestionError;

/// Implements this item.
///
/// # Errors
/// Returns an error when the underlying operation fails.
pub fn ingest_for_config_checks(
    crawl: &G3WorkspaceCrawl,
) -> Result<G3RsFmtConfigChecksInput, IngestionError> {
    let rustfmt_entry = crate::select::select_active_rustfmt_config(crawl)
        .ok_or(IngestionError::RustfmtTomlNotFound)?;
    let (rustfmt_state, rustfmt_explicit_keys) = ingest_rustfmt_state(rustfmt_entry);

    Ok(G3RsFmtConfigChecksInput {
        rustfmt_rel_path: rustfmt_entry.path.rel_path.clone(),
        rustfmt_state,
        rustfmt_explicit_keys,
        cargo_rel_path: "Cargo.toml".to_owned(),
        cargo_state: ingest_cargo_state(crawl),
        toolchain_rel_path: "rust-toolchain.toml".to_owned(),
        toolchain_state: ingest_toolchain_state(crawl),
        rust_policy: ingest_rust_policy(crawl),
    })
}

/// Implements this item.
///
/// # Errors
/// Returns an error when the underlying operation fails.
pub const fn ingest_for_source_checks(
    _crawl: &G3WorkspaceCrawl,
) -> Result<G3RsFmtSourceChecksInput, IngestionError> {
    Err(IngestionError::SourceIngestionNotImplemented)
}

/// Implements this item.
#[must_use]
pub fn ingest_for_file_tree_checks(crawl: &G3WorkspaceCrawl) -> G3RsFmtFileTreeChecksInput {
    G3RsFmtFileTreeChecksInput {
        root_rustfmt_toml_rel_path: crate::select::select_root_rustfmt_toml(crawl)
            .map(|entry| entry.path.rel_path.clone()),
        root_dot_rustfmt_toml_rel_path: crate::select::select_root_dot_rustfmt_toml(crawl)
            .map(|entry| entry.path.rel_path.clone()),
        nested_config_files: crate::select::collect_nested_config_files(crawl),
        dual_conflict_dirs: crate::select::collect_dual_conflict_dirs(crawl),
    }
}

/// Typed view of a parsed `rustfmt.toml` paired with the names of explicit top-level keys.
type RustfmtIngestion = (G3RsFmtRustfmtConfigState, Vec<String>);

/// Implements `ingest rustfmt state`.
fn ingest_rustfmt_state(entry: &g3_workspace_crawl::G3WorkspaceEntry) -> RustfmtIngestion {
    if !entry.readable {
        return (G3RsFmtRustfmtConfigState::Unreadable, Vec::new());
    }
    match crate::parse::parse_rustfmt_toml(&entry.path.abs_path) {
        Ok((rustfmt, explicit_keys)) => (
            G3RsFmtRustfmtConfigState::Parsed(Box::new(rustfmt)),
            explicit_keys,
        ),
        Err(IngestionError::Unreadable { .. }) => {
            (G3RsFmtRustfmtConfigState::Unreadable, Vec::new())
        }
        Err(
            IngestionError::ParseFailed { .. }
            | IngestionError::RustfmtTomlNotFound
            | IngestionError::CargoTomlNotFound
            | IngestionError::ToolchainTomlNotFound
            | IngestionError::SourceIngestionNotImplemented
            | IngestionError::FileTreeIngestionNotImplemented,
        ) => (G3RsFmtRustfmtConfigState::ParseError, Vec::new()),
    }
}

/// Implements `ingest cargo state`.
fn ingest_cargo_state(crawl: &G3WorkspaceCrawl) -> G3RsFmtCargoState {
    let Some(entry) = crate::select::select_cargo_toml(crawl) else {
        return G3RsFmtCargoState::Missing;
    };
    if !entry.readable {
        return G3RsFmtCargoState::Unreadable;
    }
    match crate::parse::parse_cargo_toml(&entry.path.abs_path) {
        Ok(cargo) => G3RsFmtCargoState::Parsed(Box::new(cargo)),
        Err(IngestionError::Unreadable { .. }) => G3RsFmtCargoState::Unreadable,
        Err(
            IngestionError::ParseFailed { .. }
            | IngestionError::RustfmtTomlNotFound
            | IngestionError::CargoTomlNotFound
            | IngestionError::ToolchainTomlNotFound
            | IngestionError::SourceIngestionNotImplemented
            | IngestionError::FileTreeIngestionNotImplemented,
        ) => G3RsFmtCargoState::ParseError,
    }
}

/// Implements `ingest toolchain state`.
fn ingest_toolchain_state(crawl: &G3WorkspaceCrawl) -> G3RsFmtToolchainState {
    let Some(entry) = crate::select::select_toolchain_toml(crawl) else {
        return G3RsFmtToolchainState::Missing;
    };
    if !entry.readable {
        return G3RsFmtToolchainState::Unreadable;
    }
    match crate::parse::parse_toolchain_toml(&entry.path.abs_path) {
        Ok(toolchain) => G3RsFmtToolchainState::Parsed(Box::new(toolchain)),
        Err(IngestionError::Unreadable { .. }) => G3RsFmtToolchainState::Unreadable,
        Err(
            IngestionError::ParseFailed { .. }
            | IngestionError::RustfmtTomlNotFound
            | IngestionError::CargoTomlNotFound
            | IngestionError::ToolchainTomlNotFound
            | IngestionError::SourceIngestionNotImplemented
            | IngestionError::FileTreeIngestionNotImplemented,
        ) => G3RsFmtToolchainState::ParseError,
    }
}

/// Implements `ingest rust policy`.
fn ingest_rust_policy(crawl: &G3WorkspaceCrawl) -> G3RsFmtRustPolicyState {
    let Some(entry) = crate::select::select_rust_policy_toml(crawl) else {
        return G3RsFmtRustPolicyState::Missing;
    };
    if !entry.readable {
        return G3RsFmtRustPolicyState::Unreadable {
            rel_path: entry.path.rel_path.clone(),
            reason: "file is not readable".to_owned(),
        };
    }
    let Ok(content) = crate::fs::read_to_string(&entry.path.abs_path) else {
        return G3RsFmtRustPolicyState::Unreadable {
            rel_path: entry.path.rel_path.clone(),
            reason: "file is not readable".to_owned(),
        };
    };
    let parsed = match g3rs_toml_parser::parse(&content) {
        Ok(parsed) => parsed,
        Err(err) => {
            return G3RsFmtRustPolicyState::ParseError {
                rel_path: entry.path.rel_path.clone(),
                reason: err.to_string(),
            };
        }
    };
    G3RsFmtRustPolicyState::Parsed {
        rel_path: entry.path.rel_path.clone(),
        waivers: parsed
            .waivers
            .into_iter()
            .map(|waiver| G3RsFmtWaiver {
                rule: waiver.rule,
                file: waiver.file,
                selector: waiver.selector,
                reason: waiver.reason,
            })
            .collect(),
    }
}
