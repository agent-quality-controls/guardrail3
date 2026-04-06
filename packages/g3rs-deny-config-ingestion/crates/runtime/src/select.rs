/// Select the deny config entry from a workspace crawl.
use g3rs_workspace_crawl::{G3RsWorkspaceCrawl, G3RsWorkspaceEntry};

/// Find `deny.toml` or `.deny.toml` at the workspace root.
pub(crate) fn select_deny_toml(crawl: &G3RsWorkspaceCrawl) -> Option<&G3RsWorkspaceEntry> {
    crawl
        .root_file("deny.toml")
        .or_else(|| crawl.root_file(".deny.toml"))
}
