/// Centralized filesystem access boundary.
mod fs;
/// Public ingestion entry points.
mod run;
/// Source surface state collection from a TS workspace crawl.
mod source;

#[cfg(feature = "ingest")]
pub use run::{G3TsApparchIngestionError, ingest_for_config_checks, ingest_for_source_checks};

#[cfg(test)]
use g3ts_apparch_ingestion_assertions as _;

#[cfg(test)]
use tempfile as _;
