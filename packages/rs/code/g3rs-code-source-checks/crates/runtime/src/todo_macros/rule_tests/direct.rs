use g3rs_code_source_checks_assertions::todo_macros::rule::{
    ExpectedRuleResult, G3Severity, assert_rule_results,
};

#[test]
fn warns_on_todo_macro() {
    let results = super::super::check_source("src/foo.rs", "fn foo() { todo!(); }", false);

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

#[test]
fn warns_on_unimplemented_macro() {
    let results = super::super::check_source("src/foo.rs", "fn foo() { unimplemented!(); }", false);

    assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            severity: Some(G3Severity::Warn),
            title: Some("unimplemented! macro"),
            file: Some("src/foo.rs"),
            inventory: Some(false),
            message: Some("`unimplemented!()` macro found: fn foo() { unimplemented!(); }."),
            line: Some(1),
        }],
    );
}

#[test]
fn warns_on_unreachable_macro_in_non_test_code() {
    let results = super::super::check_source("src/foo.rs", "fn foo() { unreachable!(); }", false);

    assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            severity: Some(G3Severity::Warn),
            title: Some("unreachable! macro"),
            file: Some("src/foo.rs"),
            inventory: Some(false),
            message: Some("`unreachable!()` macro found: fn foo() { unreachable!(); }."),
            line: Some(1),
        }],
    );
}
