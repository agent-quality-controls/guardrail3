mod file_tree;
mod fs;
mod manifest;
mod run;
mod source;

#[cfg(feature = "ingest")]
pub use run::{
    G3TsArchIngestionError, ingest_for_config_checks, ingest_for_file_tree_checks,
    ingest_for_source_checks,
};

#[cfg(test)]
use g3ts_arch_ingestion_assertions as _;

#[cfg(test)]
use tempfile as _;
