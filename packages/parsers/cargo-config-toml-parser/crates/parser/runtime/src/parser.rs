use cargo_config_toml_parser_types::CargoConfigToml;

use crate::Error;

/// Parse `.cargo/config.toml` or `.cargo/config` content into typed data.
///
/// # Errors
///
/// Returns [`Error::Toml`] when the input is not valid cargo config TOML.
#[allow(clippy::disallowed_methods)] // reason: this crate IS the centralized cargo config parser — toml::from_str is its core purpose
pub fn parse(input: &str) -> Result<CargoConfigToml, Error> {
    Ok(toml::from_str(input)?)
}

/// Read and parse a cargo config file from disk.
///
/// # Errors
///
/// Returns [`Error::Io`] on read failure and [`Error::Toml`] on parse failure.
pub fn from_path(path: impl AsRef<std::path::Path>) -> Result<CargoConfigToml, Error> {
    let content = crate::fs::read_to_string(path)?;
    parse(&content)
}

#[cfg(test)]
#[path = "parser_tests/mod.rs"]
mod tests;
