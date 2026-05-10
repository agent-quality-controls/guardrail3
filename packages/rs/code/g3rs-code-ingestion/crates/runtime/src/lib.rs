//! Runtime rules for the `g3rs-code-ingestion` family.

/// Rule implementation for `classify`.
mod classify;
/// Rule implementation for `config files`.
mod config_files;
/// Rule implementation for `config scope`.
mod config_scope;
/// Filesystem helpers used by ingestion and rule code.
mod fs;
/// Ingestion code that constructs the family's input bundle.
mod ingest;
/// Family entry point that runs all rules.
mod run;
/// Crate selection helpers used by the family's ingestion stage.
mod select;

#[cfg(feature = "ingest")]
pub use run::{
    IngestionError, ingest_for_config_checks, ingest_for_file_tree_checks, ingest_for_source_checks,
};
