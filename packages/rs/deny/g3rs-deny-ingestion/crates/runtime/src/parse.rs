/// Read and parse config files from disk.
use std::path::Path;

use deny_toml_parser::DenyToml;
use g3rs_deny_types::G3RsDenyRustPolicyState;

use crate::run::IngestionError;

/// Read the file at `abs_path` and parse it as raw TOML.
fn read_to_string(abs_path: &Path) -> Result<String, IngestionError> {
    let content = crate::fs::read_to_string(abs_path).map_err(|_err| IngestionError::Unreadable {
        path: abs_path.to_path_buf(),
        reason: "file is not readable".to_owned(),
    })?;
    Ok(content)
}

/// Read the file at `abs_path` and parse it as a `DenyToml`.
pub(crate) fn parse_deny_toml(abs_path: &Path) -> Result<DenyToml, IngestionError> {
    let content = read_to_string(abs_path)?;
    deny_toml_parser::parse(&content).map_err(|err| IngestionError::ParseFailed {
        path: abs_path.to_path_buf(),
        reason: err.to_string(),
    })
}

pub(crate) fn parse_rust_policy_state(
    rel_path: &str,
    abs_path: &Path,
) -> G3RsDenyRustPolicyState {
    let content = match read_to_string(abs_path) {
        Ok(content) => content,
        Err(IngestionError::Unreadable { reason, .. }) => {
            return G3RsDenyRustPolicyState::Unreadable {
                rel_path: rel_path.to_owned(),
                reason,
            };
        }
        Err(IngestionError::ParseFailed { reason, .. }) => {
            return G3RsDenyRustPolicyState::ParseError {
                rel_path: rel_path.to_owned(),
                reason,
            };
        }
        Err(IngestionError::DenyTomlNotFound | IngestionError::SourceIngestionNotImplemented) => {
            unreachable!("read_to_string cannot return unrelated deny ingestion errors");
        }
    };

    let parsed = match guardrail3_rs_toml_parser::parse(&content) {
        Ok(parsed) => parsed,
        Err(err) => {
            return G3RsDenyRustPolicyState::ParseError {
                rel_path: rel_path.to_owned(),
                reason: err.to_string(),
            };
        }
    };

    G3RsDenyRustPolicyState::Parsed {
        rel_path: rel_path.to_owned(),
        profile: parsed.profile,
    }
}
