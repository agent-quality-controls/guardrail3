use crate::domain::report::Severity;

use super::super::inputs::UnsafeCodeLintInput;
use super::check;

#[test]
fn inventories_forbid_level() {
    let input = UnsafeCodeLintInput {
        cargo_rel_path: "Cargo.toml",
        lint_level: Some("forbid"),
    };
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id, "RS-CODE-12");
    assert_eq!(result.severity, Severity::Info);
    assert!(result.inventory);
}

#[test]
fn errors_on_deny_level() {
    let input = UnsafeCodeLintInput {
        cargo_rel_path: "Cargo.toml",
        lint_level: Some("deny"),
    };
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id, "RS-CODE-12");
    assert_eq!(result.severity, Severity::Error);
    assert!(!result.inventory);
}
