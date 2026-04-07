use std::path::Path;

use g3rs_workspace_crawl_types::{
    G3RsWorkspaceEntry, G3RsWorkspaceEntryKind, G3RsWorkspaceIgnoreState, G3RsWorkspacePath,
};
pub(crate) fn build_entry(
    workspace_root: &Path,
    path: &Path,
    kind: G3RsWorkspaceEntryKind,
    ignore_state: G3RsWorkspaceIgnoreState,
) -> G3RsWorkspaceEntry {
    let rel_path = path
        .strip_prefix(workspace_root)
        .expect("walked path should stay under workspace root")
        .to_string_lossy()
        .replace('\\', "/");
    let abs_path = path.to_path_buf();

    G3RsWorkspaceEntry {
        path: G3RsWorkspacePath { rel_path, abs_path },
        kind,
        ignore_state,
        readable: is_readable(path, kind),
    }
}

fn is_readable(path: &Path, kind: G3RsWorkspaceEntryKind) -> bool {
    match kind {
        G3RsWorkspaceEntryKind::File => crate::fs::is_readable_file(path),
        G3RsWorkspaceEntryKind::Directory => crate::fs::is_readable_directory(path),
    }
}
