use g3rs_workspace_crawl::{G3RsWorkspaceCrawl, G3RsWorkspaceEntry, G3RsWorkspaceEntryKind};

/// Selected Rust source file metadata ready to be read and mapped.
pub(crate) struct SelectedCodeSourceFile<'a> {
    /// Underlying crawl entry.
    pub(crate) entry: &'a G3RsWorkspaceEntry,
    /// Whether the file belongs to test-owned code.
    pub(crate) is_test: bool,
    /// Optional pre-resolved policy profile.
    pub(crate) profile_name: Option<String>,
}

/// Select all owned Rust source files for the `code` AST lane.
pub(crate) fn select_source_files(crawl: &G3RsWorkspaceCrawl) -> Vec<SelectedCodeSourceFile<'_>> {
    crawl
        .entries
        .iter()
        .filter(|entry| entry.kind == G3RsWorkspaceEntryKind::File)
        .filter(|entry| entry.path.rel_path.ends_with(".rs"))
        .filter(|entry| !crate::classify::is_fixture_path(entry.path.rel_path.as_str()))
        .map(|entry| SelectedCodeSourceFile {
            entry,
            is_test: crate::classify::is_test_root_path(entry.path.rel_path.as_str()),
            profile_name: crate::classify::resolve_profile_name(entry.path.rel_path.as_str()),
        })
        .collect()
}
