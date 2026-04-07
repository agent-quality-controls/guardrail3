use std::path::PathBuf;

use crate::{G3RsWorkspaceEntry, G3RsWorkspaceEntryKind};

/// Filesystem crawl snapshot for one explicit workspace root.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3RsWorkspaceCrawl {
    /// Absolute workspace root path that was crawled.
    pub root_abs_path: PathBuf,
    /// Discovered descendant entries, sorted by relative path.
    pub entries: Vec<G3RsWorkspaceEntry>,
}

impl G3RsWorkspaceCrawl {
    /// Find an entry by exact workspace-relative path.
    #[must_use]
    pub fn entry(&self, rel_path: &str) -> Option<&G3RsWorkspaceEntry> {
        self.entries
            .iter()
            .find(|entry| entry.path.rel_path == rel_path)
    }

    /// Find a root-level file by filename.
    #[must_use]
    pub fn root_file(&self, file_name: &str) -> Option<&G3RsWorkspaceEntry> {
        self.entries.iter().find(|entry| {
            entry.kind == G3RsWorkspaceEntryKind::File && entry.path.rel_path == file_name
        })
    }

    /// Collect all file entries with the given extension, without filtering by ignore state.
    #[must_use]
    pub fn files_with_extension(&self, extension: &str) -> Vec<&G3RsWorkspaceEntry> {
        self.entries
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
}
