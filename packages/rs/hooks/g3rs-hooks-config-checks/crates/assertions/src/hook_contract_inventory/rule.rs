use guardrail3_check_types::{G3CheckResult, G3Severity};

/// Assert that the hook contract inventory finding is present.
///
/// # Panics
///
/// Panics when the expected inventory finding is not present.
pub fn assert_contract_loaded(
    results: &[G3CheckResult],
    rule_id: &str,
    owner_family: &str,
    message: &str,
) {
    let title = format!("{owner_family} hook contract loaded");
    assert!(
        results.iter().any(|result| {
            result.id() == rule_id
                && result.severity() == G3Severity::Info
                && result.title() == title
                && result.message() == message
                && result.file() == Some(".githooks/pre-commit")
                && result.inventory()
        }),
        "expected {rule_id} inventory finding for {owner_family}"
    );
}

/// Assert that no hook contract inventory finding is present.
///
/// # Panics
///
/// Panics when any finding exists for the supplied rule id.
pub fn assert_no_contract_inventory(results: &[G3CheckResult], rule_id: &str) {
    assert!(
        results.iter().all(|result| result.id() != rule_id),
        "expected no {rule_id} findings"
    );
}
