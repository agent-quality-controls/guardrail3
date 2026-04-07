mod fs;
mod ingest;
mod parse;
mod run;
mod select;

#[cfg(feature = "ingest")]
pub use run::{IngestionError, ingest_ast, ingest_config, ingest_file_tree};

#[cfg(test)]
use g3rs_deps_config_ingestion_assertions as _;
#[cfg(test)]
mod ingest_tests;
