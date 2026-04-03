use super::helpers::check_source;
use guardrail3_app_rs_family_code_assertions::hygiene::rs_code_13_todo_macros::{
    RuleFinding, assert_findings,
};

#[test]
fn warns_on_todo_macro() {
    let results = check_source("src/foo.rs", "fn foo() { todo!(); }", false);

    assert_findings(
        &results,
        &[RuleFinding::new(
            guardrail3_domain_report::Severity::Warn,
            "todo! macro",
            "`todo!()` macro found: fn foo() { todo!(); }.",
            Some("src/foo.rs"),
            Some(1),
            false,
        )],
    );
}
