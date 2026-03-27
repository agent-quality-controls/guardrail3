use guardrail3_domain_report::{CheckResult, Severity};

const RULE_ID: &str = "RS-TOOLCHAIN-03";

#[derive(Debug)]
pub struct ExpectedRuleResult<'a> {
    pub severity: Severity,
    pub inventory: bool,
    pub title: &'a str,
    pub message: &'a str,
    pub file: Option<&'a str>,
}

pub fn assert_rule_results(results: &[CheckResult], expected: &[ExpectedRuleResult<'_>]) {
    let actual = results
        .iter()
        .filter(|result| result.id == RULE_ID)
        .collect::<Vec<_>>();
    assert_eq!(
        actual.len(),
        expected.len(),
        "unexpected {RULE_ID} results: {results:#?}"
    );

    for expected_result in expected {
        let matched = actual.iter().any(|result| {
            result.severity == expected_result.severity
                && result.inventory == expected_result.inventory
                && result.title == expected_result.title
                && result.message == expected_result.message
                && result.file.as_deref() == expected_result.file
        });
        assert!(
            matched,
            "missing expected {RULE_ID} result: {expected_result:#?}\nactual: {actual:#?}"
        );
    }
}
