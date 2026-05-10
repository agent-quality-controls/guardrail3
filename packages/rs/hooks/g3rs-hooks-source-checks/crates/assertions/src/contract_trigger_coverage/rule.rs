crate::define_rule_assertions!("g3rs-hooks/contract-trigger-coverage");

/// Panics if the expected finding shape is absent.
///
/// # Panics
///
/// Panics if results do not satisfy the assertion.
pub fn assert_missing_pattern_error(results: &[guardrail3_check_types::G3CheckResult]) {
    assert!(
        results.iter().any(|result| {
            !result.inventory()
                && result.id() == "g3rs-hooks/contract-trigger-coverage"
                && result.severity() == Severity::Error
                && result.title() == "hook does not declare a RUST_RELEVANT_PATTERN"
        }),
        "contract trigger coverage should report missing RUST_RELEVANT_PATTERN error; got {results:#?}"
    );
}

/// Panics if the expected finding shape is absent.
///
/// # Panics
///
/// Panics if results do not satisfy the assertion.
pub fn assert_missing_coverage_error(results: &[guardrail3_check_types::G3CheckResult]) {
    assert!(
        results.iter().any(|result| {
            !result.inventory()
                && result.id() == "g3rs-hooks/contract-trigger-coverage"
                && result.severity() == Severity::Error
                && result.title() == "hook contract trigger coverage missing"
        }),
        "contract trigger coverage should report missing-coverage error; got {results:#?}"
    );
}

/// Panics if the expected finding shape is absent.
///
/// # Panics
///
/// Panics if results do not satisfy the assertion.
pub fn assert_coverage_proven_inventory(results: &[guardrail3_check_types::G3CheckResult]) {
    assert!(
        results.iter().any(|result| {
            result.inventory()
                && result.id() == "g3rs-hooks/contract-trigger-coverage"
                && result.title() == "hook contract trigger coverage proven"
        }),
        "contract trigger coverage should emit inventory-proven; got {results:#?}"
    );
}
