use g3_workspace_crawl_runtime::{
    G3WorkspaceCrawl, G3WorkspaceEntryKind, entry, files_with_extension,
};

pub use crate::run::{assert_has_rel_path, assert_root_file_exists};

/// Asserts that the entry at `rel_path` exists and has the given [`G3WorkspaceEntryKind`].
///
/// # Panics
/// Panics when no entry matches `rel_path` or when its kind differs from `kind`.
pub fn assert_entry_kind(crawl: &G3WorkspaceCrawl, rel_path: &str, kind: G3WorkspaceEntryKind) {
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
    }
}

/// Asserts that `crawl` contains exactly `expected_count` files with the given extension.
///
/// # Panics
/// Panics when the actual count of files with `extension` differs from `expected_count`.
pub fn assert_extension_count(crawl: &G3WorkspaceCrawl, extension: &str, expected_count: usize) {
    let actual = files_with_extension(crawl, extension);
    assert_eq!(
        actual.len(),
        expected_count,
        "unexpected .{extension} file count: {actual:#?}"
    );
}
