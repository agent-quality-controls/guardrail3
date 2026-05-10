use guardrail3_check_types::G3CheckResult;

/// Asserts that `results` contains zero non-inventory entries.
///
/// # Panics
///
/// Panics when at least one non-inventory result is present.
pub fn assert_no_non_inventory_findings(results: &[G3CheckResult], context: &str) {
    let non_inventory: Vec<_> = results
        .iter()
        .filter(|result| !result.inventory())
        .collect();
    assert!(
        non_inventory.is_empty(),
        "{context}: expected zero non-inventory findings, got {non_inventory:#?}",
    );
}

/// Asserts that `results` contains no entry whose id is `expected`.
///
/// # Panics
///
/// Panics when at least one result with id `expected` is present.
pub fn assert_missing_id(results: &[G3CheckResult], expected: &str) {
    let matched: Vec<_> = results
        .iter()
        .filter(|result| result.id() == expected)
        .collect();
    assert!(
        matched.is_empty(),
        "did not expect any result for `{expected}`, got {matched:#?}",
    );
}
