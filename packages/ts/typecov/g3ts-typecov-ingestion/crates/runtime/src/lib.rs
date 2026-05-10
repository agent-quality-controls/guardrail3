/// Internal `package` module.
mod package;
/// Internal `roots` module.
mod roots;
/// Internal `run` module.
mod run;
/// Internal `syncpack` module.
mod syncpack;

#[cfg(feature = "api")]
pub use run::ingest_for_config_checks;
