/// Centralized filesystem access boundary.
mod fs;
/// Assemble check inputs from parsed data.
mod ingest;
/// Read and parse config files.
mod parse;
/// Public ingestion entry point.
mod run;
/// Select config entries from a workspace crawl.
mod select;
mod workflow;

#[cfg(feature = "ingest")]
pub use run::{
    IngestionError, ingest_for_config_checks, ingest_for_file_tree_checks,
    ingest_for_source_checks,
};

#[cfg(test)]
use g3rs_release_ingestion_assertions as _;
#[cfg(test)]
mod ingest_tests;
