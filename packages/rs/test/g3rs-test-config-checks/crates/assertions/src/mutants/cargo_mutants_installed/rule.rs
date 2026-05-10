use guardrail3_check_types::{G3CheckResult, G3Severity};

/// Panics if the expected finding shape is absent.
///
/// # Panics
///
/// Panics if results do not satisfy the assertion.
pub fn assert_missing(results: &[G3CheckResult]) {
    assert!(
        results.iter().any(|result| {
            result.id() == "g3rs-test/cargo-mutants-installed"
                && result.severity() == G3Severity::Error
                && result.title() == "cargo-mutants missing"
                && result.file() == Some("Cargo.toml")
        }),
        "missing cargo-mutants missing result: {results:#?}"
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
            result.id() == "g3rs-test/cargo-mutants-installed"
                && result.severity() == G3Severity::Info
                && result.title() == "cargo-mutants installed"
                && result.file() == Some("Cargo.toml")
        }),
        "missing cargo-mutants installed result: {results:#?}"
    );
}
