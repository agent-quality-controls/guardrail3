use guardrail3_domain_report::Severity;

use guardrail3_app_rs_family_code_assertions::rs_code_09_file_length::{assert_findings, RuleFinding};
use super::super::check_source;

#[test]
fn errors_when_non_test_file_exceeds_500_effective_lines() {
    let content = "fn x() {}\n".repeat(501);
    assert_findings(
        &check_source("src/foo.rs", &content, false),
        &[RuleFinding {
            severity: Severity::Error,
            title: "file too long",
            message: "501 effective lines (max 500). Long files are hard to review and maintain.",
            file: Some("src/foo.rs"),
            line: None,
            inventory: false,
        }],
    );
}

#[test]
fn skips_non_test_file_at_exact_500_effective_lines() {
    let content = "fn x() {}\n".repeat(500);
    assert_findings(&check_source("src/foo.rs", &content, false), &[]);
}
