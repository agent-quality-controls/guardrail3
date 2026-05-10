use std::path::Path;

use g3_workspace_crawl_types::{
    G3WorkspaceEntry, G3WorkspaceEntryKind, G3WorkspaceIgnoreState, G3WorkspacePath,
};

/// Builds one [`G3WorkspaceEntry`] for `path` rooted at `workspace_root`.
///
/// Falls back to the absolute path's lossy string when `path` is not strictly
/// under `workspace_root`; this keeps the caller from having to short-circuit
/// during walks that may surface symlink-redirected entries.
pub(crate) fn build_entry(
    workspace_root: &Path,
    path: &Path,
    kind: G3WorkspaceEntryKind,
    ignore_state: G3WorkspaceIgnoreState,
) -> G3WorkspaceEntry {
    let rel_path = path
        .strip_prefix(workspace_root)
        .map_or_else(|_| path.to_string_lossy(), Path::to_string_lossy)
        .replace('\\', "/");
    let abs_path = path.to_path_buf();

    G3WorkspaceEntry {
        path: G3WorkspacePath { rel_path, abs_path },
        kind,
        ignore_state,
        readable: is_readable(path, kind),
    }
}

/// Returns `true` when `path` is readable as the indicated [`G3WorkspaceEntryKind`].
fn is_readable(path: &Path, kind: G3WorkspaceEntryKind) -> bool {
    match kind {
        G3WorkspaceEntryKind::File => crate::fs::is_readable_file(path),
        G3WorkspaceEntryKind::Directory => crate::fs::is_readable_directory(path),
    }
}
