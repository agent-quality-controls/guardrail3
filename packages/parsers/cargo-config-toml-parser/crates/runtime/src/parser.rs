use cargo_config_toml_parser_types::cargo_config_toml::CargoConfigToml;

/// Parse `.cargo/config.toml` or `.cargo/config` content into typed data.
///
/// # Errors
///
/// Returns [`crate::error::Error::Toml`] when the input is not valid cargo
/// config TOML.
#[allow(
    clippy::disallowed_methods,
    reason = "parser.rs IS the centralized cargo config parser"
)]
pub fn parse(input: &str) -> Result<CargoConfigToml, crate::error::Error> {
    Ok(toml::from_str(input)?)
}

/// Read and parse a cargo config file from disk.
///
/// # Errors
///
/// Returns [`crate::error::Error::Io`] on read failure and
/// [`crate::error::Error::Toml`] on parse failure.
pub fn from_path(path: impl AsRef<std::path::Path>) -> Result<CargoConfigToml, crate::error::Error> {
    let content = crate::fs::read_to_string(path)?;
    parse(&content)
}

#[cfg(test)]
#[path = "parser_tests/mod.rs"] // reason: owned sidecar tests for file module.
mod parser_tests;
