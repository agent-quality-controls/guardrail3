use super::super::check_source;
use guardrail3_app_rs_family_code_assertions::rs_code_16_panic_macro::{
    RuleFinding, assert_findings,
};

#[test]
fn warns_on_panic_macro_in_non_test_code() {
    let content = "fn foo() { panic!(\"boom\"); }";
    let results = check_source("src/foo.rs", content, false);

    assert_findings(
        &results,
        &[RuleFinding::new(
            guardrail3_domain_report::Severity::Warn,
            "panic! macro",
            "`panic!()` macro found: fn foo() { panic!(\"boom\"); }.",
            Some("src/foo.rs"),
            Some(1),
            false,
        )],
    );
}

#[test]
fn warns_on_qualified_panic_macro_in_non_test_code() {
    let content = "fn foo() { core::panic!(\"boom\"); }";
    let results = check_source("src/foo.rs", content, false);

    assert_findings(
        &results,
        &[RuleFinding::new(
            guardrail3_domain_report::Severity::Warn,
            "panic! macro",
            "`panic!()` macro found: fn foo() { core::panic!(\"boom\"); }.",
            Some("src/foo.rs"),
            Some(1),
            false,
        )],
    );
}
