/// Security advisory model definitions.
#[cfg(feature = "api")]
pub mod advisories;
/// Dependency ban model definitions.
#[cfg(feature = "api")]
pub mod bans;
/// Top-level deny.toml model definitions.
#[cfg(feature = "api")]
pub mod deny_toml;
/// Dependency graph model definitions.
#[cfg(feature = "api")]
pub mod graph;
/// License model definitions.
#[cfg(feature = "api")]
pub mod licenses;
/// Source restriction and output model definitions.
#[cfg(feature = "api")]
pub mod sources;
use toml as _;
