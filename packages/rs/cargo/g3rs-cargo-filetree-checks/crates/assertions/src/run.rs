/// Assert the result set is an inventory-only workspace file-tree snapshot.
///
/// # Panics
///
/// Panics when the expected inventory ids are absent.
pub fn assert_clean_workspace_filetree(results: &[guardrail3_check_types::G3CheckResult]) {
    assert!(
        results
            .iter()
            .any(|result| result.id() == "g3rs-cargo/missing-member-cargo" && result.inventory()),
        "{results:#?}"
    );
    assert!(
        results
            .iter()
            .any(|result| result.id() == "g3rs-cargo/input-failures" && result.inventory()),
        "{results:#?}"
    );
}

/// Assert that both missing-member and input-failure positive findings fire.
///
/// # Panics
///
/// Panics when either positive finding is missing.
pub fn assert_missing_members_and_input_failures(
    results: &[guardrail3_check_types::G3CheckResult],
) {
    assert!(
        results.iter().any(|result| {
            result.id() == "g3rs-cargo/missing-member-cargo"
                && result.title() == "declared workspace member missing Cargo.toml"
        }),
        "{results:#?}"
    );
    assert!(
        results.iter().any(|result| {
            result.id() == "g3rs-cargo/input-failures"
                && result.title() == "failed to read Cargo configuration"
        }),
        "{results:#?}"
    );
}

/// Assert that exactly the two inventory ids are present and all are inventory.
///
/// # Panics
///
/// Panics when ids differ or any result is non-inventory.
pub fn assert_inventory_only(results: &[guardrail3_check_types::G3CheckResult]) {
    let ids: Vec<_> = results
        .iter()
        .map(guardrail3_check_types::G3CheckResult::id)
        .collect();
    assert_eq!(
        ids,
        vec![
            "g3rs-cargo/missing-member-cargo",
            "g3rs-cargo/input-failures"
        ],
        "unexpected ids: {ids:?}"
    );
    assert!(
        results
            .iter()
            .all(guardrail3_check_types::G3CheckResult::inventory),
        "{results:#?}"
    );
}
