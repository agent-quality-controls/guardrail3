mod fs;
mod run;
mod view;

#[cfg(feature = "ingest")]
pub use run::{G3RsApparchIngestionError, ingest_for_config_checks, ingest_for_source_checks};

#[cfg(test)]
use g3rs_apparch_ingestion_assertions as _;
