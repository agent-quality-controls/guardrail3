use std::path::Path;

use g3rs_workspace_crawl_types::{
    G3RsWorkspaceEntry,
    G3RsWorkspaceEntryKind,
    G3RsWorkspacePath,
};
use ignore::gitignore::Gitignore;
use walkdir::DirEntry;

pub(crate) fn is_inside_git_dir(workspace_root: &Path, path: &Path) -> bool {
    path.strip_prefix(workspace_root)
        .ok()
        .is_some_and(|rel_path| rel_path.components().any(|component| component.as_os_str() == ".git"))
}

pub(crate) fn entry_kind(entry: &DirEntry) -> Option<G3RsWorkspaceEntryKind> {
    if entry.file_type().is_file() {
        Some(G3RsWorkspaceEntryKind::File)
    } else if entry.file_type().is_dir() {
        Some(G3RsWorkspaceEntryKind::Directory)
    } else {
        None
    }
}

pub(crate) fn build_entry(
    workspace_root: &Path,
    path: &Path,
    kind: G3RsWorkspaceEntryKind,
    ignore_matcher: &Gitignore,
) -> G3RsWorkspaceEntry {
    let rel_path = path
        .strip_prefix(workspace_root)
        .expect("walked path should stay under workspace root")
        .to_string_lossy()
        .replace('\\', "/");
    let abs_path = path.to_path_buf();

    let ignore_state = crate::ignore::ignore_state(
        ignore_matcher,
        path,
        kind == G3RsWorkspaceEntryKind::Directory,
    );

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

#[must_use]
#[cfg(test)]
pub(crate) fn ignored<'a>(
    entries: &'a [G3RsWorkspaceEntry],
    rel_path: &str,
) -> Option<&'a G3RsWorkspaceEntry> {
    entries.iter().find(|entry| {
        entry.path.rel_path == rel_path
            && entry.ignore_state == G3RsWorkspaceIgnoreState::Ignored
    })
}
#[cfg(test)]
use g3rs_workspace_crawl_types::G3RsWorkspaceIgnoreState;
