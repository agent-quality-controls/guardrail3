/// `fs` module.
mod fs;
/// `run` module.
mod run;
/// `upward` module.
mod upward;

#[cfg(test)]
use g3rs_clippy_config_checks as _;
#[cfg(test)]
use g3rs_clippy_ingestion as _;
#[cfg(test)]
use g3rs_code_ingestion as _;
#[cfg(test)]
use g3rs_code_source_checks as _;
#[cfg(test)]
use g3rs_hooks_config_checks as _;
#[cfg(test)]
use g3rs_hooks_file_tree_checks as _;
#[cfg(test)]
use g3rs_hooks_ingestion_assertions as _;
#[cfg(test)]
use g3rs_hooks_source_checks as _;
#[cfg(test)]
use guardrail3_check_types as _;

#[cfg(feature = "ingest")]
pub use run::{ingest_for_config_checks, ingest_for_file_tree_checks, ingest_for_source_checks};
