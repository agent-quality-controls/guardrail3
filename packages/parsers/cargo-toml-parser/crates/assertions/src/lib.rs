use cargo_toml_parser_runtime as _;

#[cfg(feature = "checks")]
pub mod parser;
/// Realistic-fixture deep assertions for Cargo.toml dependency tables.
#[cfg(feature = "checks")]
pub mod parser_deps;
/// Realistic-fixture deep assertions for Cargo.toml package and workspace tables.
#[cfg(feature = "checks")]
pub mod parser_manifest;
/// Top-level realistic-manifest assertion that orchestrates the per-table helpers.
#[cfg(feature = "checks")]
pub mod parser_realistic;
