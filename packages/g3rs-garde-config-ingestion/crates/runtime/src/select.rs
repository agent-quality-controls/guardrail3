/// Select config entries from a workspace crawl.
use g3rs_workspace_crawl::{G3RsWorkspaceCrawl, G3RsWorkspaceEntry};

/// Find the workspace-root `Cargo.toml` in the crawl result.
pub(crate) fn select_cargo_toml(crawl: &G3RsWorkspaceCrawl) -> Option<&G3RsWorkspaceEntry> {
    crawl.root_file("Cargo.toml")
}

/// Find `clippy.toml` or `.clippy.toml` at the workspace root.
pub(crate) fn select_clippy_toml(crawl: &G3RsWorkspaceCrawl) -> Option<&G3RsWorkspaceEntry> {
    crawl
        .root_file("clippy.toml")
        .or_else(|| crawl.root_file(".clippy.toml"))
}
