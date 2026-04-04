use clippy_toml_types::ClippyToml;

use crate::Error;

/// Parse `clippy.toml` content into typed data.
///
/// # Errors
///
/// Returns [`Error::Toml`] when the input is not valid `clippy.toml`.
#[allow(clippy::disallowed_methods)] // reason: this crate IS the centralized clippy.toml parser — toml::from_str is its core purpose
pub fn parse(input: &str) -> Result<ClippyToml, Error> {
    Ok(toml::from_str(input)?)
}

/// Read and parse a `clippy.toml` file from disk.
///
/// # Errors
///
/// Returns [`Error::Io`] on read failure and [`Error::Toml`] on parse failure.
pub fn from_path(path: impl AsRef<std::path::Path>) -> Result<ClippyToml, Error> {
    let content = crate::fs::read_to_string(path)?;
    parse(&content)
}

#[cfg(test)]
#[path = "parser_tests/mod.rs"]
mod tests;
