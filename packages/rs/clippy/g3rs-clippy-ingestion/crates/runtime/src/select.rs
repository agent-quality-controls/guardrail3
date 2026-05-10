use g3rs_workspace_crawl::{G3RsWorkspaceCrawl, G3RsWorkspaceEntry, G3RsWorkspaceEntryKind};

/// select preferred root clippy toml fn.
pub(crate) fn select_preferred_root_clippy_toml(
    crawl: &G3RsWorkspaceCrawl,
) -> Option<&G3RsWorkspaceEntry> {
    g3rs_workspace_crawl::root_file(crawl, ".clippy.toml")
        .or_else(|| g3rs_workspace_crawl::root_file(crawl, "clippy.toml"))
}

/// collect root clippy tomls fn.
pub(crate) fn collect_root_clippy_tomls(crawl: &G3RsWorkspaceCrawl) -> Vec<&G3RsWorkspaceEntry> {
    let mut entries = Vec::new();
    if let Some(entry) = g3rs_workspace_crawl::root_file(crawl, ".clippy.toml") {
        entries.push(entry);
    }
    if let Some(entry) = g3rs_workspace_crawl::root_file(crawl, "clippy.toml") {
        entries.push(entry);
    }
    entries
}

/// select root guardrail3 rs toml fn.
pub(crate) fn select_root_guardrail3_rs_toml(
    crawl: &G3RsWorkspaceCrawl,
) -> Option<&G3RsWorkspaceEntry> {
    g3rs_workspace_crawl::root_file(crawl, "guardrail3-rs.toml")
}

/// collect root cargo config overrides fn.
pub(crate) fn collect_root_cargo_config_overrides(
    crawl: &G3RsWorkspaceCrawl,
) -> Vec<&G3RsWorkspaceEntry> {
    [".cargo/config.toml", ".cargo/config"]
        .into_iter()
        .filter_map(|rel_path| {
            g3rs_workspace_crawl::entry(crawl, rel_path)
                .filter(|entry| entry.kind == G3RsWorkspaceEntryKind::File)
        })
        .collect()
}

/// select root cargo toml fn.
pub(crate) fn select_root_cargo_toml(crawl: &G3RsWorkspaceCrawl) -> Option<&G3RsWorkspaceEntry> {
    g3rs_workspace_crawl::root_file(crawl, "Cargo.toml")
}
