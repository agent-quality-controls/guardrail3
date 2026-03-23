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
    assert_eq!(results[0].severity, Severity::Error);
}
