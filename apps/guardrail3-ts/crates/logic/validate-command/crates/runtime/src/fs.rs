//! Centralized filesystem boundary for the validate-command runtime.

use std::path::Path;

/// Creates a directory and all missing parents.
#[allow(
    clippy::disallowed_methods,
    reason = "this module IS the centralized filesystem boundary for validate-command"
)]
pub(crate) fn create_dir_all(path: &Path) -> std::io::Result<()> {
    std::fs::create_dir_all(path)
}

/// Reads the contents of a file as a string.
#[allow(
    clippy::disallowed_methods,
    clippy::verbose_file_reads,
    reason = "this module IS the centralized filesystem boundary for validate-command"
)]
pub(crate) fn read_to_string(path: &Path) -> std::io::Result<String> {
    std::fs::read_to_string(path)
}

/// Writes a full file.
#[allow(
    clippy::disallowed_methods,
    reason = "this module IS the centralized filesystem boundary for validate-command"
)]
pub(crate) fn write(path: &Path, content: &str) -> std::io::Result<()> {
    std::fs::write(path, content)
}

/// Marks a file executable on Unix.
#[cfg(unix)]
#[allow(
    clippy::disallowed_methods,
    reason = "this module IS the centralized filesystem boundary for validate-command"
)]
pub(crate) fn make_executable(path: &Path) {
    use std::os::unix::fs::PermissionsExt;

    if let Ok(metadata) = std::fs::metadata(path) {
        let mut permissions = metadata.permissions();
        permissions.set_mode(0o755);
        let _ = std::fs::set_permissions(path, permissions);
    }
}

/// No-op executable marker for non-Unix platforms.
#[cfg(not(unix))]
pub(crate) fn make_executable(_path: &Path) {}
