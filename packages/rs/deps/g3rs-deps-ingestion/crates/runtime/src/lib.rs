mod fs;
mod ingest;
mod parse;
mod run;
mod select;

#[cfg(feature = "ingest")]
pub use run::{IngestionError, ingest_for_source_checks, ingest_for_config_checks, ingest_for_file_tree_checks};

#[cfg(test)]
use g3rs_deps_ingestion_assertions as _;
#[cfg(test)]
mod ingest_tests;
