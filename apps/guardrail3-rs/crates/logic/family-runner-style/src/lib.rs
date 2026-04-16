/// Runs the style-oriented family group against one workspace crawl.
mod run;

#[cfg(feature = "api")]
pub use run::run;
