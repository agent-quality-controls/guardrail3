use super::super::check_input_failure;
use guardrail3_app_rs_family_code_assertions::lint_policy::rs_code_30_input_failures::{
    RuleFinding, assert_findings,
};

#[test]
fn emits_exact_error_for_direct_input_failure_surface() {
    let results = check_input_failure(
        "src/lib.rs",
        "Failed to parse Rust source file: unexpected token",
    );

    assert_findings(
        &results,
        &[RuleFinding::new(
            guardrail3_domain_report::Severity::Error,
            "code-family input failure",
            "Failed to parse Rust source file: unexpected token",
            Some("src/lib.rs"),
            None,
            false,
        )],
    );
}
