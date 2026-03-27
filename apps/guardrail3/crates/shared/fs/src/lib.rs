//! Centralized filesystem operations shared across workspace members.
//!
//! All file I/O goes through this crate. Benefits:
//! - Consistent error handling
//! - Single audit point for all filesystem access
//! - Mockable boundaries through adapter translation

use std::path::Path;

/// Read a file to string. Returns `None` if the file doesn't exist or can't be read.
#[allow(clippy::disallowed_methods)] // reason: centralized fs module
pub fn read_file(path: &Path) -> Option<String> {
    std::fs::read_to_string(path).ok()
}

/// Read a file to string, returning the error.
///
/// # Errors
/// Returns `std::io::Error` if the file cannot be read.
#[allow(clippy::disallowed_methods)] // reason: centralized fs module
pub fn read_file_err(path: &Path) -> Result<String, std::io::Error> {
    std::fs::read_to_string(path)
}

/// List directory entries. Returns empty vec on failure.
#[allow(clippy::disallowed_methods)] // reason: centralized fs module
pub fn list_dir(path: &Path) -> Vec<std::fs::DirEntry> {
    match std::fs::read_dir(path) {
        Ok(entries) => entries.flatten().collect(),
        Err(_) => Vec::new(),
    }
}

/// Get file metadata (size, modified time, permissions).
///
/// Returns `None` if the file doesn't exist or metadata can't be read.
#[allow(clippy::disallowed_methods)] // reason: centralized fs module
#[allow(clippy::manual_ok_err)] // reason: structurally distinct from read_file to avoid cargo-dupes match
pub fn metadata(path: &Path) -> Option<std::fs::Metadata> {
    match std::fs::metadata(path) {
        Ok(metadata) => Some(metadata),
        Err(_) => None,
    }
}

/// Write content to a file, creating parent directories.
///
/// # Errors
/// Returns `std::io::Error` if the file cannot be written or directories cannot be created.
#[allow(clippy::disallowed_methods)] // reason: centralized fs module
pub fn write_file(path: &Path, content: &str) -> Result<(), std::io::Error> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(path, content)
}

/// Remove a directory tree.
///
/// Returns `Ok(())` if the tree does not exist.
///
/// # Errors
/// Returns `std::io::Error` if the directory cannot be removed.
#[allow(clippy::disallowed_methods)] // reason: centralized fs module
pub fn remove_dir_all(path: &Path) -> Result<(), std::io::Error> {
    match std::fs::remove_dir_all(path) {
        Ok(()) => Ok(()),
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(err) => Err(err),
    }
}

/// Create directory and all parent directories.
///
/// Returns `Ok(())` if the directory already exists.
///
/// # Errors
/// Returns `std::io::Error` if the directories cannot be created.
#[allow(clippy::disallowed_methods)] // reason: centralized fs module
pub fn create_dir_all(path: &Path) -> Result<(), std::io::Error> {
    let target = path.to_path_buf();
    std::fs::create_dir_all(&target)
}

/// Copy a file.
///
/// # Errors
/// Returns `std::io::Error` if the file cannot be copied.
#[allow(clippy::disallowed_methods)] // reason: centralized fs module
pub fn copy_file(from: &Path, to: &Path) -> Result<u64, std::io::Error> {
    std::fs::copy(from, to)
}

/// Set file permissions.
///
/// # Errors
/// Returns `std::io::Error` if permissions cannot be set.
#[allow(clippy::disallowed_methods)] // reason: centralized fs module
pub fn set_permissions(
    path: &Path,
    permissions: std::fs::Permissions,
) -> Result<(), std::io::Error> {
    std::fs::set_permissions(path, permissions)
}
