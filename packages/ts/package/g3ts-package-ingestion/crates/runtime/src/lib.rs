/// Entry point that ingests the workspace crawl into a package checks input.
mod run;

#[cfg(feature = "ingest")]
pub use run::ingest_for_config_checks;

#[cfg(test)]
use tempfile as _;
