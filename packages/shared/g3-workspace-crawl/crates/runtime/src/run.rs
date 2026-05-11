//! Public crawl entry points and error type.

use std::path::Path;

use g3_workspace_crawl_types::G3WorkspaceCrawl;

/// Crawl failure for one explicit workspace root.
#[derive(Debug)]
pub enum G3WorkspaceCrawlError {
    /// The provided root path does not exist or is not a directory.
    InvalidRoot(std::path::PathBuf),
    /// The provided root path is not an explicit Rust workspace or package root.
    MissingWorkspaceManifest(std::path::PathBuf),
}

impl std::fmt::Display for G3WorkspaceCrawlError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidRoot(path) => {
                write!(formatter, "path is not a directory: {}", path.display())
            }
            Self::MissingWorkspaceManifest(path) => write!(
                formatter,
                "g3rs validates one Rust workspace or package root at a time. Target path \"{}\" has no root Cargo.toml. Run g3rs with --path pointing at a directory that contains the Rust workspace Cargo.toml.",
                path.display()
            ),
        }
    }
}

impl std::error::Error for G3WorkspaceCrawlError {}

/// Crawl one explicit workspace root into a neutral filesystem snapshot.
///
/// # Errors
///
/// Returns an error when the path is not a directory or when no
/// `Cargo.toml` is found at the workspace root.
pub fn crawl(workspace_root: &Path) -> Result<G3WorkspaceCrawl, G3WorkspaceCrawlError> {
    crate::crawl::crawl_workspace(workspace_root)
}

/// Crawl one explicit non-Rust project root into a neutral filesystem snapshot.
///
/// # Errors
///
/// Returns an error when the path is not a directory.
pub fn crawl_any_root(workspace_root: &Path) -> Result<G3WorkspaceCrawl, G3WorkspaceCrawlError> {
    crate::crawl::crawl_any_root(workspace_root)
}

#[cfg(test)]
#[path = "run_tests/mod.rs"] // reason: owned sidecar tests for file module.
mod run_tests;
