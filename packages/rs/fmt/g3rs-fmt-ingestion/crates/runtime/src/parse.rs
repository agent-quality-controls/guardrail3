/// Read and parse config files from disk.
use std::path::Path;

use cargo_toml_parser::types::CargoToml;
use rust_toolchain_toml_parser::types::RustToolchainToml;
use rustfmt_toml_parser::types::RustfmtToml;

use crate::run::IngestionError;

/// Typed view of a parsed `rustfmt.toml`, paired with the names of every top-level
/// key that appeared explicitly in the source document.
pub(crate) type ParsedRustfmtToml = (RustfmtToml, Vec<String>);

/// Read the file at `abs_path` and parse it as a `RustfmtToml`.
pub(crate) fn parse_rustfmt_toml(abs_path: &Path) -> Result<ParsedRustfmtToml, IngestionError> {
    let content =
        crate::fs::read_to_string(abs_path).map_err(|err| IngestionError::Unreadable {
            path: abs_path.to_path_buf(),
            reason: err.to_string(),
        })?;
    let parsed =
        rustfmt_toml_parser::parse(&content).map_err(|err| IngestionError::ParseFailed {
            path: abs_path.to_path_buf(),
            reason: err.to_string(),
        })?;
    Ok((parsed, explicit_rustfmt_keys(&content)))
}

/// Implements `explicit rustfmt keys`.
///
/// Returns the names of every top-level key whose value was provided by the source
/// document, distinct from the typed view in `RustfmtToml` which cannot represent
/// the set-vs-default distinction needed by the rustfmt config checks.
#[expect(
    clippy::disallowed_methods,
    reason = "rustfmt config checks need the set-of-explicit-top-level-keys signal that the typed `RustfmtToml` cannot encode; the central rustfmt parser already uses toml::from_str under the same justification, so the duplicate raw-Value parse here is the local fallback until rustfmt-toml-parser exposes an explicit-keys API"
)]
fn explicit_rustfmt_keys(content: &str) -> Vec<String> {
    match toml::from_str::<toml::Value>(content) {
        Ok(toml::Value::Table(table)) => table.keys().cloned().collect(),
        Ok(_) | Err(_) => Vec::new(),
    }
}

/// Read the file at `abs_path` and parse it as a `CargoToml`.
pub(crate) fn parse_cargo_toml(abs_path: &Path) -> Result<CargoToml, IngestionError> {
    let content =
        crate::fs::read_to_string(abs_path).map_err(|err| IngestionError::Unreadable {
            path: abs_path.to_path_buf(),
            reason: err.to_string(),
        })?;
    cargo_toml_parser::parse(&content).map_err(|err| IngestionError::ParseFailed {
        path: abs_path.to_path_buf(),
        reason: err.to_string(),
    })
}

/// Read the file at `abs_path` and parse it as a `RustToolchainToml`.
pub(crate) fn parse_toolchain_toml(abs_path: &Path) -> Result<RustToolchainToml, IngestionError> {
    let content =
        crate::fs::read_to_string(abs_path).map_err(|err| IngestionError::Unreadable {
            path: abs_path.to_path_buf(),
            reason: err.to_string(),
        })?;
    rust_toolchain_toml_parser::parse(&content).map_err(|err| IngestionError::ParseFailed {
        path: abs_path.to_path_buf(),
        reason: err.to_string(),
    })
}
