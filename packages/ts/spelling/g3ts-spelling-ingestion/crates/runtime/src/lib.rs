//! Workspace ingestion runtime for g3ts spelling hooks.

/// Reading and parsing the root cspell config.
mod config;
/// Reading and parsing the root package.json for spelling-related script and dependency state.
mod package;
/// Helpers for locating spelling-related files within the workspace crawl.
mod roots;
/// Top-level ingestion entry point.
mod run;
/// Reading and parsing the root syncpack config for spelling-related policy.
mod syncpack;

#[cfg(feature = "api")]
pub use run::ingest_for_config_checks;
