//! Runtime rules for the `g3rs-garde-ingestion` family.

/// Filesystem helpers used by ingestion and rule code.
mod fs;
/// Ingestion code that constructs the family's input bundle.
mod ingest;
/// Parser helpers for the family's structured inputs.
mod parse;
/// Family entry point that runs all rules.
mod run;
/// Crate selection helpers used by the family's ingestion stage.
mod select;
/// Rule implementation for `source analysis`.
mod source_analysis;

#[cfg(feature = "ingest")]
pub use run::{
    IngestionError, ingest_for_config_checks, ingest_for_file_tree_checks, ingest_for_source_checks,
};
