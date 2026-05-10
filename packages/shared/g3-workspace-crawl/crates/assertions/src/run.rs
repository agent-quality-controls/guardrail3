use g3_workspace_crawl_runtime::{
    G3WorkspaceCrawl, G3WorkspaceEntry, G3WorkspaceEntryKind, G3WorkspaceIgnoreState, entry,
    root_file,
};

/// Asserts that `entries` contains an entry with the given workspace-relative path.
///
/// # Panics
/// Panics when no entry has `path.rel_path == rel_path`.
pub fn assert_has_rel_path(entries: &[G3WorkspaceEntry], rel_path: &str) {
    assert!(
        entries.iter().any(|entry| entry.path.rel_path == rel_path),
        "missing crawl entry for {rel_path}; entries: {entries:#?}"
    );
}

/// Asserts that `crawl` contains an entry at `rel_path`.
///
/// # Panics
/// Panics when no entry has `path.rel_path == rel_path`.
pub fn assert_crawl_entry_exists(crawl: &G3WorkspaceCrawl, rel_path: &str) {
    assert!(
        entry(crawl, rel_path).is_some(),
        "missing crawl entry for {rel_path}; crawl: {crawl:#?}"
    );
}

/// Asserts that `crawl` does not contain an entry at `rel_path`.
///
/// # Panics
/// Panics when an entry with `path.rel_path == rel_path` is present.
pub fn assert_crawl_entry_absent(crawl: &G3WorkspaceCrawl, rel_path: &str) {
    assert!(
        entry(crawl, rel_path).is_none(),
        "unexpected crawl entry for {rel_path}; crawl: {crawl:#?}"
    );
}

/// Asserts that `crawl` contains a fully matching entry at `rel_path`.
///
/// # Panics
/// Panics when no entry matches `rel_path`, or when its kind, ignore state, or
/// readability differ from the supplied expectations.
pub fn assert_crawl_entry(
    crawl: &G3WorkspaceCrawl,
    rel_path: &str,
    kind: G3WorkspaceEntryKind,
    ignore_state: G3WorkspaceIgnoreState,
    readable: bool,
) {
    let found = entry(crawl, rel_path);
    assert!(
        found.is_some(),
        "missing crawl entry for {rel_path}; crawl: {crawl:#?}",
    );
    if let Some(found_entry) = found {
        assert_eq!(
            found_entry.kind, kind,
            "unexpected entry kind: {found_entry:#?}",
        );
        assert_eq!(
            found_entry.ignore_state, ignore_state,
            "unexpected ignore state: {found_entry:#?}",
        );
        assert_eq!(
            found_entry.readable, readable,
            "unexpected readability: {found_entry:#?}",
        );
    }
}

/// Asserts that `crawl` contains a root-level file with the given filename.
///
/// # Panics
/// Panics when no root-level file entry has the given filename.
pub fn assert_root_file_exists(crawl: &G3WorkspaceCrawl, file_name: &str) {
    assert!(
        root_file(crawl, file_name).is_some(),
        "missing root file {file_name}; crawl: {crawl:#?}"
    );
}
