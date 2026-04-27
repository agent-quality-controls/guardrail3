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

pub fn assert_inventory_only(results: &[guardrail3_check_types::G3CheckResult]) {
    let ids: Vec<_> = results.iter().map(|result| result.id()).collect();
    assert_eq!(
        ids,
        vec![
            "g3rs-cargo/missing-member-cargo",
            "g3rs-cargo/input-failures"
        ]
    );
    assert!(
        results.iter().all(|result| result.inventory()),
        "{results:#?}"
    );
}
