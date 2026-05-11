//! Internal entry-construction helpers.

use std::path::Path;

use g3_workspace_crawl_types::{
    G3WorkspaceEntry, G3WorkspaceEntryKind, G3WorkspaceIgnoreState, G3WorkspacePath,
};

/// Build a `G3WorkspaceEntry` from a walker-discovered path.
///
/// When `path` is not under `workspace_root` the relative path falls back to
/// the file's lossy representation (this should not occur for entries
/// produced by the configured walkers, which descend from `workspace_root`).
pub(crate) fn build_entry(
    workspace_root: &Path,
    path: &Path,
    kind: G3WorkspaceEntryKind,
    ignore_state: G3WorkspaceIgnoreState,
) -> G3WorkspaceEntry {
    let rel_path = path
        .strip_prefix(workspace_root)
        .map_or_else(|_| path.to_path_buf(), Path::to_path_buf)
        .to_string_lossy()
        .replace('\\', "/");
    let abs_path = path.to_path_buf();

    G3WorkspaceEntry {
        path: G3WorkspacePath { rel_path, abs_path },
        kind,
        ignore_state,
        readable: is_readable(path, kind),
    }
}

/// Whether the entry at `path` can be read from disk.
fn is_readable(path: &Path, kind: G3WorkspaceEntryKind) -> bool {
    match kind {
        G3WorkspaceEntryKind::File => crate::fs::is_readable_file(path),
        G3WorkspaceEntryKind::Directory => crate::fs::is_readable_directory(path),
    }
}
