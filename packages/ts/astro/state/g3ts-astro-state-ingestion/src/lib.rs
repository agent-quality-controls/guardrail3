/// Astro state policy detection.
#[cfg(feature = "api")]
mod policy;
/// Astro app root discovery from a workspace crawl.
#[cfg(feature = "api")]
mod roots;
/// Top-level ingestion entry point.
#[cfg(feature = "api")]
mod run;

#[cfg(feature = "api")]
pub use run::ingest_for_file_tree_checks;
