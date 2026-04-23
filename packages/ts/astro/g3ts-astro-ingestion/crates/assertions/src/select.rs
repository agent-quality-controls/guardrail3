pub fn assert_selected_rel_path(entry: &g3_workspace_crawl::G3WorkspaceEntry, expected: &str) {
    assert_eq!(
        entry.path.rel_path, expected,
        "selected ESLint config path mismatch"
    );
}
