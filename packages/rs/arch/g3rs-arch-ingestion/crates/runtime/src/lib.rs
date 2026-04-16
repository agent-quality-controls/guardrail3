#[cfg(test)]
use g3rs_arch_ingestion_assertions as _;
#[cfg(test)]
use guardrail3_check_types as _;

mod config;
mod error;
mod file_tree;
mod fs;
mod run;
mod source;
mod view;
mod workspace;

#[cfg(feature = "ingest")]
pub use run::{
    G3RsArchIngestionError, ingest_for_config_checks, ingest_for_file_tree_checks,
    ingest_for_source_checks,
};
