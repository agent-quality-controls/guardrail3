/// Read and parse a `Cargo.toml` file from disk.
use std::path::Path;

use cargo_toml_parser::types::CargoToml;
use g3rs_cargo_types::{G3RsCargoRustPolicyState, G3RsCargoWaiver};

use crate::run::IngestionError;

/// Read the file at `abs_path` and parse it as a `CargoToml`.
pub(crate) fn parse_cargo_toml(abs_path: &Path) -> Result<CargoToml, IngestionError> {
    let content = crate::fs::read_to_string(abs_path).map_err(|err| {
        IngestionError::Unreadable {
            path: abs_path.to_path_buf(),
            reason: err.to_string(),
        }
    })?;
    cargo_toml_parser::parse(&content).map_err(|err| IngestionError::ParseFailed {
        path: abs_path.to_path_buf(),
        reason: err.to_string(),
    })
}

pub(crate) fn parse_raw_toml(abs_path: &Path) -> Result<toml::Value, IngestionError> {
    let content = crate::fs::read_to_string(abs_path).map_err(|err| {
        IngestionError::Unreadable {
            path: abs_path.to_path_buf(),
            reason: err.to_string(),
        }
    })?;
    toml::from_str(&content).map_err(|err| IngestionError::ParseFailed {
        path: abs_path.to_path_buf(),
        reason: err.to_string(),
    })
}

pub(crate) fn parse_rust_policy_state(
    rel_path: &str,
    abs_path: &Path,
) -> G3RsCargoRustPolicyState {
    let content = match crate::fs::read_to_string(abs_path) {
        Ok(content) => content,
        Err(err) => {
            return G3RsCargoRustPolicyState::Unreadable {
                rel_path: rel_path.to_owned(),
                reason: err.to_string(),
            };
        }
    };

    let parsed = match guardrail3_rs_toml_parser::parse(&content) {
        Ok(parsed) => parsed,
        Err(err) => {
            return G3RsCargoRustPolicyState::ParseError {
                rel_path: rel_path.to_owned(),
                reason: err.to_string(),
            };
        }
    };

    G3RsCargoRustPolicyState::Parsed {
        rel_path: rel_path.to_owned(),
        profile: parsed.profile,
        waivers: parsed
            .waivers
            .into_iter()
            .map(|waiver| G3RsCargoWaiver {
                rule: waiver.rule,
                file: waiver.file,
                selector: waiver.selector,
                reason: waiver.reason,
            })
            .collect(),
    }
}
