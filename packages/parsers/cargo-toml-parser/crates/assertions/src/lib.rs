use cargo_toml_parser_runtime as _;

#[cfg(feature = "checks")]
pub mod parser;
#[cfg(feature = "checks")]
mod parser_deps;
#[cfg(feature = "checks")]
mod parser_manifest;
#[cfg(feature = "checks")]
mod parser_realistic;
