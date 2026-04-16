use g3rs_workspace_crawl_types::{G3RsWorkspaceCrawl, G3RsWorkspaceEntry, G3RsWorkspaceEntryKind};

/// Find an entry by exact workspace-relative path.
#[must_use]
pub fn entry<'a>(crawl: &'a G3RsWorkspaceCrawl, rel_path: &str) -> Option<&'a G3RsWorkspaceEntry> {
    crawl
        .entries
        .iter()
        .find(|entry| entry.path.rel_path == rel_path)
}

/// Find a root-level file by filename.
#[must_use]
pub fn root_file<'a>(
    crawl: &'a G3RsWorkspaceCrawl,
    file_name: &str,
) -> Option<&'a G3RsWorkspaceEntry> {
    crawl.entries.iter().find(|entry| {
        entry.kind == G3RsWorkspaceEntryKind::File && entry.path.rel_path == file_name
    })
}

/// Collect all file entries with the given extension, without filtering by ignore state.
#[must_use]
pub fn files_with_extension<'a>(
    crawl: &'a G3RsWorkspaceCrawl,
    extension: &str,
) -> Vec<&'a G3RsWorkspaceEntry> {
    crawl
        .entries
        .iter()
        .filter(|entry| {
            entry.kind == G3RsWorkspaceEntryKind::File
                && entry
                    .path
                    .abs_path
                    .extension()
                    .is_some_and(|ext| ext == extension)
        })
        .collect()
}
