pub(super) use crate::types::{ClippyToml, ClippyTomlDocument, ClippyTomlParseState};

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

/// Parses `clippy.toml` content into a [`ClippyTomlDocument`] capturing both raw and typed views.
///
/// # Errors
///
/// Returns [`crate::error::Error::Toml`] when the raw TOML cannot be parsed.
#[allow(
    clippy::disallowed_methods,
    reason = "parser.rs IS the centralized clippy.toml parser"
)]
pub fn parse_document(input: &str) -> Result<ClippyTomlDocument, crate::error::Error> {
    let raw = toml::from_str(input)?;
    let typed = match parse(input) {
        Ok(clippy) => ClippyTomlParseState::Parsed(clippy),
        Err(err) => ClippyTomlParseState::Invalid(err.to_string()),
    };
    Ok(ClippyTomlDocument { raw, typed })
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
