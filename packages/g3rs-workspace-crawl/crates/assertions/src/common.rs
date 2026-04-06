use g3rs_workspace_crawl_types::G3RsWorkspaceEntry;

pub(crate) fn require_rel_path<'a>(
    entries: &'a [G3RsWorkspaceEntry],
    rel_path: &str,
) -> &'a G3RsWorkspaceEntry {
    entries
        .iter()
        .find(|entry| entry.path.rel_path == rel_path)
        .unwrap_or_else(|| panic!("missing crawl entry for {rel_path}; entries: {entries:#?}"))
}
