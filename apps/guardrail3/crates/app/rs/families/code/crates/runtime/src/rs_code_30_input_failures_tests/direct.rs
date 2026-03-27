use guardrail3_domain_report::Severity;

use guardrail3_app_rs_family_code_assertions::rs_code_30_input_failures::{assert_normalized_len, findings};
use super::super::check_input_failure;

#[test]
fn emits_exact_error_for_direct_input_failure_surface() {
    let binding = check_input_failure("src/lib.rs", "Failed to parse Rust source file: unexpected token");
    let results = findings(&binding);

    assert_normalized_len(&results, 1);
    let result = &results[0];
    assert_eq!(result.id, "RS-CODE-30");
    assert_eq!(result.severity, Severity::Error);
    assert_eq!(result.title, "code-family input failure");
    assert_eq!(result.file.as_deref(), Some("src/lib.rs"));
    assert_eq!(result.line, None);
    assert_eq!(
        result.message,
        "Failed to parse Rust source file: unexpected token"
    );
    assert!(!result.inventory);
}
