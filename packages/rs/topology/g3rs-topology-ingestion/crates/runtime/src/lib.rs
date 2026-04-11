mod run;
mod view;

#[cfg(feature = "ingest")]
pub use run::{ingest_for_config_checks, ingest_for_file_tree_checks, ingest_for_source_checks};

#[cfg(test)]
mod ingest_tests;
