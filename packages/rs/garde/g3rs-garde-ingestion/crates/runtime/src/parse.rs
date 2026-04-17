/// Read and parse config files from disk.
use std::path::Path;

use cargo_toml_parser::types::CargoToml;
use clippy_toml_parser::types::ClippyToml;

use crate::run::IngestionError;

/// Read the file at `abs_path` and parse it as a `CargoToml`.
pub(crate) fn parse_cargo_toml(abs_path: &Path) -> Result<CargoToml, IngestionError> {
    let content = crate::fs::read_to_string(abs_path).map_err(|err| IngestionError::Unreadable {
        path: abs_path.to_path_buf(),
        reason: err.to_string(),
    })?;
    cargo_toml_parser::parse(&content).map_err(|err| IngestionError::ParseFailed {
        path: abs_path.to_path_buf(),
        reason: err.to_string(),
    })
}

/// Read the file at `abs_path` and parse it as a `ClippyToml`.
pub(crate) fn parse_clippy_toml(abs_path: &Path) -> Result<ClippyToml, IngestionError> {
    let content = crate::fs::read_to_string(abs_path).map_err(|err| IngestionError::Unreadable {
        path: abs_path.to_path_buf(),
        reason: err.to_string(),
    })?;
    clippy_toml_parser::parse(&content).map_err(|err| IngestionError::ParseFailed {
        path: abs_path.to_path_buf(),
        reason: err.to_string(),
    })
}
