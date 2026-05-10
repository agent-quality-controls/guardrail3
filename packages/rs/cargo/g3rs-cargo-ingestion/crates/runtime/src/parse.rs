/// Read and parse a `Cargo.toml` file from disk.
use std::path::Path;

use cargo_toml_parser::types::CargoTomlDocument;
use g3rs_cargo_types::{G3RsCargoRustPolicyState, G3RsCargoWaiver};

use crate::run::IngestionError;

/// Read the file at `abs_path` and parse it as a `Cargo.toml` document.
pub(crate) fn parse_root_cargo_toml(abs_path: &Path) -> Result<CargoTomlDocument, IngestionError> {
    let content =
        crate::fs::read_to_string(abs_path).map_err(|err| IngestionError::Unreadable {
            path: abs_path.to_path_buf(),
            reason: err.to_string(),
        })?;
    let document =
        cargo_toml_parser::parse_document(&content).map_err(|err| IngestionError::ParseFailed {
            path: abs_path.to_path_buf(),
            reason: err.to_string(),
        })?;
    if let Some(reason) = cargo_toml_parser::document::parse_error_reason(&document) {
        return Err(IngestionError::ParseFailed {
            path: abs_path.to_path_buf(),
            reason: reason.to_owned(),
        });
    }
    Ok(document)
}

/// parse member cargo toml fn.
pub(crate) fn parse_member_cargo_toml(
    abs_path: &Path,
) -> Result<CargoTomlDocument, IngestionError> {
    let content =
        crate::fs::read_to_string(abs_path).map_err(|err| IngestionError::Unreadable {
            path: abs_path.to_path_buf(),
            reason: err.to_string(),
        })?;
    cargo_toml_parser::parse_document(&content).map_err(|err| IngestionError::ParseFailed {
        path: abs_path.to_path_buf(),
        reason: err.to_string(),
    })
}

/// Parse a TOML file into an untyped [`toml::Value`] tree.
///
/// # Errors
///
/// Returns [`IngestionError::Unreadable`] when the file cannot be read, or
/// [`IngestionError::ParseFailed`] when the contents are not valid TOML.
#[expect(
    clippy::disallowed_methods,
    reason = "the toml::from_str ban exists to force typed deserialization through Validated<T>::new(); this call deserializes into untyped toml::Value to inspect raw workspace section shape across heterogeneous Cargo.toml files, which has no garde-validatable schema"
)]
pub(crate) fn parse_raw_toml(abs_path: &Path) -> Result<toml::Value, IngestionError> {
    let content =
        crate::fs::read_to_string(abs_path).map_err(|err| IngestionError::Unreadable {
            path: abs_path.to_path_buf(),
            reason: err.to_string(),
        })?;
    toml::from_str(&content).map_err(|err| IngestionError::ParseFailed {
        path: abs_path.to_path_buf(),
        reason: err.to_string(),
    })
}

/// parse rust policy state fn.
pub(crate) fn parse_rust_policy_state(rel_path: &str, abs_path: &Path) -> G3RsCargoRustPolicyState {
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
