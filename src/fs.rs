//! Centralized filesystem operations.
//!
//! All file I/O goes through this module. Benefits:
//! - Consistent error handling
//! - Single audit point for all filesystem access
//! - Mockable for testing

use std::path::Path;

/// Read a file to string. Returns `None` if the file doesn't exist or can't be read.
#[allow(clippy::disallowed_methods)] // reason: centralized fs module
pub fn read_file(path: &Path) -> Option<String> {
    std::fs::read_to_string(path).ok()
}

/// Read a file to string, returning the error.
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
        Ok(m) => Some(m),
        Err(_) => None,
    }
}

/// Write content to a file, creating parent directories.
#[allow(clippy::disallowed_methods)] // reason: centralized fs module
pub fn write_file(path: &Path, content: &str) -> Result<(), std::io::Error> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(path, content)
}

/// Create directory and all parent directories.
///
/// Returns `Ok(())` if the directory already exists.
#[allow(clippy::disallowed_methods)] // reason: centralized fs module
pub fn create_dir_all(path: &Path) -> Result<(), std::io::Error> {
    let target = path.to_path_buf();
    std::fs::create_dir_all(&target)
}

/// Set file permissions.
#[allow(clippy::disallowed_methods)] // reason: centralized fs module
pub fn set_permissions(path: &Path, perm: std::fs::Permissions) -> Result<(), std::io::Error> {
    std::fs::set_permissions(path, perm)
}
