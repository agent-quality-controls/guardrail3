/// Public ingestion entry point.
use g3rs_fmt_types::{
    G3RsFmtCargoState, G3RsFmtConfigChecksInput, G3RsFmtEscapeHatch, G3RsFmtFileTreeChecksInput,
    G3RsFmtRustfmtConfigState, G3RsFmtSourceChecksInput, G3RsFmtToolchainState,
};
use g3rs_workspace_crawl::G3RsWorkspaceCrawl;

/// Re-export of `G3RsFmtIngestionError` so the facade can reach it.
pub use g3rs_fmt_ingestion_types::G3RsFmtIngestionError as IngestionError;

pub fn ingest_for_config_checks(
    crawl: &G3RsWorkspaceCrawl,
) -> Result<G3RsFmtConfigChecksInput, IngestionError> {
    let rustfmt_entry = crate::select::select_active_rustfmt_config(crawl)
        .ok_or(IngestionError::RustfmtTomlNotFound)?;
    let rustfmt_state = ingest_rustfmt_state(rustfmt_entry)?;

    Ok(G3RsFmtConfigChecksInput {
        rustfmt_rel_path: rustfmt_entry.path.rel_path.clone(),
        rustfmt_state,
        cargo_rel_path: "Cargo.toml".to_owned(),
        cargo_state: ingest_cargo_state(crawl),
        toolchain_rel_path: "rust-toolchain.toml".to_owned(),
        toolchain_state: ingest_toolchain_state(crawl),
        escape_hatches: ingest_escape_hatches(crawl),
    })
}

pub fn ingest_for_source_checks(
    _crawl: &G3RsWorkspaceCrawl,
) -> Result<G3RsFmtSourceChecksInput, IngestionError> {
    Err(IngestionError::SourceIngestionNotImplemented)
}

pub fn ingest_for_file_tree_checks(
    crawl: &G3RsWorkspaceCrawl,
) -> Result<G3RsFmtFileTreeChecksInput, IngestionError> {
    Ok(G3RsFmtFileTreeChecksInput {
        root_rustfmt_toml_rel_path: crate::select::select_root_rustfmt_toml(crawl)
            .map(|entry| entry.path.rel_path.clone()),
        root_dot_rustfmt_toml_rel_path: crate::select::select_root_dot_rustfmt_toml(crawl)
            .map(|entry| entry.path.rel_path.clone()),
        nested_config_files: crate::select::collect_nested_config_files(crawl),
        dual_conflict_dirs: crate::select::collect_dual_conflict_dirs(crawl),
    })
}

fn ingest_rustfmt_state(
    entry: &g3rs_workspace_crawl::G3RsWorkspaceEntry,
) -> Result<G3RsFmtRustfmtConfigState, IngestionError> {
    if !entry.readable {
        return Ok(G3RsFmtRustfmtConfigState::ParseError);
    }
    match crate::parse::parse_rustfmt_toml(&entry.path.abs_path) {
        Ok(rustfmt) => Ok(G3RsFmtRustfmtConfigState::Parsed(rustfmt)),
        Err(IngestionError::ParseFailed { .. }) => Ok(G3RsFmtRustfmtConfigState::ParseError),
        Err(err) => Err(err),
    }
}

fn ingest_cargo_state(crawl: &G3RsWorkspaceCrawl) -> G3RsFmtCargoState {
    let Some(entry) = crate::select::select_cargo_toml(crawl) else {
        return G3RsFmtCargoState::Missing;
    };
    if !entry.readable {
        return G3RsFmtCargoState::ParseError;
    }
    match crate::parse::parse_cargo_toml(&entry.path.abs_path) {
        Ok(cargo) => G3RsFmtCargoState::Parsed(cargo),
        Err(IngestionError::ParseFailed { .. }) => G3RsFmtCargoState::ParseError,
        Err(IngestionError::Unreadable { .. }) => G3RsFmtCargoState::ParseError,
        Err(_) => G3RsFmtCargoState::ParseError,
    }
}

fn ingest_toolchain_state(crawl: &G3RsWorkspaceCrawl) -> G3RsFmtToolchainState {
    let Some(entry) = crate::select::select_toolchain_toml(crawl) else {
        return G3RsFmtToolchainState::Missing;
    };
    if !entry.readable {
        return G3RsFmtToolchainState::ParseError;
    }
    match crate::parse::parse_toolchain_toml(&entry.path.abs_path) {
        Ok(toolchain) => G3RsFmtToolchainState::Parsed(toolchain),
        Err(IngestionError::ParseFailed { .. }) => G3RsFmtToolchainState::ParseError,
        Err(IngestionError::Unreadable { .. }) => G3RsFmtToolchainState::ParseError,
        Err(_) => G3RsFmtToolchainState::ParseError,
    }
}

fn ingest_escape_hatches(crawl: &G3RsWorkspaceCrawl) -> Vec<G3RsFmtEscapeHatch> {
    let Some(entry) = crate::select::select_guardrail3_toml(crawl) else {
        return Vec::new();
    };
    if !entry.readable {
        return Vec::new();
    }
    let Ok(content) = crate::fs::read_to_string(&entry.path.abs_path) else {
        return Vec::new();
    };
    let Ok(value) = toml::from_str::<toml::Value>(&content) else {
        return Vec::new();
    };
    value
        .get("escape_hatches")
        .and_then(toml::Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(|entry| {
            let table = entry.as_table()?;
            Some(G3RsFmtEscapeHatch {
                family: table.get("family")?.as_str()?.to_owned(),
                file: table.get("file")?.as_str()?.to_owned(),
                kind: table.get("kind")?.as_str()?.to_owned(),
                selector: table.get("selector")?.as_str()?.to_owned(),
                reason: table.get("reason")?.as_str()?.to_owned(),
            })
        })
        .collect()
}
