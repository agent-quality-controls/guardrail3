/// Fails the calling test when `entry.path.rel_path` does not equal `expected`.
///
/// # Panics
/// Panics on path mismatch, which the assertion treats as a test failure.
pub fn assert_selected_rel_path(entry: &g3_workspace_crawl::G3RsWorkspaceEntry, expected: &str) {
    assert_eq!(
        entry.path.rel_path, expected,
        "selected ESLint config path mismatch"
    );
}
