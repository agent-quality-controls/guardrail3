/// `ESLint` config surface ingestion for the i18n ingestion crate.
#[cfg(feature = "api")]
mod eslint;
/// `package.json` surface ingestion for the i18n ingestion crate.
#[cfg(feature = "api")]
mod package;
/// `guardrail3-ts.toml` i18n policy ingestion for the i18n ingestion crate.
#[cfg(feature = "api")]
mod policy;
/// Astro app root discovery for the i18n ingestion crate.
#[cfg(feature = "api")]
mod roots;
/// Top-level orchestration of i18n config checks ingestion.
#[cfg(feature = "api")]
mod run;

#[cfg(feature = "api")]
pub use run::ingest_for_config_checks;
