mod activation;
mod components;
#[cfg(feature = "ingest")]
pub mod fixtures;
mod fs;
mod hooks;
mod ingest;
mod parse;
mod roots;
mod source_analysis;

#[cfg(feature = "ingest")]
pub use ingest::{
    IngestionError, ingest_for_config_checks, ingest_for_file_tree_checks, ingest_for_source_checks,
};
