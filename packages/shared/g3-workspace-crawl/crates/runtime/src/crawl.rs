use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use g3_workspace_crawl_types::{
    G3WorkspaceCrawl, G3WorkspaceEntry, G3WorkspaceEntryKind, G3WorkspaceIgnoreState,
};
use ignore::WalkBuilder;
use walkdir::WalkDir;

use crate::run::G3WorkspaceCrawlError;

/// Crawls `workspace_root` and returns one [`G3WorkspaceCrawl`] aggregate.
pub(crate) fn crawl_workspace(
    workspace_root: &Path,
) -> Result<G3WorkspaceCrawl, G3WorkspaceCrawlError> {
    if !workspace_root.is_dir() {
        return Err(G3WorkspaceCrawlError::InvalidRoot(
            workspace_root.to_path_buf(),
        ));
    }

    let root_abs_path = workspace_root.to_path_buf();
    let mut entries = Vec::<G3WorkspaceEntry>::new();
    let mut included_paths = BTreeSet::<PathBuf>::new();

    collect_included_entries(workspace_root, &mut entries, &mut included_paths);
    collect_recovered_entries(workspace_root, &mut entries, &included_paths);

    entries.sort_by(|left, right| left.path.rel_path.cmp(&right.path.rel_path));

    Ok(G3WorkspaceCrawl {
        root_abs_path,
        entries,
    })
}

/// Phase 1: walk with the `ignore` crate for correct gitignore semantics.
///
/// Handles ancestor gitignores (parents), nested gitignores during descent,
/// and dotfiles as normal entries. No global or exclude-file semantics so
/// validation is machine-independent. Banned directories (`target`,
/// `node_modules`) are excluded even if not gitignored.
fn collect_included_entries(
    workspace_root: &Path,
    entries: &mut Vec<G3WorkspaceEntry>,
    included_paths: &mut BTreeSet<PathBuf>,
) {
    let root_for_filter = workspace_root.to_path_buf();
    let walker = WalkBuilder::new(workspace_root)
        .hidden(false)
        .git_ignore(true)
        .git_global(false)
        .git_exclude(false)
        .parents(true)
        .ignore(false)
        .follow_links(false)
        .filter_entry(move |entry| {
            if !entry.file_type().is_some_and(|ft| ft.is_dir()) {
                return true;
            }
            let Ok(rel) = entry.path().strip_prefix(&root_for_filter) else {
                return true;
            };
            !crate::recovery::is_banned(&rel.to_string_lossy())
        })
        .build();

    for entry in walker.flatten() {
        let path = entry.path();
        if path == workspace_root {
            continue;
        }

        let Some(kind) = entry_kind(entry.file_type()) else {
            continue;
        };

        let _ = included_paths.insert(path.to_path_buf());
        entries.push(crate::support::build_entry(
            workspace_root,
            path,
            kind,
            G3WorkspaceIgnoreState::Included,
        ));
    }
}

/// Phase 2: targeted recovery of ignored-but-relevant files.
///
/// Walks the workspace with walkdir (which does not respect gitignore),
/// skips banned directories, and recovers files matching the recovery list
/// that were not found in phase 1.
fn collect_recovered_entries(
    workspace_root: &Path,
    entries: &mut Vec<G3WorkspaceEntry>,
    included_paths: &BTreeSet<PathBuf>,
) {
    let walker = WalkDir::new(workspace_root)
        .follow_links(false)
        .into_iter()
        .filter_entry(|e| {
            if !e.file_type().is_dir() {
                return true;
            }
            let Ok(rel) = e.path().strip_prefix(workspace_root) else {
                return true;
            };
            !crate::recovery::is_banned(&rel.to_string_lossy())
        })
        .filter_map(Result::ok);

    for entry in walker {
        let path = entry.path();
        if path == workspace_root || !entry.file_type().is_file() || included_paths.contains(path) {
            continue;
        }

        let Ok(rel_stripped) = path.strip_prefix(workspace_root) else {
            continue;
        };
        let rel_path = rel_stripped.to_string_lossy();
        let name = path
            .file_name()
            .map(|n| n.to_string_lossy())
            .unwrap_or_default();

        if !crate::recovery::should_recover(&name, &rel_path) {
            continue;
        }

        entries.push(crate::support::build_entry(
            workspace_root,
            path,
            G3WorkspaceEntryKind::File,
            G3WorkspaceIgnoreState::Ignored,
        ));
    }
}

/// Maps a walker entry's file type into a [`G3WorkspaceEntryKind`].
fn entry_kind(file_type: Option<std::fs::FileType>) -> Option<G3WorkspaceEntryKind> {
    file_type.and_then(|ft| {
        if ft.is_file() {
            Some(G3WorkspaceEntryKind::File)
        } else if ft.is_dir() {
            Some(G3WorkspaceEntryKind::Directory)
        } else {
            None
        }
    })
}
