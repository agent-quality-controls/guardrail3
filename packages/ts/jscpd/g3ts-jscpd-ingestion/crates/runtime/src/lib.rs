mod fs;
mod run;

#[cfg(feature = "ingest")]
pub use run::ingest_for_config_checks;

#[cfg(test)]
use tempfile as _;
