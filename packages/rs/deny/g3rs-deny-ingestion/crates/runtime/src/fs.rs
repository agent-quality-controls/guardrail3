/// Centralized filesystem access for the ingestion runtime.
use std::path::Path;

/// Read a file's contents as a UTF-8 string.
#[expect(
    clippy::disallowed_methods,
    reason = "this module is the centralized wrapper around `std::fs::read_to_string`"
)]
pub(crate) fn read_to_string(path: &Path) -> std::io::Result<String> {
    std::fs::read_to_string(path)
}
