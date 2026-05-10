#![expect(
    clippy::multiple_crate_versions,
    reason = "transitive: hashbrown and siphasher pulled at different versions by upstream parser crates; pinning here would break cross-workspace ingestion contract"
)]

#[cfg(feature = "api")]
/// Internal `astro_config` module.
mod astro_config;
#[cfg(feature = "api")]
/// Internal `eslint` module.
mod eslint;
#[cfg(feature = "api")]
/// Internal `package` module.
mod package;
#[cfg(feature = "api")]
/// Internal `policy` module.
mod policy;
#[cfg(feature = "api")]
/// Internal `roots` module.
mod roots;
#[cfg(feature = "api")]
/// Internal `run` module.
mod run;

#[cfg(feature = "api")]
pub use run::ingest_for_config_checks;

#[cfg(test)]
use g3ts_astro_media_ingestion_assertions as _;
