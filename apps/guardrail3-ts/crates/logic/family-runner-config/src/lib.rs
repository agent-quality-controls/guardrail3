/// Runs the config-oriented family group against one workspace crawl.
mod run;

#[cfg(feature = "api")]
pub use run::run;
