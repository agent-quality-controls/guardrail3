#![expect(
    clippy::multiple_crate_versions,
    reason = "transitive: hashbrown and siphasher pulled at different versions by upstream parser crates (swc, tree-sitter); pinning here would break cross-workspace ingestion contract"
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
/// Internal `roots` module.
mod roots;
#[cfg(feature = "api")]
/// Internal `run` module.
mod run;
#[cfg(feature = "api")]
/// Internal `syncpack` module.
mod syncpack;

#[cfg(feature = "api")]
pub use run::{ingest_for_config_checks, ingest_for_file_tree_checks};
