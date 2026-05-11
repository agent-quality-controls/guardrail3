/// Select deny-family files from a workspace crawl.
use g3_workspace_crawl::{G3WorkspaceCrawl, G3WorkspaceEntry};

/// Find the highest-precedence deny config at the workspace root.
pub(crate) fn select_deny_toml(crawl: &G3WorkspaceCrawl) -> Option<&G3WorkspaceEntry> {
    root_deny_entries(crawl).into_iter().next()
}

/// Implements `root deny entries`.
pub(crate) fn root_deny_entries(crawl: &G3WorkspaceCrawl) -> Vec<&G3WorkspaceEntry> {
    let mut entries = ["deny.toml", ".deny.toml", ".cargo/deny.toml"]
        .into_iter()
        .filter_map(|rel_path| g3_workspace_crawl::root_file(crawl, rel_path))
        .collect::<Vec<_>>();
    entries.sort_by_key(|entry| deny_precedence(&entry.path.rel_path));
    entries
}

/// Implements `select guardrail3 rs toml`.
pub(crate) fn select_guardrail3_rs_toml(crawl: &G3WorkspaceCrawl) -> Option<&G3WorkspaceEntry> {
    g3_workspace_crawl::root_file(crawl, "guardrail3-rs.toml")
}

/// Implements `deny precedence`.
fn deny_precedence(rel_path: &str) -> usize {
    match rel_path {
        ".deny.toml" => 1,
        ".cargo/deny.toml" => 2,
        // Treat the canonical `deny.toml` and any unknown path as highest precedence (0).
        _ => 0,
    }
}
