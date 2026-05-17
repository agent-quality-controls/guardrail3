/// Centralized filesystem port used by the ingestion runtime.
mod fs;
/// Entry point that ingests the workspace crawl into a jscpd checks input.
mod run;

#[cfg(feature = "ingest")]
pub use run::ingest_for_config_checks;
