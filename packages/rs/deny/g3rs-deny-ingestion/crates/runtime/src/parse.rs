/// Read and parse config files from disk.
use std::path::Path;

use deny_toml_parser::types::DenyToml;
use g3rs_deny_types::G3RsDenyRustPolicyState;

use crate::run::IngestionError;

fn read_to_string(abs_path: &Path) -> Result<String, String> {
    crate::fs::read_to_string(abs_path).map_err(|_err| "file is not readable".to_owned())
}

/// Read the file at `abs_path` and parse it as a `DenyToml`.
pub(crate) fn parse_deny_toml(abs_path: &Path) -> Result<DenyToml, IngestionError> {
    let content = read_to_string(abs_path).map_err(|reason| IngestionError::Unreadable {
        path: abs_path.to_path_buf(),
        reason,
    })?;
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
        Err(reason) => {
            return G3RsDenyRustPolicyState::Unreadable {
                rel_path: rel_path.to_owned(),
                reason,
            };
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
