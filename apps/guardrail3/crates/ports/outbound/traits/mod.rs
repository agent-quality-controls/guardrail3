//! Outbound port traits for external I/O.
//!
//! These traits define the boundaries between business logic and infrastructure.
//! The `app` layer depends on these traits; the `adapters` layer implements them.

use std::path::Path;

/// Filesystem operations abstraction.
pub trait FileSystem: Send + Sync {
    /// Read a file to string. Returns `None` if the file doesn't exist or can't be read.
    fn read_file(&self, path: &Path) -> Option<String>;

    /// Read a file to string, returning the error.
    ///
    /// # Errors
    /// Returns `std::io::Error` if the file cannot be read.
    fn read_file_err(&self, path: &Path) -> Result<String, std::io::Error>;

    /// List directory entries. Returns empty vec on failure.
    fn list_dir(&self, path: &Path) -> Vec<std::fs::DirEntry>;

    /// Get file metadata (size, modified time, permissions).
    /// Returns `None` if the file doesn't exist or metadata can't be read.
    fn metadata(&self, path: &Path) -> Option<std::fs::Metadata>;
}

/// Tool/command runner abstraction.
pub trait ToolChecker: Send + Sync {
    /// Check if a tool is installed and available on PATH.
    fn is_installed(&self, tool: &str) -> bool;

    /// Run `cargo publish --dry-run` and return stderr output.
    /// Returns `None` if the command fails to execute.
    fn run_cargo_publish_dry_run(&self, path: &Path) -> Option<String>;
}
