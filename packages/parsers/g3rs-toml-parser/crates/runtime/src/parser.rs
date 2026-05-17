use crate::Error;
use crate::types::Guardrail3RsToml;

/// Parse `guardrail3-rs.toml` content into typed data.
///
/// # Errors
///
/// Returns [`Error::Toml`] when the input is not valid `guardrail3-rs.toml`.
#[allow(
    clippy::disallowed_methods,
    reason = "parser.rs IS the centralized guardrail3-rs.toml parser"
)]
pub fn parse(input: &str) -> Result<Guardrail3RsToml, Error> {
    Ok(toml::from_str(input)?)
}

/// Read and parse a `guardrail3-rs.toml` file from disk.
///
/// # Errors
///
/// Returns [`Error::Io`] on read failure and [`Error::Toml`] on parse failure.
pub fn from_path(path: impl AsRef<std::path::Path>) -> Result<Guardrail3RsToml, Error> {
    let content = crate::fs::read_to_string(path)?;
    parse(&content)
}
