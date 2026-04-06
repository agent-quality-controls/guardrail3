use std::collections::HashSet;
use std::path::{Path, PathBuf};

use g3rs_workspace_crawl_types::{
    G3RsWorkspaceCrawl, G3RsWorkspaceEntry, G3RsWorkspaceEntryKind, G3RsWorkspaceIgnoreState,
};
use ignore::WalkBuilder;
use walkdir::WalkDir;

use crate::run::G3RsWorkspaceCrawlError;

pub(crate) fn crawl_workspace(
    workspace_root: &Path,
) -> Result<G3RsWorkspaceCrawl, G3RsWorkspaceCrawlError> {
    if !workspace_root.is_dir() {
        return Err(G3RsWorkspaceCrawlError::InvalidRoot(
            workspace_root.to_path_buf(),
        ));
    }

    let root_abs_path = workspace_root.to_path_buf();
    let mut entries = Vec::<G3RsWorkspaceEntry>::new();
    let mut included_paths = HashSet::<PathBuf>::new();

    // Phase 1: Walk with ignore crate for correct gitignore semantics.
    // Handles ancestor gitignores (parents), nested gitignores during descent,
    // and dotfiles as normal entries. No global or exclude-file semantics
    // so validation is machine-independent.
    let walker = WalkBuilder::new(workspace_root)
        .hidden(false)
        .git_ignore(true)
        .git_global(false)
        .git_exclude(false)
        .parents(true)
        .ignore(false)
        .follow_links(false)
        .build();

    for entry in walker.flatten() {
        let path = entry.path();
        if path == workspace_root {
            continue;
        }

        let kind = if entry.file_type().is_some_and(|ft| ft.is_file()) {
            G3RsWorkspaceEntryKind::File
        } else if entry.file_type().is_some_and(|ft| ft.is_dir()) {
            G3RsWorkspaceEntryKind::Directory
        } else {
            continue;
        };

        let _ = included_paths.insert(path.to_path_buf());
        entries.push(crate::support::build_entry(
            workspace_root,
            path,
            kind,
            G3RsWorkspaceIgnoreState::Included,
        ));
    }

    // Phase 2: Targeted recovery of ignored-but-relevant files.
    // Walks the workspace with walkdir (which does not respect gitignore),
    // skips banned directories, and recovers files matching the recovery list
    // that were not found in phase 1.
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
        if !entry.file_type().is_file() {
            continue;
        }
        if included_paths.contains(path) {
            continue;
        }

        let rel_path = path
            .strip_prefix(workspace_root)
            .expect("walked path should stay under workspace root")
            .to_string_lossy();
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
            G3RsWorkspaceEntryKind::File,
            G3RsWorkspaceIgnoreState::Ignored,
        ));
    }

    entries.sort_by(|left, right| left.path.rel_path.cmp(&right.path.rel_path));

    Ok(G3RsWorkspaceCrawl {
        root_abs_path,
        entries,
    })
}
