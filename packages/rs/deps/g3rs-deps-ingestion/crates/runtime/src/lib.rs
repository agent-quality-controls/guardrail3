//! Runtime ingestion pipeline for the g3rs deps family.

/// Centralized filesystem boundary for the ingestion runtime.
mod fs;
/// Workspace and member assembly into the deps checks input.
mod ingest;
/// Parsing helpers for `Cargo.toml` and `guardrail3-rs.toml` artifacts.
mod parse;
/// Top-level entry points exported as the runtime's public API.
mod run;
/// Workspace selection and member discovery.
mod select;

#[cfg(feature = "ingest")]
pub use run::{
    IngestionError, ingest_for_config_checks, ingest_for_file_tree_checks, ingest_for_source_checks,
};
