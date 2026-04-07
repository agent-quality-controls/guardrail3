/// Select the root clippy config entry from a workspace crawl.
use g3rs_workspace_crawl::{G3RsWorkspaceCrawl, G3RsWorkspaceEntry};

/// Find the workspace-root `clippy.toml` or `.clippy.toml` in the crawl result.
///
/// Prefers `clippy.toml` over `.clippy.toml` when both exist.
pub(crate) fn select_clippy_toml(crawl: &G3RsWorkspaceCrawl) -> Option<&G3RsWorkspaceEntry> {
    crawl
        .root_file("clippy.toml")
        .or_else(|| crawl.root_file(".clippy.toml"))
}
