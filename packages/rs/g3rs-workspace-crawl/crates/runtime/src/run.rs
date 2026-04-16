use std::path::Path;

use g3rs_workspace_crawl_types::G3RsWorkspaceCrawl;

/// Crawl failure for one explicit workspace root.
#[derive(Debug)]
pub enum G3RsWorkspaceCrawlError {
    /// The provided root path does not exist or is not a directory.
    InvalidRoot(std::path::PathBuf),
}

/// Crawl one explicit workspace root into a neutral filesystem snapshot.
pub fn crawl(workspace_root: &Path) -> Result<G3RsWorkspaceCrawl, G3RsWorkspaceCrawlError> {
    crate::crawl::crawl_workspace(workspace_root)
}

#[cfg(test)]
#[path = "run_tests/mod.rs"] // reason: owned sidecar tests for file module.
mod run_tests;
