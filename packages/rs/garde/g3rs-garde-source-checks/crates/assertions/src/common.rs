use guardrail3_check_types::{G3CheckResult, G3Severity};

#[derive(Clone, Copy, Debug, Default)]
pub struct ExpectedRuleResult<'a> {
    pub severity: Option<G3Severity>,
    pub title: Option<&'a str>,
    pub title_contains: Option<&'a str>,
    pub file: Option<&'a str>,
    pub inventory: Option<bool>,
    pub message: Option<&'a str>,
    pub message_contains: Option<&'a str>,
    pub line: Option<usize>,
}

pub(crate) fn rule_results<'a>(
    results: &'a [G3CheckResult],
    rule_id: &str,
) -> Vec<&'a G3CheckResult> {
    results.iter().filter(|result| result.id() == rule_id).collect()
}

pub(crate) fn assert_rule_results(
    results: &[G3CheckResult],
    rule_id: &str,
    expected: &[ExpectedRuleResult<'_>],
) {
    let actual = rule_results(results, rule_id);
    assert_eq!(
        actual.len(),
        expected.len(),
        "unexpected {rule_id} results: {actual:#?}"
    );

    for expected_result in expected {
        let matched = actual.iter().any(|result| {
            expected_result
                .severity
                .is_none_or(|severity| result.severity() == severity)
                && expected_result
                    .title
                    .is_none_or(|title| result.title() == title)
                && expected_result
                    .title_contains
                    .is_none_or(|needle| result.title().contains(needle))
                && expected_result
                    .file
                    .is_none_or(|file| result.file() == Some(file))
                && expected_result
                    .inventory
                    .is_none_or(|inventory| result.inventory() == inventory)
                && expected_result
                    .message
                    .is_none_or(|message| result.message() == message)
                && expected_result
                    .message_contains
                    .is_none_or(|needle| result.message().contains(needle))
                && expected_result
                    .line
                    .is_none_or(|line| result.line() == Some(line))
        });
        assert!(
            matched,
            "missing expected {rule_id} result: {expected_result:#?}\nactual: {actual:#?}"
        );
    }
}

#[macro_export]
macro_rules! define_rule_assertions {
    ($rule_id:literal) => {
        pub use crate::common::ExpectedRuleResult;
        pub use guardrail3_check_types::{G3CheckResult, G3Severity};

        const RULE_ID: &str = $rule_id;

        pub fn assert_rule_results(results: &[G3CheckResult], expected: &[ExpectedRuleResult<'_>]) {
            crate::common::assert_rule_results(results, RULE_ID, expected);
        }
    };
}
