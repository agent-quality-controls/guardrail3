use std::path::Path;

use g3_workspace_crawl_types::G3WorkspaceCrawl;

/// Crawl failure for one explicit workspace root.
#[derive(Debug)]
pub enum G3WorkspaceCrawlError {
    /// The provided root path does not exist or is not a directory.
    InvalidRoot(std::path::PathBuf),
}

/// Crawls one explicit workspace root into a neutral filesystem snapshot.
///
/// # Errors
/// Returns [`G3WorkspaceCrawlError::InvalidRoot`] when `workspace_root` does not
/// resolve to a readable directory.
pub fn crawl(workspace_root: &Path) -> Result<G3WorkspaceCrawl, G3WorkspaceCrawlError> {
    crate::crawl::crawl_workspace(workspace_root)
}

#[cfg(test)]
#[path = "run_tests/mod.rs"] // reason: owned sidecar tests for file module.
mod run_tests;
