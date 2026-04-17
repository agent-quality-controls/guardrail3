/// Read and parse config files from disk.
use std::path::Path;

use cargo_toml_parser::types::CargoToml;
use guardrail3_rs_toml_parser::types::Guardrail3RsToml;

use crate::run::IngestionError;

/// Parsed workspace `guardrail3-rs.toml` plus presence-only policy details.
pub(crate) struct ParsedGuardrail3RsToml {
    /// Typed parsed config.
    pub(crate) config: Guardrail3RsToml,
    /// Whether `allowed_deps` was explicitly present in the source file.
    pub(crate) allowlist_present: bool,
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

/// Read the file at `abs_path` and parse it as a `Guardrail3RsToml`.
pub(crate) fn parse_guardrail3_rs_toml(
    abs_path: &Path,
) -> Result<ParsedGuardrail3RsToml, IngestionError> {
    let content =
        crate::fs::read_to_string(abs_path).map_err(|err| IngestionError::Unreadable {
            path: abs_path.to_path_buf(),
            reason: err.to_string(),
        })?;
    let config =
        guardrail3_rs_toml_parser::parse(&content).map_err(|err| IngestionError::ParseFailed {
            path: abs_path.to_path_buf(),
            reason: err.to_string(),
        })?;
    let raw =
        toml::from_str::<toml::Value>(&content).map_err(|err| IngestionError::ParseFailed {
            path: abs_path.to_path_buf(),
            reason: err.to_string(),
        })?;
    let allowlist_present = raw
        .as_table()
        .is_some_and(|table| table.contains_key("allowed_deps"));

    Ok(ParsedGuardrail3RsToml {
        config,
        allowlist_present,
    })
}
