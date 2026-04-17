mod activation;
mod components;
mod fs;
mod hook_shell;
mod hooks;
mod roots;
mod ingest;

#[cfg(feature = "ingest")]
pub use ingest::{
    IngestionError, ingest_for_config_checks, ingest_for_file_tree_checks, ingest_for_source_checks,
};
