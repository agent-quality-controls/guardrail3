/// Select deny-family files from a workspace crawl.
use g3rs_workspace_crawl::{G3RsWorkspaceCrawl, G3RsWorkspaceEntry};

/// Find the highest-precedence deny config at the workspace root.
pub(crate) fn select_deny_toml(crawl: &G3RsWorkspaceCrawl) -> Option<&G3RsWorkspaceEntry> {
    root_deny_entries(crawl).into_iter().next()
}

pub(crate) fn root_deny_entries(crawl: &G3RsWorkspaceCrawl) -> Vec<&G3RsWorkspaceEntry> {
    let mut entries = ["deny.toml", ".deny.toml", ".cargo/deny.toml"]
        .into_iter()
        .filter_map(|rel_path| g3rs_workspace_crawl::root_file(crawl, rel_path))
        .collect::<Vec<_>>();
    entries.sort_by_key(|entry| deny_precedence(&entry.path.rel_path));
    entries
}

pub(crate) fn select_guardrail3_rs_toml(crawl: &G3RsWorkspaceCrawl) -> Option<&G3RsWorkspaceEntry> {
    g3rs_workspace_crawl::root_file(crawl, "guardrail3-rs.toml")
}

fn deny_precedence(rel_path: &str) -> usize {
    match rel_path {
        "deny.toml" => 0,
        ".deny.toml" => 1,
        ".cargo/deny.toml" => 2,
        _ => 0,
    }
}
