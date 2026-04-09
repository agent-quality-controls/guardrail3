use super::helpers::check_source;
use g3rs_code_ast_checks_assertions::rs_code_09_too_many_effective_code_lines::{
    ExpectedRuleResult, G3Severity, assert_rule_results,
};

#[test]
fn errors_when_effective_code_lines_exceed_cap() {
    let content = (0..501)
        .map(|i| format!("fn f{i}() {{}}\n"))
        .collect::<String>();

    let results = check_source("src/lib.rs", &content, false);

    assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            severity: Some(G3Severity::Error),
            title: Some("too many effective code lines"),
            file: Some("src/lib.rs"),
            inventory: Some(false),
            message: Some(
                "501 effective code-bearing lines (max 500). Split this file into smaller modules.",
            ),
            line: None,
        }],
    );
}
