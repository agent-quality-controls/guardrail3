use g3rs_workspace_crawl_types::G3RsWorkspaceCrawl;

pub fn assert_root_file_exists(crawl: &G3RsWorkspaceCrawl, file_name: &str) {
    assert!(
        crawl.root_file(file_name).is_some(),
        "missing root file {file_name}; crawl: {crawl:#?}"
    );
}

pub fn assert_extension_count(crawl: &G3RsWorkspaceCrawl, extension: &str, expected_count: usize) {
    let actual = crawl.files_with_extension(extension);
    assert_eq!(
        actual.len(),
        expected_count,
        "unexpected .{extension} file count: {actual:#?}"
    );
}
