use g3rs_workspace_crawl::{G3RsWorkspaceCrawl, G3RsWorkspaceEntry, G3RsWorkspaceEntryKind};

pub(crate) fn select_preferred_root_clippy_toml(
    crawl: &G3RsWorkspaceCrawl,
) -> Option<&G3RsWorkspaceEntry> {
    crawl.root_file(".clippy.toml")
        .or_else(|| crawl.root_file("clippy.toml"))
}

pub(crate) fn collect_root_clippy_tomls(crawl: &G3RsWorkspaceCrawl) -> Vec<&G3RsWorkspaceEntry> {
    let mut entries = Vec::new();
    if let Some(entry) = crawl.root_file(".clippy.toml") {
        entries.push(entry);
    }
    if let Some(entry) = crawl.root_file("clippy.toml") {
        entries.push(entry);
    }
    entries
}

pub(crate) fn select_root_guardrail_toml(
    crawl: &G3RsWorkspaceCrawl,
) -> Option<&G3RsWorkspaceEntry> {
    crawl.root_file("guardrail3.toml")
}

pub(crate) fn collect_root_cargo_config_overrides(
    crawl: &G3RsWorkspaceCrawl,
) -> Vec<&G3RsWorkspaceEntry> {
    [".cargo/config.toml", ".cargo/config"]
        .into_iter()
        .filter_map(|rel_path| {
            crawl.entry(rel_path).filter(|entry| entry.kind == G3RsWorkspaceEntryKind::File)
        })
        .collect()
}

pub(crate) fn select_root_cargo_toml(crawl: &G3RsWorkspaceCrawl) -> Option<&G3RsWorkspaceEntry> {
    crawl.root_file("Cargo.toml")
}
