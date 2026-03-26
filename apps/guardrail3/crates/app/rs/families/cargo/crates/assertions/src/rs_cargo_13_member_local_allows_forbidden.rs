use guardrail3_domain_project_tree::ProjectTree;
use guardrail3_domain_report::CheckResult;

pub use guardrail3_app_rs_family_cargo_assertions_common::ExpectedRuleResult;

const RULE_ID: &str = "RS-CARGO-13";

pub fn check_results(tree: &ProjectTree) -> Vec<CheckResult> {
    guardrail3_app_rs_family_cargo_assertions_common::check_results(tree)
}

pub fn rule_results<'a>(results: &'a [CheckResult], _rule_id: &str) -> Vec<&'a CheckResult> {
    guardrail3_app_rs_family_cargo_assertions_common::rule_results(results, RULE_ID)
}

pub fn assert_rule_results(results: &[CheckResult], expected: &[ExpectedRuleResult<'_>]) {
    let actual = rule_results(results, RULE_ID);
    assert_eq!(
        actual.len(),
        expected.len(),
        "unexpected {RULE_ID} results: {results:#?}"
    );

    for expected_result in expected {
        let matched = actual.iter().any(|result| {
            expected_result
                .file
                .is_none_or(|file| result.file.as_deref() == Some(file))
                && expected_result
                    .title
                    .is_none_or(|title| result.title == title)
                && expected_result
                    .inventory
                    .is_none_or(|inventory| result.inventory == inventory)
        });
        assert!(
            matched,
            "missing expected {RULE_ID} result: {expected_result:#?}
actual: {actual:#?}"
        );
    }
}

pub fn assert_result_count(results: &[CheckResult], expected: usize) {
    assert_eq!(
        rule_results(results, RULE_ID).len(),
        expected,
        "unexpected {RULE_ID} results: {results:#?}"
    );
}
