use std::borrow::Cow;
use std::path::{Path, PathBuf};

use g3rs_workspace_crawl::{
    G3RsWorkspaceCrawl, G3RsWorkspaceEntry, G3RsWorkspaceEntryKind, G3RsWorkspaceIgnoreState,
    G3RsWorkspacePath,
};

/// Locate a workspace-relative file path by first searching the crawl, then
/// walking upward from the workspace root until an ancestor contains the file.
///
/// The returned `Cow` either borrows the crawl entry (workspace-local hit) or
/// owns a synthesized entry whose `rel_path` is the ancestor-relative path
/// supplied as `rel_path` (e.g. `.githooks/pre-commit`). The synthesized
/// `abs_path` always points at the actual file on disk.
pub(crate) fn find_file_entry<'a>(
    crawl: &'a G3RsWorkspaceCrawl,
    rel_path: &str,
) -> Option<Cow<'a, G3RsWorkspaceEntry>> {
    if let Some(entry) = g3rs_workspace_crawl::entry(crawl, rel_path) {
        return Some(Cow::Borrowed(entry));
    }

    let abs = walk_upward_for(crawl.root_abs_path.as_path(), rel_path, |candidate| {
        crate::fs::metadata(candidate)
            .map(|metadata| metadata.is_file())
            .unwrap_or(false)
    })?;

    Some(Cow::Owned(synthetic_file_entry(abs, rel_path)))
}

/// Locate a workspace-relative directory path by first searching the crawl,
/// then walking upward. Mirrors [`find_file_entry`] for directories.
pub(crate) fn find_dir_entry<'a>(
    crawl: &'a G3RsWorkspaceCrawl,
    rel_path: &str,
) -> Option<Cow<'a, G3RsWorkspaceEntry>> {
    if let Some(entry) = g3rs_workspace_crawl::entry(crawl, rel_path) {
        if entry.kind == G3RsWorkspaceEntryKind::Directory {
            return Some(Cow::Borrowed(entry));
        }
    }

    let abs = walk_upward_for(crawl.root_abs_path.as_path(), rel_path, |candidate| {
        crate::fs::metadata(candidate)
            .map(|metadata| metadata.is_dir())
            .unwrap_or(false)
    })?;

    Some(Cow::Owned(synthetic_dir_entry(abs, rel_path)))
}

/// Walk upward from `start` (exclusive of `start` itself) and return the first
/// ancestor's `<ancestor>/<rel_path>` that satisfies `predicate`.
fn walk_upward_for(
    start: &Path,
    rel_path: &str,
    predicate: impl Fn(&Path) -> bool,
) -> Option<PathBuf> {
    let mut cursor = start.parent();
    while let Some(ancestor) = cursor {
        let candidate = ancestor.join(rel_path);
        if predicate(candidate.as_path()) {
            return Some(candidate);
        }
        cursor = ancestor.parent();
    }
    None
}

/// `synthetic_file_entry` function.
fn synthetic_file_entry(abs_path: PathBuf, rel_path: &str) -> G3RsWorkspaceEntry {
    G3RsWorkspaceEntry {
        path: G3RsWorkspacePath {
            rel_path: rel_path.to_owned(),
            abs_path,
        },
        kind: G3RsWorkspaceEntryKind::File,
        ignore_state: G3RsWorkspaceIgnoreState::Included,
        readable: true,
    }
}

/// `synthetic_dir_entry` function.
fn synthetic_dir_entry(abs_path: PathBuf, rel_path: &str) -> G3RsWorkspaceEntry {
    G3RsWorkspaceEntry {
        path: G3RsWorkspacePath {
            rel_path: rel_path.to_owned(),
            abs_path,
        },
        kind: G3RsWorkspaceEntryKind::Directory,
        ignore_state: G3RsWorkspaceIgnoreState::Included,
        readable: true,
    }
}

/// Walk upward from `start` (exclusive of `start` itself) and return the first
/// ancestor that contains `<ancestor>/.git` as a file or directory.
pub(crate) fn find_git_root(start: &Path) -> Option<PathBuf> {
    walk_upward_for(start, ".git", |candidate| {
        crate::fs::metadata(candidate).is_ok()
    })
}

/// List direct children of a directory on disk, returning absolute paths for
/// each direct child file. Used to enumerate modular hook scripts that live in
/// an ancestor directory and are therefore not part of the workspace crawl.
pub(crate) fn read_direct_files(dir_abs_path: &Path) -> Vec<PathBuf> {
    crate::fs::list_direct_files(dir_abs_path)
}
