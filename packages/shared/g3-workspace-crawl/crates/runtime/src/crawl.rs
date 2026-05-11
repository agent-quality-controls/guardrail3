//! Two-phase Cargo workspace crawl.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use g3_workspace_crawl_types::{
    G3WorkspaceCrawl, G3WorkspaceEntry, G3WorkspaceEntryKind, G3WorkspaceIgnoreState,
};
use ignore::WalkBuilder;
use walkdir::WalkDir;

use crate::run::G3WorkspaceCrawlError;

/// Crawl a directory that must be a Cargo workspace root.
pub(crate) fn crawl_workspace(
    workspace_root: &Path,
) -> Result<G3WorkspaceCrawl, G3WorkspaceCrawlError> {
    crawl_root(workspace_root, true)
}

/// Crawl a directory without requiring a `Cargo.toml` manifest at its root.
pub(crate) fn crawl_any_root(
    workspace_root: &Path,
) -> Result<G3WorkspaceCrawl, G3WorkspaceCrawlError> {
    crawl_root(workspace_root, false)
}

/// Run the two-phase crawl, optionally requiring a `Cargo.toml` at the root.
fn crawl_root(
    workspace_root: &Path,
    require_workspace_manifest: bool,
) -> Result<G3WorkspaceCrawl, G3WorkspaceCrawlError> {
    if !workspace_root.is_dir() {
        return Err(G3WorkspaceCrawlError::InvalidRoot(
            workspace_root.to_path_buf(),
        ));
    }
    if require_workspace_manifest && !workspace_root.join("Cargo.toml").is_file() {
        return Err(G3WorkspaceCrawlError::MissingWorkspaceManifest(
            workspace_root.to_path_buf(),
        ));
    }

    let root_abs_path = workspace_root
        .canonicalize()
        .unwrap_or_else(|_| workspace_root.to_path_buf());
    let mut entries = Vec::<G3WorkspaceEntry>::new();
    let mut included_paths = BTreeSet::<PathBuf>::new();

    walk_phase_one(root_abs_path.as_path(), &mut entries, &mut included_paths);
    walk_phase_two(root_abs_path.as_path(), &mut entries, &included_paths);

    entries.sort_by(|left, right| left.path.rel_path.cmp(&right.path.rel_path));

    Ok(G3WorkspaceCrawl {
        root_abs_path,
        entries,
    })
}

/// Phase 1: walk with the `ignore` crate honouring gitignore semantics.
///
/// Handles ancestor gitignores (parents), nested gitignores during descent,
/// and dotfiles as normal entries. No global or exclude-file semantics so
/// validation is machine-independent. Banned directories (`target`,
/// `node_modules`) are excluded even if not gitignored.
fn walk_phase_one(
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
            let rel = rel.to_string_lossy();
            !crate::recovery::is_banned(&rel)
        })
        .build();

    for entry in walker.flatten() {
        let path = entry.path();
        if path == workspace_root {
            continue;
        }

        let Some(kind) = entry_kind_from_ignore(&entry) else {
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
/// Walks the workspace with `walkdir` (which does not respect gitignore),
/// skips banned directories, and recovers files matching the recovery list
/// that were not found in phase 1. Banned directories include `target`,
/// `node_modules`, and `.git`.
fn walk_phase_two(
    workspace_root: &Path,
    entries: &mut Vec<G3WorkspaceEntry>,
    included_paths: &BTreeSet<PathBuf>,
) {
    for entry in WalkDir::new(workspace_root)
        .follow_links(false)
        .into_iter()
        .filter_entry(|e| {
            if !e.file_type().is_dir() {
                return true;
            }
            let Ok(rel) = e.path().strip_prefix(workspace_root) else {
                return true;
            };
            let rel = rel.to_string_lossy();
            !crate::recovery::is_banned(&rel)
        })
        .filter_map(Result::ok)
    {
        let path = entry.path();
        if path == workspace_root {
            continue;
        }
        let Some(kind) = entry_kind_from_walkdir(&entry) else {
            continue;
        };
        if included_paths.contains(path) {
            continue;
        }
        let Ok(rel_path_buf) = path.strip_prefix(workspace_root) else {
            continue;
        };
        let rel_path = rel_path_buf.to_string_lossy();
        let name = path
            .file_name()
            .map(|n| n.to_string_lossy())
            .unwrap_or_default();

        let should_recover = match kind {
            G3WorkspaceEntryKind::File => crate::recovery::should_recover(&name, &rel_path),
            G3WorkspaceEntryKind::Directory => crate::recovery::should_recover_dir(&name),
        };

        if !should_recover {
            continue;
        }

        entries.push(crate::support::build_entry(
            workspace_root,
            path,
            kind,
            G3WorkspaceIgnoreState::Ignored,
        ));
    }
}

/// Map an `ignore::DirEntry` file-type to a workspace-entry kind, or `None`
/// when neither file nor directory.
fn entry_kind_from_ignore(entry: &ignore::DirEntry) -> Option<G3WorkspaceEntryKind> {
    let ft = entry.file_type()?;
    if ft.is_file() {
        Some(G3WorkspaceEntryKind::File)
    } else if ft.is_dir() {
        Some(G3WorkspaceEntryKind::Directory)
    } else {
        None
    }
}

/// Map a `walkdir::DirEntry` file-type to a workspace-entry kind, or `None`
/// when neither file nor directory.
fn entry_kind_from_walkdir(entry: &walkdir::DirEntry) -> Option<G3WorkspaceEntryKind> {
    let ft = entry.file_type();
    if ft.is_file() {
        Some(G3WorkspaceEntryKind::File)
    } else if ft.is_dir() {
        Some(G3WorkspaceEntryKind::Directory)
    } else {
        None
    }
}
