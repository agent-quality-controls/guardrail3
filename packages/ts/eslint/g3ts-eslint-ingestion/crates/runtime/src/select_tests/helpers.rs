use tempfile::TempDir;

use g3_workspace_crawl::{
    G3WorkspaceCrawl, G3WorkspaceEntry, G3WorkspaceEntryKind, G3WorkspaceIgnoreState,
    G3WorkspacePath,
};

pub(super) fn fake_root() -> TempDir {
    TempDir::new().expect("tempdir should be created")
}

pub(super) fn crawl_with_entries(root: &TempDir, rel_paths: &[&str]) -> G3WorkspaceCrawl {
    G3WorkspaceCrawl {
        root_abs_path: root.path().to_path_buf(),
        entries: rel_paths
            .iter()
            .map(|rel_path| entry(root, rel_path))
            .collect(),
    }
}

/// `(rel_path, ignore_state, readable)` triple used when seeding a test workspace crawl.
type CustomEntrySpec<'a> = (&'a str, G3WorkspaceIgnoreState, bool);

pub(super) fn crawl_with_custom_entries(
    root: &TempDir,
    entries: &[CustomEntrySpec<'_>],
) -> G3WorkspaceCrawl {
    G3WorkspaceCrawl {
        root_abs_path: root.path().to_path_buf(),
        entries: entries
            .iter()
            .map(|(rel_path, ignore_state, readable)| {
                custom_entry(root, rel_path, *ignore_state, *readable)
            })
            .collect(),
    }
}

fn entry(root: &TempDir, rel_path: &str) -> G3WorkspaceEntry {
    custom_entry(root, rel_path, G3WorkspaceIgnoreState::Included, true)
}

fn custom_entry(
    root: &TempDir,
    rel_path: &str,
    ignore_state: G3WorkspaceIgnoreState,
    readable: bool,
) -> G3WorkspaceEntry {
    G3WorkspaceEntry {
        path: G3WorkspacePath {
            rel_path: rel_path.to_owned(),
            abs_path: root.path().join(rel_path),
        },
        kind: G3WorkspaceEntryKind::File,
        ignore_state,
        readable,
    }
}
