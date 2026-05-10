//! Centralized filesystem boundary for the validate-command runtime.

use std::path::{Path, PathBuf};

/// Reads the contents of a file as a string.
///
/// # Errors
///
/// Returns the underlying [`std::io::Error`] when the file cannot be read.
#[allow(
    clippy::disallowed_methods,
    clippy::verbose_file_reads,
    reason = "this module IS the centralized filesystem boundary for validate-command"
)]
pub(crate) fn read_to_string(path: &Path) -> std::io::Result<String> {
    std::fs::read_to_string(path)
}

/// Returns true when the path exists and is a regular file.
#[allow(
    clippy::disallowed_methods,
    reason = "this module IS the centralized filesystem boundary for validate-command"
)]
pub(crate) fn is_file(path: &Path) -> bool {
    path.is_file()
}

/// Returns true when the path exists and is a directory.
#[allow(
    clippy::disallowed_methods,
    reason = "this module IS the centralized filesystem boundary for validate-command"
)]
pub(crate) fn is_dir(path: &Path) -> bool {
    path.is_dir()
}

/// Lists the immediate child entries of a directory. Returns an empty vector
/// when the directory cannot be read.
#[allow(
    clippy::disallowed_methods,
    reason = "this module IS the centralized filesystem boundary for validate-command"
)]
pub(crate) fn read_dir_paths(dir: &Path) -> Vec<PathBuf> {
    let Ok(iter) = std::fs::read_dir(dir) else {
        return Vec::new();
    };
    iter.flatten().map(|entry| entry.path()).collect()
}
