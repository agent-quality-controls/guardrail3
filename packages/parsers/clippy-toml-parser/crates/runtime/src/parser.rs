pub(super) use crate::types::ClippyToml;
#[cfg(test)]
pub(super) use crate::types::{
    InherentImplLintScope, MatchLintBehaviour, PubUnderscoreFieldsBehaviour,
    SourceItemOrderingWithinModuleItemGroupings,
};

/// Parse `clippy.toml` content into typed data.
///
/// # Errors
///
/// Returns [`Error::Toml`] when the input is not valid `clippy.toml`.
#[allow(
    clippy::disallowed_methods,
    reason = "this crate IS the centralized clippy.toml parser"
)]
pub fn parse(input: &str) -> Result<ClippyToml, crate::error::Error> {
    Ok(toml::from_str(input)?)
}

/// Read and parse a `clippy.toml` file from disk.
///
/// # Errors
///
/// Returns [`Error::Io`] on read failure and [`Error::Toml`] on parse failure.
pub fn from_path(path: impl AsRef<std::path::Path>) -> Result<ClippyToml, crate::error::Error> {
    let content = crate::fs::read_to_string(path)?;
    parse(&content)
}

#[cfg(test)]
#[path = "parser_tests/mod.rs"] // reason: owned sidecar tests for file module.
mod parser_tests;
