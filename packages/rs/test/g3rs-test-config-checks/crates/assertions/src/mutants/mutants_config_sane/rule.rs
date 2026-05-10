use guardrail3_check_types::{G3CheckResult, G3Severity};

/// Panics if the expected finding shape is absent.
///
/// # Panics
///
/// Panics if results do not satisfy the assertion.
pub fn assert_excludes_everything(results: &[G3CheckResult]) {
    assert!(
        results.iter().any(|result| {
            result.id() == "g3rs-test/mutants-config-sane"
                && result.severity() == G3Severity::Error
                && result.title() == "mutants config excludes everything"
                && result.file() == Some(".cargo/mutants.toml")
        }),
        "missing mutants excludes-everything result: {results:#?}"
    );
}

/// Panics if the expected finding shape is absent.
///
/// # Panics
///
/// Panics if results do not satisfy the assertion.
pub fn assert_timeout_too_low(results: &[G3CheckResult]) {
    assert!(
        results.iter().any(|result| {
            result.id() == "g3rs-test/mutants-config-sane"
                && result.severity() == G3Severity::Error
                && result.title() == "mutants timeout multiplier too low"
                && result.file() == Some(".cargo/mutants.toml")
        }),
        "missing mutants timeout-too-low result: {results:#?}"
    );
}

/// Panics if the expected finding shape is absent.
///
/// # Panics
///
/// Panics if results do not satisfy the assertion.
pub fn assert_sane(results: &[G3CheckResult]) {
    assert!(
        results.iter().any(|result| {
            result.id() == "g3rs-test/mutants-config-sane"
                && result.severity() == G3Severity::Info
                && result.title() == "mutants config looks sane"
                && result.file() == Some(".cargo/mutants.toml")
        }),
        "missing mutants sane result: {results:#?}"
    );
}
