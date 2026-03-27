use guardrail3_domain_report::Severity;

use guardrail3_app_rs_family_code_assertions::rs_code_12_unsafe_code_lint::{assert_normalized_empty, assert_normalized_len, findings};
use super::super::check_unsafe_code_lint;

#[test]
fn errors_on_deny_level() {
    let raw_results = check_unsafe_code_lint("Cargo.toml", Some("deny"));
    let results = findings(&raw_results);

    assert_normalized_len(&results, 1);
    assert_eq!(results[0].id, "RS-CODE-12");
    assert_eq!(results[0].severity, Severity::Error);
    assert_eq!(results[0].file.as_deref(), Some("Cargo.toml"));
    assert_eq!(results[0].line, None);
    assert_eq!(results[0].title, "unsafe_code should be forbid");
    assert_eq!(
        results[0].message,
        "unsafe_code = deny can be overridden; use forbid in workspace lints."
    );
}

#[test]
fn skips_unexpected_workspace_lint_levels() {
    let raw_results = check_unsafe_code_lint("Cargo.toml", Some("warn"));
    let results = findings(&raw_results);

    assert_normalized_empty(&results);
}
