/// Read and parse a clippy config file from disk.
use std::path::Path;

use clippy_toml_parser::ClippyToml;

use crate::run::IngestionError;

/// Read the file at `abs_path` and parse it as a `ClippyToml`.
pub(crate) fn parse_clippy_toml(abs_path: &Path) -> Result<ClippyToml, IngestionError> {
    let content = crate::fs::read_to_string(abs_path).map_err(|err| {
        IngestionError::Unreadable {
            path: abs_path.to_path_buf(),
            reason: err.to_string(),
        }
    })?;
    clippy_toml_parser::parse(&content).map_err(|err| IngestionError::ParseFailed {
        path: abs_path.to_path_buf(),
        reason: err.to_string(),
    })
}
