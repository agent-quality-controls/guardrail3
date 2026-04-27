mod classify;
mod config_files;
mod config_scope;
mod fs;
mod ingest;
mod run;
mod select;

#[cfg(feature = "ingest")]
pub use run::{
    IngestionError, ingest_for_config_checks, ingest_for_file_tree_checks, ingest_for_source_checks,
};
