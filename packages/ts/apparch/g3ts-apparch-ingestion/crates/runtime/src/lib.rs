mod fs;
mod run;
mod source;

#[cfg(feature = "ingest")]
pub use run::{G3TsApparchIngestionError, ingest_for_config_checks, ingest_for_source_checks};

#[cfg(test)]
use g3ts_apparch_ingestion_assertions as _;

#[cfg(test)]
use tempfile as _;
