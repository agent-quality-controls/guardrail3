use release_plz_toml_parser_types::release_plz_toml::ReleasePlzToml;

use crate::Error;

/// Parse `release-plz.toml` content into typed data.
///
/// # Errors
///
/// Returns [`Error::Toml`] when the input is not valid `release-plz.toml`.
#[allow(
    clippy::disallowed_methods,
    reason = "this crate IS the centralized release-plz.toml parser - toml::from_str is its core purpose"
)]
pub fn parse(input: &str) -> Result<ReleasePlzToml, Error> {
    Ok(toml::from_str(input)?)
}

/// Read and parse a `release-plz.toml` file from disk.
///
/// # Errors
///
/// Returns [`Error::Io`] on read failure and [`Error::Toml`] on parse failure.
pub fn from_path(path: impl AsRef<std::path::Path>) -> Result<ReleasePlzToml, Error> {
    let content = crate::fs::read_to_string(path)?;
    parse(&content)
}

#[cfg(test)]
#[path = "parser_tests/mod.rs"] // reason: owned sidecar tests for file module.
mod parser_tests;
