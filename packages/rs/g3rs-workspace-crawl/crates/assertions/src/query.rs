//! Query-shape assertions over a `G3RsWorkspaceCrawl`.

use g3rs_workspace_crawl_runtime::{
    G3RsWorkspaceCrawl, G3RsWorkspaceEntry, G3RsWorkspaceEntryKind, entry, files_with_extension,
    root_file,
};

/// Assert the entry list (as queried via the public iterator) contains an
/// entry with the given workspace-relative path.
///
/// # Panics
///
/// Panics if no entry with `rel_path` is found.
pub fn assert_has_rel_path(entries: &[G3RsWorkspaceEntry], rel_path: &str) {
    let found = entries
        .iter()
        .any(|entry| entry.path.rel_path.as_str() == rel_path);
    assert!(
        found,
        "query side: no crawl entry matched rel_path={rel_path}; entries={entries:#?}"
    );
}

/// Assert the entry at `rel_path` has the expected kind.
///
/// # Panics
///
/// Panics if no entry with `rel_path` exists or if the entry's kind does
/// not equal `kind`.
pub fn assert_entry_kind(crawl: &G3RsWorkspaceCrawl, rel_path: &str, kind: G3RsWorkspaceEntryKind) {
    let found = entry(crawl, rel_path);
    assert!(
        found.is_some(),
        "missing crawl entry for {rel_path}; crawl: {crawl:#?}"
    );
    let Some(found) = found else { return };
    assert_eq!(found.kind, kind, "unexpected entry kind: {found:#?}");
}

/// Assert the crawl contains a root-level file with the given filename.
///
/// # Panics
///
/// Panics if no matching root file is found.
pub fn assert_root_file_exists(crawl: &G3RsWorkspaceCrawl, file_name: &str) {
    let result = root_file(crawl, file_name);
    let names: Vec<&str> = crawl
        .entries
        .iter()
        .map(|e| e.path.rel_path.as_str())
        .collect();
    assert!(
        result.is_some(),
        "query side: root file {file_name} missing; available: {names:?}"
    );
}

/// Assert the crawl contains exactly `expected_count` files with the given
/// extension.
///
/// # Panics
///
/// Panics if the observed count differs from `expected_count`.
pub fn assert_extension_count(crawl: &G3RsWorkspaceCrawl, extension: &str, expected_count: usize) {
    let actual = files_with_extension(crawl, extension);
    assert_eq!(
        actual.len(),
        expected_count,
        "unexpected .{extension} file count: {actual:#?}"
    );
}
