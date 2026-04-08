/// Whether a repo-relative source path is fixture content that should not be
/// treated as owned code.
pub(crate) fn is_fixture_path(rel_path: &str) -> bool {
    rel_path.contains("/tests/fixtures/") || rel_path.starts_with("tests/fixtures/")
}

/// Whether a repo-relative source path belongs to test-owned code.
pub(crate) fn is_test_root_path(rel_path: &str) -> bool {
    let segments = rel_path.split('/').collect::<Vec<_>>();
    if segments.first().is_some_and(|segment| *segment == "tests") {
        return true;
    }
    if segments.iter().any(|segment| segment.ends_with("_tests")) {
        return true;
    }
    let Some(file_name) = segments.last() else {
        return false;
    };
    file_name.ends_with("_test.rs") || file_name.ends_with("_tests.rs")
}

/// Placeholder profile lookup for the first AST ingestion cut.
pub(crate) fn resolve_profile_name(_rel_path: &str) -> Option<String> {
    None
}
