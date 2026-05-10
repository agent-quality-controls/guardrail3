use std::path::PathBuf;

/// One workspace-relative path with both repo-relative and absolute forms.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct G3WorkspacePath {
    /// Path relative to the explicit workspace root.
    pub rel_path: String,
    /// Absolute filesystem path to the same entry.
    pub abs_path: PathBuf,
}

/// Neutral entry kind discovered by the crawl.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum G3WorkspaceEntryKind {
    /// A regular file.
    File,
    /// A directory.
    Directory,
}

/// Shared ignore state computed from workspace-local gitignore semantics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum G3WorkspaceIgnoreState {
    /// The path is included by gitignore rules.
    Included,
    /// The path is ignored by gitignore rules.
    Ignored,
}

/// One crawled entry under the explicit workspace root.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3WorkspaceEntry {
    /// Neutral path identity.
    pub path: G3WorkspacePath,
    /// File vs directory.
    pub kind: G3WorkspaceEntryKind,
    /// Shared ignore state.
    pub ignore_state: G3WorkspaceIgnoreState,
    /// Whether the entry can currently be read/listed.
    pub readable: bool,
}
