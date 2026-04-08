use super::helpers::check_source;
use g3rs_code_ast_checks_assertions::rs_code_36_string_dispatch_cap::{
    ExpectedRuleResult, G3Severity, assert_rule_results,
};

#[test]
fn errors_on_match_with_too_many_string_arms() {
    let arms = (0..11)
        .map(|index| format!("\"v{index}\" => {index},"))
        .collect::<Vec<_>>()
        .join("\n");
    let content =
        format!("pub fn dispatch(value: &str) -> usize {{ match value {{ {arms} _ => 0 }} }}");
    let results = check_source("src/lib.rs", &content, false);

    assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            severity: Some(G3Severity::Error),
            title: Some("string dispatch is too large"),
            file: Some("src/lib.rs"),
            inventory: Some(false),
            message: Some(
                "match site has 11 string-literal branches (cap 10). Replace string dispatch with typed models.",
            ),
            line: Some(1),
        }],
    );
}

#[test]
fn errors_on_if_else_chain_with_too_many_string_branches() {
    let mut chain = String::new();
    for index in 0..11 {
        let prefix = if index == 0 { "if" } else { "else if" };
        chain.push_str(&format!("{prefix} value == \"v{index}\" {{ {index} }} "));
    }
    chain.push_str("else { 0 }");
    let content = format!("pub fn dispatch(value: &str) -> usize {{ {chain} }}");
    let results = check_source("src/lib.rs", &content, false);

    assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            severity: Some(G3Severity::Error),
            title: Some("string dispatch is too large"),
            file: Some("src/lib.rs"),
            inventory: Some(false),
            message: Some(
                "if/else if chain site has 11 string-literal branches (cap 10). Replace string dispatch with typed models.",
            ),
            line: Some(1),
        }],
    );
}
