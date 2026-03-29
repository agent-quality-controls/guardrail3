use super::super::check_input_failure;
use guardrail3_app_rs_family_code_assertions::rs_code_30_input_failures::{
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
        &[RuleFinding {
            severity: guardrail3_domain_report::Severity::Error,
            title: "code-family input failure",
            message: "Failed to parse Rust source file: unexpected token",
            file: Some("src/lib.rs"),
            line: None,
            inventory: false,
        }],
    );
}
