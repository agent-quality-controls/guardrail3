use std::path::Path;

use g3rs_workspace_crawl_types::G3RsWorkspaceCrawl;

/// Crawl failure for one explicit workspace root.
#[derive(Debug)]
pub enum G3RsWorkspaceCrawlError {
    /// The provided root path does not exist or is not a directory.
    InvalidRoot(std::path::PathBuf),
    /// The provided root path is not an explicit Rust workspace or package root.
    MissingWorkspaceManifest(std::path::PathBuf),
}

impl std::fmt::Display for G3RsWorkspaceCrawlError {
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

impl std::error::Error for G3RsWorkspaceCrawlError {}

/// Crawl one explicit workspace root into a neutral filesystem snapshot.
pub fn crawl(workspace_root: &Path) -> Result<G3RsWorkspaceCrawl, G3RsWorkspaceCrawlError> {
    crate::crawl::crawl_workspace(workspace_root)
}

#[cfg(test)]
#[path = "run_tests/mod.rs"] // reason: owned sidecar tests for file module.
mod run_tests;
