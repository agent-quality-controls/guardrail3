/// Select config entries from a workspace crawl.
use g3rs_workspace_crawl::{G3RsWorkspaceCrawl, G3RsWorkspaceEntry};

/// Find `Cargo.toml` at the workspace root.
pub(crate) fn select_cargo_toml(crawl: &G3RsWorkspaceCrawl) -> Option<&G3RsWorkspaceEntry> {
    crawl.root_file("Cargo.toml")
}

/// Find `release-plz.toml` or `.release-plz.toml` at the workspace root.
pub(crate) fn select_release_plz_toml(crawl: &G3RsWorkspaceCrawl) -> Option<&G3RsWorkspaceEntry> {
    crawl
        .root_file("release-plz.toml")
        .or_else(|| crawl.root_file(".release-plz.toml"))
}

/// Find `cliff.toml` at the workspace root.
pub(crate) fn select_cliff_toml(crawl: &G3RsWorkspaceCrawl) -> Option<&G3RsWorkspaceEntry> {
    crawl.root_file("cliff.toml")
}
