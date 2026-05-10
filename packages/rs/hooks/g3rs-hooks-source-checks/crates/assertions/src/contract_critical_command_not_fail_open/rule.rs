crate::define_rule_assertions!("g3rs-hooks/contract-critical-command-not-fail-open");

/// Assert that the contract-critical fail-open finding is reported.
///
/// # Panics
///
/// Panics if no matching error finding is present.
pub fn assert_fail_open_error(results: &[guardrail3_check_types::G3CheckResult]) {
    assert!(
        results.iter().any(|result| {
            !result.inventory()
                && result.id() == "g3rs-hooks/contract-critical-command-not-fail-open"
                && result.severity() == Severity::Error
                && result.title() == "contract-critical hook command is fail-open"
        }),
        "fail-open contract-critical command should be reported; got {results:#?}"
    );
}
