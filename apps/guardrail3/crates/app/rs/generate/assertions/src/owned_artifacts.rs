pub fn assert_contains(haystack: &str, needle: &str, context: &str) {
    assert!(
        haystack.contains(needle),
        "{context}: expected to find `{needle}` in:\n{haystack}"
    );
}

pub fn assert_not_contains(haystack: &str, needle: &str, context: &str) {
    assert!(
        !haystack.contains(needle),
        "{context}: did not expect to find `{needle}` in:\n{haystack}"
    );
}
