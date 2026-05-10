use guardrail3_check_types::{G3CheckResult, G3Severity};

/// Panics if the expected finding shape is absent.
///
/// # Panics
///
/// Panics if results do not satisfy the assertion.
pub fn assert_missing(results: &[G3CheckResult]) {
    assert!(
        results.iter().any(|result| {
            result.id() == "g3rs-test/mutation-hook-present"
                && result.severity() == G3Severity::Error
                && result.title() == "mutation hook step missing"
                && result.file() == Some("Cargo.toml")
        }),
        "missing mutation hook missing result: {results:#?}"
    );
}

/// Panics if the expected finding shape is absent.
///
/// # Panics
///
/// Panics if results do not satisfy the assertion.
pub fn assert_present(results: &[G3CheckResult]) {
    assert!(
        results.iter().any(|result| {
            result.id() == "g3rs-test/mutation-hook-present"
                && result.severity() == G3Severity::Info
                && result.title() == "mutation hook step present"
                && result.file() == Some(".githooks/pre-commit")
        }),
        "missing mutation hook present result: {results:#?}"
    );
}
