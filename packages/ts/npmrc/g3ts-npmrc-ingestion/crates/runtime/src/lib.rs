//! Workspace ingestion runtime for g3ts .npmrc hooks.

/// Crawl-driven ingestion pipeline that produces the .npmrc checks input.
mod run;

#[cfg(feature = "ingest")]
pub use run::ingest_for_config_checks;
