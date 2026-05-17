/// File-tree facts: presence of declared entrypoints on disk.
mod file_tree;
/// Centralized filesystem access boundary.
mod fs;
/// Manifest parsing for declared facade entrypoints.
mod manifest;
/// Public ingestion entry points.
mod run;
/// Source surface state collection.
mod source;

#[cfg(feature = "ingest")]
pub use run::{
    G3TsArchIngestionError, ingest_for_config_checks, ingest_for_file_tree_checks,
    ingest_for_source_checks,
};
