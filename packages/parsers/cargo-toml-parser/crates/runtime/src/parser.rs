use cargo_toml_parser_types::cargo_toml::CargoToml;
use cargo_toml_parser_types::document::{CargoTomlDocument, CargoTomlParseState};

/// Parse `Cargo.toml` content into typed data.
///
/// # Errors
///
/// Returns [`crate::error::Error::Toml`] when the input is not valid Cargo.toml.
#[allow(
    clippy::disallowed_methods,
    reason = "parser.rs IS the centralized Cargo.toml parser"
)]
pub fn parse(input: &str) -> Result<CargoToml, crate::error::Error> {
    Ok(toml::from_str(input)?)
}

/// Parses `input` into a `CargoTomlDocument`, capturing typed-parse failures as `Invalid`.
///
/// # Errors
/// Returns [`crate::error::Error::Toml`] when the raw TOML structure cannot be parsed.
#[allow(
    clippy::disallowed_methods,
    reason = "parser.rs IS the centralized Cargo.toml parser"
)]
pub fn parse_document(input: &str) -> Result<CargoTomlDocument, crate::error::Error> {
    let raw = toml::from_str(input)?;
    let typed = match parse(input) {
        Ok(cargo) => CargoTomlParseState::Parsed(Box::new(cargo)),
        Err(err) => CargoTomlParseState::Invalid(err.to_string()),
    };
    Ok(CargoTomlDocument { raw, typed })
}

/// Read and parse a Cargo.toml file from disk.
///
/// # Errors
///
/// Returns [`crate::error::Error::Io`] on read failure and
/// [`crate::error::Error::Toml`] on parse failure.
pub fn from_path(path: impl AsRef<std::path::Path>) -> Result<CargoToml, crate::error::Error> {
    let content = crate::fs::read_to_string(path)?;
    parse(&content)
}
