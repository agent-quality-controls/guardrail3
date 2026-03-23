use crate::domain::report::Severity;

use super::check;
use super::super::inputs::RustHookCommandInput;
use super::super::test_support::parsed_hook;

#[test]
fn warns_when_only_jscpd_exists() {
    let parsed = parsed_hook("jscpd .\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results[0].severity, Severity::Warn);
    assert_eq!(results[0].title, "wrong Rust duplication tool");
}

#[test]
fn passes_when_cargo_dupes_exists() {
    let parsed = parsed_hook("cargo dupes --exclude-tests\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
}

#[test]
fn warns_when_cargo_dupes_is_only_echoed() {
    let parsed = parsed_hook("echo \"cargo dupes --exclude-tests\"\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results[0].severity, Severity::Warn);
    assert_eq!(results[0].title, "Rust duplication tool missing");
}
