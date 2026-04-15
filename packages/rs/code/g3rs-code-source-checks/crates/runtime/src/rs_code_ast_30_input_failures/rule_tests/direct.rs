use g3rs_code_source_checks_assertions::rs_code_ast_30_input_failures::rule::{
    ExpectedRuleResult, G3Severity, assert_rule_results,
};

#[test]
fn emits_code_family_input_failure_on_parse_error() {
    let results = super::super::check_broken_source("src/lib.rs", "fn broken( {", false, false);

    assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            severity: Some(G3Severity::Error),
            title: Some("code-family input failure"),
            file: Some("src/lib.rs"),
            inventory: Some(false),
            message: None,
            line: None,
        }],
    );
}
