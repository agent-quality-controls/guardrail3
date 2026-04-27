use g3rs_code_source_checks_assertions::panic_macro::rule::{
    ExpectedRuleResult, G3Severity, assert_rule_results,
};

#[test]
fn warns_on_panic_macro_in_non_test_code() {
    let content = "fn foo() { panic!(\"boom\"); }";
    let results = super::super::check_source("src/foo.rs", content, false);

    assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            severity: Some(G3Severity::Warn),
            title: Some("panic! macro"),
            file: Some("src/foo.rs"),
            inventory: Some(false),
            message: Some("`panic!()` macro found: fn foo() { panic!(\"boom\"); }."),
            line: Some(1),
        }],
    );
}

#[test]
fn warns_on_qualified_panic_macro_in_non_test_code() {
    let content = "fn foo() { core::panic!(\"boom\"); }";
    let results = super::super::check_source("src/foo.rs", content, false);

    assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            severity: Some(G3Severity::Warn),
            title: Some("panic! macro"),
            file: Some("src/foo.rs"),
            inventory: Some(false),
            message: Some("`panic!()` macro found: fn foo() { core::panic!(\"boom\"); }."),
            line: Some(1),
        }],
    );
}
