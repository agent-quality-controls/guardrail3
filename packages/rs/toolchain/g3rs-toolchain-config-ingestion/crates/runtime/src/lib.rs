/// Centralized filesystem access boundary.
mod fs;
/// Assemble check inputs from parsed data.
mod ingest;
/// Read and parse config files.
mod parse;
/// Public ingestion entry point.
mod run;

#[cfg(feature = "ingest")]
pub use run::{IngestionError, ingest};

#[cfg(test)]
mod ingest_tests;
