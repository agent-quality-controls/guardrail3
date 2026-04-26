mod run;
mod select;

#[cfg(test)]
use g3ts_astro_ingestion_assertions as _;

#[cfg(feature = "ingest")]
pub use run::ingest_for_config_checks;
#[cfg(feature = "ingest")]
pub use run::ingest_for_file_tree_checks;
