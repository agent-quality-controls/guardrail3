//! Workspace ingestion runtime for g3ts tsconfig hooks.

/// Resolution of the tsconfig `extends` chain.
mod resolve;
/// Top-level ingestion entry point.
mod run;

#[cfg(feature = "ingest")]
pub use run::ingest_for_config_checks;

#[cfg(test)]
use tempfile as _;
