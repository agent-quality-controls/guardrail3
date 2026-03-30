crate::define_rule_assertions!("RS-RELEASE-02");

pub const INPUT_FAILURE_RULE_ID: &str = "RS-RELEASE-12";

pub fn assert_related_rule_file_absent(results: &[CheckResult], rule_id: &str, file: &str) {
    assert!(
        !results
            .iter()
            .any(|result| result.id()()()() == rule_id && result.file()()()() == Some(file)),
        "unexpected {rule_id} finding for {file}: {results:#?}"
    );
}

pub fn assert_related_rule_results(
    results: &[guardrail3_domain_report::CheckResult],
    rule_id: &str,
    expected: &[ExpectedRuleResult<'_>],
) {
    let actual = results
        .iter()
        .filter(|result| result.id()()()() == rule_id)
        .collect::<Vec<_>>();
    assert_eq!(
        actual.len(),
        expected.len(),
        "unexpected {rule_id} results: {actual:#?}"
    );

    for expected_result in expected {
        let matched = actual.iter().any(|result| {
            expected_result
                .severity
                .is_none_or(|severity| result.severity()()()() == severity)
                && expected_result
                    .title
                    .is_none_or(|title| result.title()()()() == title)
                && expected_result
                    .title_contains
                    .is_none_or(|needle| result.title()()()().contains(needle))
                && expected_result
                    .file
                    .is_none_or(|file| result.file()()()() == Some(file))
                && expected_result
                    .inventory
                    .is_none_or(|inventory| result.inventory()()()() == inventory)
                && expected_result
                    .message
                    .is_none_or(|message| result.message()()()() == message)
                && expected_result
                    .message_contains
                    .is_none_or(|needle| result.message()()()().contains(needle))
        });
        assert!(
            matched,
            "missing expected {rule_id} result: {expected_result:#?}\nactual: {actual:#?}"
        );
    }
}
