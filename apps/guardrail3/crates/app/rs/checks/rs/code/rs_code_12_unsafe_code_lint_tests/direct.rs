use crate::domain::report::Severity;

use super::super::super::inputs::UnsafeCodeLintInput;
use super::super::check;

#[test]
fn errors_on_deny_level() {
    let input = UnsafeCodeLintInput {
        cargo_rel_path: "Cargo.toml",
        lint_level: Some("deny"),
    };
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
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
    let input = UnsafeCodeLintInput {
        cargo_rel_path: "Cargo.toml",
        lint_level: Some("warn"),
    };
    let mut results = Vec::new();

    check(&input, &mut results);

    assert!(results.is_empty());
}
