pub fn assert_precomputed_dispatch(results: &[guardrail3_check_types::G3CheckResult]) {
    let ids = results.iter().map(|result| result.id()).collect::<Vec<_>>();
    assert_eq!(
        ids,
        vec![
            "RS-TOPOLOGY-FILETREE-07",
            "RS-TOPOLOGY-FILETREE-11",
            "RS-TOPOLOGY-FILETREE-12",
            "RS-TOPOLOGY-FILETREE-13",
            "RS-TOPOLOGY-FILETREE-16",
        ],
        "{results:#?}"
    );
}
