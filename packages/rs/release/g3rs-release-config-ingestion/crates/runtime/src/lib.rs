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

#[cfg(feature = "ingest")]
pub use run::{IngestionError, ingest};

#[cfg(test)]
mod ingest_tests;
