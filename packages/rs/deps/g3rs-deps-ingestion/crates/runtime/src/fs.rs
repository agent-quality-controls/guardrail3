/// Centralized filesystem access for the ingestion runtime.
use std::path::Path;

/// Read a file's contents as a UTF-8 string.
#[allow(
    clippy::disallowed_methods,
    reason = "fs.rs IS the centralized fs boundary"
)]
pub(crate) fn read_to_string(path: &Path) -> std::io::Result<String> {
    std::fs::read_to_string(path)
}

#[allow(
    clippy::disallowed_methods,
    reason = "fs.rs IS the centralized fs boundary"
)]
pub(crate) fn metadata(path: &Path) -> std::io::Result<std::fs::Metadata> {
    std::fs::metadata(path)
}
