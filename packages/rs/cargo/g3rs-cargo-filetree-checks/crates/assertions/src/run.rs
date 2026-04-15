pub fn assert_clean_workspace_filetree(results: &[guardrail3_check_types::G3CheckResult]) {
    assert!(
        results
            .iter()
            .any(|result| result.id() == "RS-CARGO-FILETREE-10" && result.inventory()),
        "{results:#?}"
    );
    assert!(
        results
            .iter()
            .any(|result| result.id() == "RS-CARGO-FILETREE-14" && result.inventory()),
        "{results:#?}"
    );
}

pub fn assert_missing_members_and_input_failures(
    results: &[guardrail3_check_types::G3CheckResult],
) {
    assert!(
        results.iter().any(|result| {
            result.id() == "RS-CARGO-FILETREE-10"
                && result.title() == "declared workspace member missing Cargo.toml"
        }),
        "{results:#?}"
    );
    assert!(
        results.iter().any(|result| {
            result.id() == "RS-CARGO-FILETREE-14"
                && result.title() == "failed to read Cargo configuration"
        }),
        "{results:#?}"
    );
}

pub fn assert_inventory_only(results: &[guardrail3_check_types::G3CheckResult]) {
    let ids: Vec<_> = results.iter().map(|result| result.id()).collect();
    assert_eq!(ids, vec!["RS-CARGO-FILETREE-10", "RS-CARGO-FILETREE-14"]);
    assert!(results.iter().all(|result| result.inventory()), "{results:#?}");
}
