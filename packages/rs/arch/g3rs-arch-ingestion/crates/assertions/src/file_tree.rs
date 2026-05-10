use g3rs_arch_types::types::G3RsArchRustPolicyState;
use guardrail3_check_types::{G3CheckResult, G3Severity};

/// Asserts that `results` contains at least one finding matching `id`, `severity`, and `file`.
///
/// # Panics
///
/// Panics when no matching finding is present in `results`.
pub fn assert_has_result(
    results: &[G3CheckResult],
    id: &str,
    severity: G3Severity,
    file: Option<&str>,
) {
    assert!(
        results.iter().any(|result| {
            result.id() == id
                && result.severity() == severity
                && file.is_none_or(|expected| result.file() == Some(expected))
        }),
        "{results:#?}"
    );
}

/// Asserts that no finding in `results` has the given `id`.
///
/// # Panics
///
/// Panics when at least one finding with `id` is present in `results`.
pub fn assert_missing_result(results: &[G3CheckResult], id: &str) {
    assert!(
        !results.iter().any(|result| result.id() == id),
        "{results:#?}"
    );
}

/// Asserts the parsed rust policy carries exactly one waiver for the given rule/file/selector.
///
/// # Panics
///
/// Panics when `rust_policy` is not `Parsed`, when the waiver count is not one, or when
/// the waiver's rule/file/selector do not match the expected values.
pub fn assert_parsed_rust_policy(
    rust_policy: &G3RsArchRustPolicyState,
    expected_rule: &str,
    expected_file: &str,
    expected_selector: &str,
) {
    assert!(
        matches!(rust_policy, G3RsArchRustPolicyState::Parsed { .. }),
        "expected parsed rust policy, got {rust_policy:#?}"
    );
    let G3RsArchRustPolicyState::Parsed { waivers, .. } = rust_policy else {
        return;
    };

    assert_eq!(waivers.len(), 1, "{waivers:#?}");
    let Some(waiver) = waivers.first() else {
        return;
    };
    assert_eq!(waiver.rule, expected_rule, "waiver rule mismatch");
    assert_eq!(waiver.file, expected_file, "waiver file mismatch");
    assert_eq!(
        waiver.selector, expected_selector,
        "waiver selector mismatch"
    );
}
