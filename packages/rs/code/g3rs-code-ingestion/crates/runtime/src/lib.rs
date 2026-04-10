mod classify;
mod config;
mod config_comments;
mod config_scope;
mod fs;
mod ingest;
mod run;
mod select;
mod unsafe_code_lints;

#[cfg(feature = "ingest")]
pub use run::{
    IngestionError, ingest_for_source_checks, ingest_for_config_checks,
    ingest_for_file_tree_checks,
};

#[cfg(test)]
mod ingest_tests;
