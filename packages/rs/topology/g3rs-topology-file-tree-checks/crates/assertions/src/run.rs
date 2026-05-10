/// Asserts that the runtime emits findings in the documented dispatch order.
///
/// # Panics
/// Panics when the actual finding ids do not match the expected dispatch order.
pub fn assert_precomputed_dispatch(results: &[guardrail3_check_types::G3CheckResult]) {
    let ids = results
        .iter()
        .map(guardrail3_check_types::G3CheckResult::id)
        .collect::<Vec<_>>();
    assert_eq!(
        ids,
        vec![
            "g3rs-topology/required-inputs-fail-closed",
            "g3rs-topology/no-nested-workspaces",
            "g3rs-topology/no-nested-guardrail3-rs-toml",
            "g3rs-topology/declared-workspace-members-only",
            "g3rs-topology/member-paths-must-not-escape-root",
            "g3rs-topology/workspace-local-file-placement",
        ],
        "{results:#?}"
    );
}
