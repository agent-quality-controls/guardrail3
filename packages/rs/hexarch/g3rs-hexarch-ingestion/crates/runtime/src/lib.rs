#[cfg(test)]
use g3rs_hexarch_ingestion_assertions as _;

#[cfg(test)]
mod ingest_tests;
mod run;
mod view;

#[cfg(feature = "ingest")]
pub use run::{
    ingest_for_config_checks, ingest_for_file_tree_checks, ingest_for_source_checks,
};
