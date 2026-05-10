/// Asserts the cargo gate command sequence matches expectations.
///
/// # Panics
///
/// Panics when the actual command list does not match the expected list.
pub fn assert_command_sequence(actual: &[&[&str]], expected: &[&[&str]]) {
    assert_eq!(
        actual.len(),
        expected.len(),
        "cargo gate command count should match: actual={actual:?} expected={expected:?}"
    );
    for (index, (got, want)) in actual.iter().zip(expected.iter()).enumerate() {
        assert_eq!(
            got, want,
            "cargo gate command {index} should match expected tokens"
        );
    }
}

/// Asserts a path is or is not classified as Rust-relevant.
///
/// # Panics
///
/// Panics when the actual classification does not match `expected`.
pub fn assert_rust_relevance(path: &str, actual: bool, expected: bool) {
    assert_eq!(
        actual, expected,
        "path {path:?} rust-relevance should be {expected}"
    );
}
