use g3rs_workspace_crawl_runtime::{
    G3RsWorkspaceCrawl, G3RsWorkspaceEntry, G3RsWorkspaceEntryKind, entry, files_with_extension,
    root_file,
};

pub fn assert_has_rel_path(entries: &[G3RsWorkspaceEntry], rel_path: &str) {
    assert!(
        entries.iter().any(|entry| entry.path.rel_path == rel_path),
        "missing crawl entry for {rel_path}; entries: {entries:#?}"
    );
}

pub fn assert_entry_kind(crawl: &G3RsWorkspaceCrawl, rel_path: &str, kind: G3RsWorkspaceEntryKind) {
    let Some(found) = entry(crawl, rel_path) else {
        assert!(
            false,
            "missing crawl entry for {rel_path}; crawl: {crawl:#?}"
        );
        return;
    };
    assert_eq!(found.kind, kind, "unexpected entry kind: {found:#?}");
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
