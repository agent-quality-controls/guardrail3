mod activation;
mod fs;
mod hook_shell;
mod hooks;
mod roots;
mod run;

#[cfg(feature = "ingest")]
pub use run::{
    IngestionError, ingest_for_ast_checks, ingest_for_config_checks,
    ingest_for_file_tree_checks,
};

#[cfg(test)]
mod ingest_tests;
