use super::helpers::check_source;
use g3rs_code_ast_checks_assertions::rs_code_13_todo_macros::{
    ExpectedRuleResult, G3Severity, assert_rule_results,
};

#[test]
fn warns_on_todo_macro() {
    let results = check_source("src/foo.rs", "fn foo() { todo!(); }", false);

    assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            severity: Some(G3Severity::Warn),
            title: Some("todo! macro"),
            file: Some("src/foo.rs"),
            inventory: Some(false),
            message: Some("`todo!()` macro found: fn foo() { todo!(); }."),
            line: Some(1),
        }],
    );
}
