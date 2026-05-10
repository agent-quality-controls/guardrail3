/// `activation` module.
mod activation;
/// `components` module.
mod components;
/// `file_tree_analysis` module.
mod file_tree_analysis;
#[cfg(feature = "ingest")]
pub mod fixtures;
/// `fs` module.
mod fs;
/// `hooks` module.
mod hooks;
/// `ingest` module.
mod ingest;
/// `parse` module.
mod parse;
/// `roots` module.
mod roots;
/// `source_analysis` module.
mod source_analysis;

#[cfg(feature = "ingest")]
pub use ingest::{
    IngestionError, ingest_for_config_checks, ingest_for_file_tree_checks, ingest_for_source_checks,
};
