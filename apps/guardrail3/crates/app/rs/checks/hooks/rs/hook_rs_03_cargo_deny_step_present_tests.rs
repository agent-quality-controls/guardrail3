use crate::domain::report::Severity;

use super::check;
use super::super::inputs::RustHookCommandInput;
use super::super::test_support::parsed_hook;

#[test]
fn warns_when_cargo_deny_only_appears_in_comment() {
    let parsed = parsed_hook("# cargo deny check\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results[0].severity, Severity::Warn);
    assert!(!results[0].inventory);
}

#[test]
fn passes_when_cargo_deny_command_exists() {
    let parsed = parsed_hook("cargo deny check\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
}

#[test]
fn warns_when_cargo_deny_is_only_echoed() {
    let parsed = parsed_hook("echo \"cargo deny check\"\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results[0].severity, Severity::Warn);
    assert!(!results[0].inventory);
}
