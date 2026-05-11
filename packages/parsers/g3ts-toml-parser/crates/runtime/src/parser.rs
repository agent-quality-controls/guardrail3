#[cfg(test)]
pub(super) use crate::Error;
#[cfg(not(test))]
use crate::Error;
#[cfg(test)]
pub(super) use crate::types::Guardrail3TsToml;
#[cfg(not(test))]
use crate::types::Guardrail3TsToml;

/// Parse `guardrail3-ts.toml` content into typed data.
///
/// # Errors
///
/// Returns [`Error::Toml`] when the input is not valid `guardrail3-ts.toml`.
#[allow(
    clippy::disallowed_methods,
    reason = "parser.rs IS the centralized guardrail3-ts.toml parser"
)]
pub fn parse(input: &str) -> Result<Guardrail3TsToml, Error> {
    Ok(toml::from_str(input)?)
}

/// Read and parse a `guardrail3-ts.toml` file from disk.
///
/// # Errors
///
/// Returns [`Error::Io`] on read failure and [`Error::Toml`] on parse failure.
pub fn from_path(path: impl AsRef<std::path::Path>) -> Result<Guardrail3TsToml, Error> {
    let content = crate::fs::read_to_string(path)?;
    parse(&content)
}

#[cfg(test)]
#[path = "parser_tests/mod.rs"] // reason: owned sidecar tests for file module.
mod parser_tests;
