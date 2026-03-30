use guardrail3_domain_report::CheckResult;

pub use guardrail3_app_rs_family_cargo_assertions_common::check_results;

#[derive(Debug)]
pub struct ExpectedRuleResult<'a> {
    pub file: Option<&'a str>,
    pub title: Option<&'a str>,
    pub inventory: Option<bool>,
}

const RULE_ID: &str = "RS-CARGO-06";

pub fn rule_results<'a>(results: &'a [CheckResult], _rule_id: &str) -> Vec<&'a CheckResult> {
    results
        .iter()
        .filter(|result| result.id() == RULE_ID)
        .collect()
}

pub fn assert_rule_results(results: &[CheckResult], expected: &[ExpectedRuleResult<'_>]) -> () {
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
                .is_none_or(|file| result.file() == Some(file))
                && expected_result
                    .title
                    .is_none_or(|title| result.title() == title)
                && expected_result
                    .inventory
                    .is_none_or(|inventory| result.inventory() == inventory)
        });
        assert!(
            matched,
            "missing expected {RULE_ID} result: {expected_result:#?}\nactual: {actual:#?}"
        );
    }
}

pub fn assert_result_count(results: &[CheckResult], expected: usize) -> bool {
    rule_results(results, RULE_ID).len() == expected
}
