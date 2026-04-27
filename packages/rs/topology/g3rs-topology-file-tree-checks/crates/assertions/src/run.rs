pub fn assert_precomputed_dispatch(results: &[guardrail3_check_types::G3CheckResult]) {
    let ids = results.iter().map(|result| result.id()).collect::<Vec<_>>();
    assert_eq!(
        ids,
        vec![
            "g3rs-topology/required-inputs-fail-closed",
            "g3rs-topology/no-nested-workspaces",
            "g3rs-topology/declared-workspace-members-only",
            "g3rs-topology/member-paths-must-not-escape-root",
            "g3rs-topology/workspace-local-file-placement",
        ],
        "{results:#?}"
    );
}
