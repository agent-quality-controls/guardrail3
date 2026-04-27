/// Centralized filesystem access boundary.
mod fs;
/// Assemble check inputs from parsed data.
mod ingest;
/// Read and parse config files.
mod parse;
/// Public ingestion entry point.
mod run;

#[cfg(test)]
use guardrail3_check_types as _;

#[cfg(feature = "ingest")]
pub use run::{
    IngestionError, ingest_for_config_checks, ingest_for_file_tree_checks, ingest_for_source_checks,
};
