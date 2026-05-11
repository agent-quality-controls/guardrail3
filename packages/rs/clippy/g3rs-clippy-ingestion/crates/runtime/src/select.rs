use g3_workspace_crawl::{G3WorkspaceCrawl, G3WorkspaceEntry, G3WorkspaceEntryKind};

/// select preferred root clippy toml fn.
pub(crate) fn select_preferred_root_clippy_toml(
    crawl: &G3WorkspaceCrawl,
) -> Option<&G3WorkspaceEntry> {
    g3_workspace_crawl::root_file(crawl, ".clippy.toml")
        .or_else(|| g3_workspace_crawl::root_file(crawl, "clippy.toml"))
}

/// collect root clippy tomls fn.
pub(crate) fn collect_root_clippy_tomls(crawl: &G3WorkspaceCrawl) -> Vec<&G3WorkspaceEntry> {
    let mut entries = Vec::new();
    if let Some(entry) = g3_workspace_crawl::root_file(crawl, ".clippy.toml") {
        entries.push(entry);
    }
    if let Some(entry) = g3_workspace_crawl::root_file(crawl, "clippy.toml") {
        entries.push(entry);
    }
    entries
}

/// select root guardrail3 rs toml fn.
pub(crate) fn select_root_guardrail3_rs_toml(
    crawl: &G3WorkspaceCrawl,
) -> Option<&G3WorkspaceEntry> {
    g3_workspace_crawl::root_file(crawl, "guardrail3-rs.toml")
}

/// collect root cargo config overrides fn.
pub(crate) fn collect_root_cargo_config_overrides(
    crawl: &G3WorkspaceCrawl,
) -> Vec<&G3WorkspaceEntry> {
    [".cargo/config.toml", ".cargo/config"]
        .into_iter()
        .filter_map(|rel_path| {
            g3_workspace_crawl::entry(crawl, rel_path)
                .filter(|entry| entry.kind == G3WorkspaceEntryKind::File)
        })
        .collect()
}

/// select root cargo toml fn.
pub(crate) fn select_root_cargo_toml(crawl: &G3WorkspaceCrawl) -> Option<&G3WorkspaceEntry> {
    g3_workspace_crawl::root_file(crawl, "Cargo.toml")
}
