/// Centralized filesystem access for the ingestion runtime.
use std::path::Path;

/// Read a file's contents as a UTF-8 string.
pub(crate) fn read_to_string(path: &Path) -> std::io::Result<String> {
    std::fs::read_to_string(path)
}

/// Read a directory iterator.
pub(crate) fn read_dir(path: &Path) -> std::io::Result<std::fs::ReadDir> {
    std::fs::read_dir(path)
}
