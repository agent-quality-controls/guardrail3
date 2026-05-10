use std::path::Path;

use cargo_toml_parser::types::CargoToml;
use cliff_toml_parser::types::CliffToml;
use release_plz_toml_parser::types::ReleasePlzToml;

use crate::run::IngestionError;

/// `read_to_string` function.
pub(crate) fn read_to_string(abs_path: &Path) -> Result<String, IngestionError> {
    crate::fs::read_to_string(abs_path).map_err(|err| IngestionError::Unreadable {
        path: abs_path.to_path_buf(),
        reason: err.to_string(),
    })
}

/// `parse_cargo_toml` function.
pub(crate) fn parse_cargo_toml(
    content: &str,
    abs_path: &Path,
) -> Result<CargoToml, IngestionError> {
    cargo_toml_parser::parse(content).map_err(|err| IngestionError::ParseFailed {
        path: abs_path.to_path_buf(),
        reason: err.to_string(),
    })
}

/// `parse_release_plz_toml` function.
pub(crate) fn parse_release_plz_toml(
    content: &str,
    abs_path: &Path,
) -> Result<ReleasePlzToml, IngestionError> {
    release_plz_toml_parser::parse(content).map_err(|err| IngestionError::ParseFailed {
        path: abs_path.to_path_buf(),
        reason: err.to_string(),
    })
}

/// `parse_cliff_toml` function.
pub(crate) fn parse_cliff_toml(
    content: &str,
    abs_path: &Path,
) -> Result<CliffToml, IngestionError> {
    cliff_toml_parser::parse(content).map_err(|err| IngestionError::ParseFailed {
        path: abs_path.to_path_buf(),
        reason: err.to_string(),
    })
}

/// `parse_workflow_yaml` function.
#[expect(
    clippy::disallowed_methods,
    reason = "parse.rs is the centralized YAML parsing boundary for release workflows"
)]
pub(crate) fn parse_workflow_yaml(
    content: &str,
    abs_path: &Path,
) -> Result<serde_yaml::Value, IngestionError> {
    serde_yaml::from_str(content).map_err(|err| IngestionError::ParseFailed {
        path: abs_path.to_path_buf(),
        reason: err.to_string(),
    })
}
