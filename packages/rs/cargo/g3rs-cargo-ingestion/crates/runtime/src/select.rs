/// Select the root `Cargo.toml` entry from a workspace crawl.
use g3rs_workspace_crawl::{G3RsWorkspaceCrawl, G3RsWorkspaceEntry};

/// Find the workspace-root `Cargo.toml` in the crawl result.
pub(crate) fn select_root_cargo_toml(crawl: &G3RsWorkspaceCrawl) -> Option<&G3RsWorkspaceEntry> {
    crawl.root_file("Cargo.toml")
}
