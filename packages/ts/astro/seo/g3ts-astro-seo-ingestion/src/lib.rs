#[cfg(feature = "api")]
mod run;

#[cfg(feature = "api")]
pub use run::ingest_for_config_checks;
