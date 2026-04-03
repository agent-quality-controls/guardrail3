use super::helpers::check_source;
use guardrail3_app_rs_family_code_assertions::hygiene::rs_code_09_file_length::{
    RuleFinding, Severity, assert_findings,
};

#[test]
fn errors_when_non_test_file_exceeds_500_effective_lines() {
    let content = "fn x() {}\n".repeat(501);
    assert_findings(
        &check_source("src/foo.rs", &content, false),
        &[RuleFinding::new(
            Severity::Error,
            "file too long",
            "501 effective code-bearing lines (max 500). Long files are hard to review and maintain. Split this file into smaller modules.",
            Some("src/foo.rs"),
            None,
            false,
        )],
    );
}

#[test]
fn skips_non_test_file_at_exact_500_effective_lines() {
    let content = "fn x() {}\n".repeat(500);
    assert_findings(&check_source("src/foo.rs", &content, false), &[]);
}
