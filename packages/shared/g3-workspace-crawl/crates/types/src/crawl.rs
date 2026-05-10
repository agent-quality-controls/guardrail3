use std::path::PathBuf;

use crate::G3WorkspaceEntry;

/// Filesystem crawl snapshot for one explicit workspace root.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3WorkspaceCrawl {
    /// Absolute workspace root path that was crawled.
    pub root_abs_path: PathBuf,
    /// Discovered descendant entries, sorted by relative path.
    pub entries: Vec<G3WorkspaceEntry>,
}
