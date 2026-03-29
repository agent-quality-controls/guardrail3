use super::super::check_source;
use guardrail3_app_rs_family_code_assertions::rs_code_13_todo_macros::{
    assert_findings, RuleFinding,
};

#[test]
fn warns_on_todo_macro() {
    let results = check_source("src/foo.rs", "fn foo() { todo!(); }", false);

    assert_findings(
        &results,
        &[RuleFinding {
            severity: guardrail3_domain_report::Severity::Warn,
            title: "todo! macro",
            message: "`todo!()` macro found: fn foo() { todo!(); }.",
            file: Some("src/foo.rs"),
            line: Some(1),
            inventory: false,
        }],
    );
}
