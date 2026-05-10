#![expect(
    clippy::multiple_crate_versions,
    reason = "Transitive dependencies on serde_json and tree-sitter pull in different \
              `hashbrown` and `siphasher` versions; resolving the duplication requires \
              coordinated upgrades in upstream crates that are outside this workspace's \
              control"
)]

/// Ingest the eslint configuration surface for content rules.
#[cfg(feature = "api")]
mod eslint;
/// Ingest disable directives carried by eslint configuration files.
#[cfg(feature = "api")]
mod eslint_directives;
/// Ingest the surface of the workspace `package.json` manifest.
#[cfg(feature = "api")]
mod package;
/// Ingest the project-level content policy configuration.
#[cfg(feature = "api")]
mod policy;
/// Discover the workspace roots that participate in content checks.
#[cfg(feature = "api")]
mod roots;
/// Top-level ingestion entry points for the content family.
#[cfg(feature = "api")]
mod run;
/// Ingest the source-file inventory used by content rules.
#[cfg(feature = "api")]
mod sources;

#[cfg(feature = "api")]
pub use run::{ingest_for_config_checks, ingest_for_file_tree_checks};
