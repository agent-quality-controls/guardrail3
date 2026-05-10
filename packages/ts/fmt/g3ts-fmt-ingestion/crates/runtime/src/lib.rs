/// Prettier configuration ingestion.
mod config;
/// Per-package script and metadata ingestion.
mod package;
/// fmt scope (root) discovery.
mod roots;
/// Top-level ingestion entry point.
mod run;
/// Syncpack version-group ingestion.
mod syncpack;

#[cfg(feature = "api")]
pub use run::ingest_for_config_checks;
