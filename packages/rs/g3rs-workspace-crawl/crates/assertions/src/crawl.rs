use g3rs_workspace_crawl_runtime::{
    G3RsWorkspaceCrawl, G3RsWorkspaceEntry, G3RsWorkspaceEntryKind, G3RsWorkspaceIgnoreState,
    entry, files_with_extension, root_file,
};

pub fn assert_has_rel_path(entries: &[G3RsWorkspaceEntry], rel_path: &str) {
    assert!(
        entries.iter().any(|entry| entry.path.rel_path == rel_path),
        "missing crawl entry for {rel_path}; entries: {entries:#?}"
    );
}

pub fn assert_entry(
    entry: &G3RsWorkspaceEntry,
    kind: G3RsWorkspaceEntryKind,
    ignore_state: G3RsWorkspaceIgnoreState,
    readable: bool,
) {
    assert_eq!(entry.kind, kind, "unexpected entry kind: {entry:#?}");
    assert_eq!(
        entry.ignore_state, ignore_state,
        "unexpected ignore state: {entry:#?}"
    );
    assert_eq!(
        entry.readable, readable,
        "unexpected readability: {entry:#?}"
    );
}

pub fn assert_crawl_entry_exists(crawl: &G3RsWorkspaceCrawl, rel_path: &str) {
    assert!(
        entry(crawl, rel_path).is_some(),
        "missing crawl entry for {rel_path}; crawl: {crawl:#?}"
    );
}

pub fn assert_crawl_entry_absent(crawl: &G3RsWorkspaceCrawl, rel_path: &str) {
    assert!(
        entry(crawl, rel_path).is_none(),
        "unexpected crawl entry for {rel_path}; crawl: {crawl:#?}"
    );
}

pub fn assert_crawl_entry(
    crawl: &G3RsWorkspaceCrawl,
    rel_path: &str,
    kind: G3RsWorkspaceEntryKind,
    ignore_state: G3RsWorkspaceIgnoreState,
    readable: bool,
) {
    if let Some(entry) = entry(crawl, rel_path) {
        assert_entry(entry, kind, ignore_state, readable);
    } else {
        assert!(
            false,
            "missing crawl entry for {rel_path}; crawl: {crawl:#?}"
        );
    }
}

pub fn assert_root_file_exists(crawl: &G3RsWorkspaceCrawl, file_name: &str) {
    assert!(
        root_file(crawl, file_name).is_some(),
        "missing root file {file_name}; crawl: {crawl:#?}"
    );
}

pub fn assert_extension_count(crawl: &G3RsWorkspaceCrawl, extension: &str, expected_count: usize) {
    let actual = files_with_extension(crawl, extension);
    assert_eq!(
        actual.len(),
        expected_count,
        "unexpected .{extension} file count: {actual:#?}"
    );
}
