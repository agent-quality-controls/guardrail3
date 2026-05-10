/// Internal module `fs`.
mod fs;
/// Internal module `process`.
mod process;
/// Internal module `run`.
mod run;

#[cfg(feature = "api")]
pub use run::{
    ingest_for_config_checks, ingest_for_config_checks_with_path, ingest_for_file_tree_checks,
    ingest_for_source_checks,
};
