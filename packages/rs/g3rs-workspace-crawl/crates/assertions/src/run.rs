//! Crawl-shape assertions used by integration tests.

use g3rs_workspace_crawl_runtime::{
    G3RsWorkspaceCrawl, G3RsWorkspaceEntry, G3RsWorkspaceEntryKind, G3RsWorkspaceIgnoreState,
    entry, root_file,
};

/// Assert the entry list contains an entry whose workspace-relative path
/// matches `rel_path`.
///
/// # Panics
///
/// Panics if no matching entry is found.
pub fn assert_has_rel_path(entries: &[G3RsWorkspaceEntry], rel_path: &str) {
    let mut iter = entries.iter();
    let matched = iter.any(|entry| entry.path.rel_path == rel_path);
    assert!(
        matched,
        "run side: missing crawl entry rel_path={rel_path}; entries: {entries:#?}"
    );
}

/// Assert the crawl contains an entry at `rel_path`.
///
/// # Panics
///
/// Panics if no matching entry is found.
pub fn assert_crawl_entry_exists(crawl: &G3RsWorkspaceCrawl, rel_path: &str) {
    let lookup = entry(crawl, rel_path);
    assert!(
        lookup.is_some(),
        "expected presence: missing crawl entry for {rel_path}; crawl: {crawl:#?}"
    );
}

/// Assert the crawl does not contain an entry at `rel_path`.
///
/// # Panics
///
/// Panics if a matching entry is found.
pub fn assert_crawl_entry_absent(crawl: &G3RsWorkspaceCrawl, rel_path: &str) {
    let lookup = entry(crawl, rel_path);
    assert!(
        lookup.is_none(),
        "absence required: unexpected crawl entry at {rel_path}: {lookup:#?}"
    );
}

/// Assert the entry at `rel_path` matches the expected kind, ignore state,
/// and readability.
///
/// # Panics
///
/// Panics if no entry exists at `rel_path` or if any of its observed fields
/// differ from the expected values.
pub fn assert_crawl_entry(
    crawl: &G3RsWorkspaceCrawl,
    rel_path: &str,
    kind: G3RsWorkspaceEntryKind,
    ignore_state: G3RsWorkspaceIgnoreState,
    readable: bool,
) {
    let found = entry(crawl, rel_path);
    assert!(
        found.is_some(),
        "missing crawl entry for {rel_path}; crawl: {crawl:#?}"
    );
    let Some(found) = found else { return };
    assert_eq!(found.kind, kind, "unexpected entry kind: {found:#?}");
    assert_eq!(
        found.ignore_state, ignore_state,
        "unexpected ignore state: {found:#?}"
    );
    assert_eq!(
        found.readable, readable,
        "unexpected readability: {found:#?}"
    );
}

/// Assert the crawl contains a root-level file with the given filename.
///
/// # Panics
///
/// Panics if no matching root file is found.
pub fn assert_root_file_exists(crawl: &G3RsWorkspaceCrawl, file_name: &str) {
    let entry_opt = root_file(crawl, file_name);
    let total = crawl.entries.len();
    assert!(
        entry_opt.is_some(),
        "root-level file {file_name} not in crawl ({total} entries): {crawl:#?}"
    );
}
