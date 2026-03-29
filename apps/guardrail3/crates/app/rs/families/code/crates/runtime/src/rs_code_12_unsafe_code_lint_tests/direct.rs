use super::super::check_unsafe_code_lint;
use guardrail3_app_rs_family_code_assertions::rs_code_12_unsafe_code_lint::{
    RuleFinding, assert_findings, assert_no_hits,
};

#[test]
fn errors_on_deny_level() {
    let results = check_unsafe_code_lint("Cargo.toml", Some("deny"));

    assert_findings(
        &results,
        &[RuleFinding {
            severity: guardrail3_domain_report::Severity::Error,
            title: "unsafe_code should be forbid",
            message: "unsafe_code = deny can be overridden; use forbid in workspace lints.",
            file: Some("Cargo.toml"),
            line: None,
            inventory: false,
        }],
    );
}

#[test]
fn skips_unexpected_workspace_lint_levels() {
    let results = check_unsafe_code_lint("Cargo.toml", Some("warn"));

    assert_no_hits(&results);
}
