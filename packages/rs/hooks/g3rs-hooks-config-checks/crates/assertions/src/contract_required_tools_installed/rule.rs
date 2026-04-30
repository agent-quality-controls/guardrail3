crate::define_result_assertions!("g3rs-hooks/contract-required-tools-installed");

pub fn assert_all_inventory(results: &[guardrail3_check_types::G3CheckResult]) {
    let findings = findings(results);
    assert!(!findings.is_empty(), "expected contract tool findings");
    assert!(
        results
            .iter()
            .filter(|result| result.id() == "g3rs-hooks/contract-required-tools-installed")
            .all(guardrail3_check_types::G3CheckResult::inventory),
        "all installed-tool findings should be inventory; got {findings:#?}"
    );
}

pub fn assert_missing_tool(results: &[guardrail3_check_types::G3CheckResult], tool: &str) {
    assert_contains(
        results,
        error(
            &format!("{tool} missing for hook contract"),
            &format!(
                "{tool} is required by a family hook contract but is not available on PATH or via a path-qualified command."
            ),
            ".githooks/pre-commit",
        ),
    );
}
