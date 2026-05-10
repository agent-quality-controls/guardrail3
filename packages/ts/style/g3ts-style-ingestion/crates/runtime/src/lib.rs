//! Workspace ingestion runtime for g3ts style hooks.

/// `ESLint` config surface ingestion.
mod eslint;
/// Inline `ESLint` directive ingestion across source files.
mod eslint_directives;
/// Package manifest surface ingestion for style-related dependencies and scripts.
mod package;
/// Style policy surface ingestion from `guardrail3-ts.toml`.
mod policy;
/// Helpers for locating style-related files within the workspace crawl.
mod roots;
/// Top-level ingestion entry point.
mod run;
/// Stylelint config surface ingestion.
mod stylelint;
/// Syncpack config surface ingestion for the style policy pin.
mod syncpack;

#[cfg(feature = "api")]
pub use run::ingest_for_config_checks;
