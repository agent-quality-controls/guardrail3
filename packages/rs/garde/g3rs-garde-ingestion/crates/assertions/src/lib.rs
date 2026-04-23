/// Shared helper primitives used by the ingestion proof modules.
mod common;

#[cfg(feature = "ingest")]
pub mod run;
#[cfg(feature = "ingest")]
pub mod run_tests;
