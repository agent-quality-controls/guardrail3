#[cfg(test)]
pub(super) use crate::Error;
#[cfg(test)]
pub(super) use crate::types::DenyToml;
#[cfg(not(test))]
use crate::types::DenyToml;

/// Parse `deny.toml` content into typed data.
///
/// # Errors
///
/// Returns [`Error::Toml`] when the input is not valid `deny.toml`.
#[allow(
    clippy::disallowed_methods,
    reason = "parser.rs IS the centralized deny.toml parser"
)]
pub fn parse(input: &str) -> Result<DenyToml, crate::error::Error> {
    Ok(toml::from_str(input)?)
}

/// Read and parse a `deny.toml` file from disk.
///
/// # Errors
///
/// Returns [`Error::Io`] on read failure and [`Error::Toml`] on parse failure.
pub fn from_path(path: impl AsRef<std::path::Path>) -> Result<DenyToml, crate::error::Error> {
    let content = crate::fs::read_to_string(path)?;
    parse(&content)
}

#[cfg(test)]
#[path = "parser_tests/mod.rs"] // reason: owned sidecar tests for file module.
mod parser_tests;
