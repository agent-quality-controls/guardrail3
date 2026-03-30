use super::super::check_unsafe_code_lint;
use guardrail3_app_rs_family_code_assertions::rs_code_12_unsafe_code_lint::{
    RuleFinding, assert_findings, assert_no_hits,
};

#[test]
fn errors_on_deny_level() {
    let results = check_unsafe_code_lint("Cargo.toml", Some("deny"));

    assert_findings(
        &results,
        &[RuleFinding::new(
            guardrail3_domain_report::Severity::Error,
            "unsafe_code should be forbid",
            "unsafe_code = deny can be overridden; use forbid in workspace lints.",
            Some("Cargo.toml"),
            None,
            false,
        )],
    );
}

#[test]
fn skips_unexpected_workspace_lint_levels() {
    let results = check_unsafe_code_lint("Cargo.toml", Some("warn"));

    assert_no_hits(&results);
}
