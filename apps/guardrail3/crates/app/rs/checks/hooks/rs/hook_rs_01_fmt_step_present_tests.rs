use crate::domain::report::Severity;

use super::check;
use super::super::inputs::RustHookCommandInput;
use super::super::test_support::parsed_hook;

#[test]
fn warns_when_fmt_only_appears_in_comment() {
    let parsed = parsed_hook("# cargo fmt --all -- --check\n");
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
fn passes_when_fmt_check_command_exists() {
    let parsed = parsed_hook("cargo fmt --all -- --check\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
}

#[test]
fn warns_when_fmt_check_is_only_echoed() {
    let parsed = parsed_hook("echo \"cargo fmt --all -- --check\"\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results[0].severity, Severity::Warn);
    assert!(!results[0].inventory);
}
