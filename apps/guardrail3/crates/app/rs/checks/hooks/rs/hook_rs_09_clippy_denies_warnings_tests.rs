use crate::app::rs::checks::hooks::shell::parse_script;
use crate::domain::report::Severity;

use super::super::inputs::RustHookCommandInput;
use super::check;

#[test]
fn warns_for_clippy_without_deny_warnings() {
    let parsed = parse_script("cargo clippy --workspace\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results[0].severity, Severity::Warn);
}

#[test]
fn passes_for_clippy_with_deny_warnings() {
    let parsed = parse_script("cargo clippy --workspace -- -D warnings\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
}

#[test]
fn warns_when_deny_warnings_only_appears_inside_echo() {
    let parsed = parse_script("echo \"cargo clippy --workspace -- -D warnings\"\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results[0].severity, Severity::Warn);
    assert!(!results[0].inventory);
}
