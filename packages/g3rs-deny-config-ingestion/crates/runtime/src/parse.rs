/// Read and parse a deny config file from disk.
use std::path::Path;

use deny_toml_parser::DenyToml;

use crate::run::IngestionError;

/// Read the file at `abs_path` and parse it as a `DenyToml`.
pub(crate) fn parse_deny_toml(abs_path: &Path) -> Result<DenyToml, IngestionError> {
    let content = crate::fs::read_to_string(abs_path).map_err(|err| IngestionError::Unreadable {
        path: abs_path.to_path_buf(),
        reason: err.to_string(),
    })?;
    deny_toml_parser::parse(&content).map_err(|err| IngestionError::ParseFailed {
        path: abs_path.to_path_buf(),
        reason: err.to_string(),
    })
}
