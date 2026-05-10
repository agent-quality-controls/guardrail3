use g3_workspace_crawl_types::{G3WorkspaceCrawl, G3WorkspaceEntry, G3WorkspaceEntryKind};

/// Find an entry by exact workspace-relative path.
#[must_use]
pub fn entry<'a>(crawl: &'a G3WorkspaceCrawl, rel_path: &str) -> Option<&'a G3WorkspaceEntry> {
    crawl
        .entries
        .iter()
        .find(|entry| entry.path.rel_path == rel_path)
}

/// Find a root-level file by filename.
#[must_use]
pub fn root_file<'a>(crawl: &'a G3WorkspaceCrawl, file_name: &str) -> Option<&'a G3WorkspaceEntry> {
    crawl
        .entries
        .iter()
        .find(|entry| entry.kind == G3WorkspaceEntryKind::File && entry.path.rel_path == file_name)
}

/// Collect all file entries with the given extension, without filtering by ignore state.
#[must_use]
pub fn files_with_extension<'a>(
    crawl: &'a G3WorkspaceCrawl,
    extension: &str,
) -> Vec<&'a G3WorkspaceEntry> {
    crawl
        .entries
        .iter()
        .filter(|entry| {
            entry.kind == G3WorkspaceEntryKind::File
                && entry
                    .path
                    .abs_path
                    .extension()
                    .is_some_and(|ext| ext == extension)
        })
        .collect()
}

#[cfg(test)]
#[path = "query_tests/mod.rs"] // reason: owned sidecar tests for file module.
mod query_tests;
