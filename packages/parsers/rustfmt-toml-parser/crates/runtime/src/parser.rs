use rustfmt_toml_parser_types::rustfmt_toml::RustfmtToml;

use crate::Error;

/// Parse `rustfmt.toml` content into typed data.
///
/// # Errors
///
/// Returns [`Error::Toml`] when the input is not valid `rustfmt.toml`.
#[allow(clippy::disallowed_methods)] // reason: this crate IS the centralized rustfmt.toml parser - toml::from_str is its core purpose
pub fn parse(input: &str) -> Result<RustfmtToml, Error> {
    Ok(toml::from_str(input)?)
}

/// Read and parse a `rustfmt.toml` file from disk.
///
/// # Errors
///
/// Returns [`Error::Io`] on read failure and [`Error::Toml`] on parse failure.
pub fn from_path(path: impl AsRef<std::path::Path>) -> Result<RustfmtToml, Error> {
    let content = crate::fs::read_to_string(path)?;
    parse(&content)
}

#[cfg(test)]
#[path = "parser_tests/mod.rs"] // reason: owned sidecar tests for file module.
mod parser_tests;
