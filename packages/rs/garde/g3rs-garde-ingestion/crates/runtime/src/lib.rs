mod fs;
mod ingest;
mod parse;
mod run;
mod select;
mod source_analysis;

#[cfg(feature = "ingest")]
pub use run::{
    IngestionError, ingest_for_config_checks, ingest_for_file_tree_checks, ingest_for_source_checks,
};
