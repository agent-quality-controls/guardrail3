use crate::app::rs::checks::hooks::shell::parse_script;
use crate::domain::report::Severity;

use super::super::inputs::DispatcherSyntaxInput;
use super::check;

#[test]
fn reports_inventory_when_modular_dir_is_absent() {
    let parsed = parse_script("");
    let input = DispatcherSyntaxInput {
        rel_path: ".githooks/pre-commit",
        has_modular_dir: false,
        parsed: &parsed,
    };

    let mut results = Vec::new();
    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].severity, Severity::Error);
    assert!(results[0].inventory);
}

#[test]
fn warns_when_dispatcher_targets_lookalike_path() {
    let parsed = parse_script(". .githooks/pre-commit.dummy/10-rust.sh\n");
    let input = DispatcherSyntaxInput {
        rel_path: ".githooks/pre-commit",
        has_modular_dir: true,
        parsed: &parsed,
    };

    let mut results = Vec::new();
    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].severity, Severity::Error);
    assert!(!results[0].inventory);
}

#[test]
fn passes_when_dispatcher_targets_pre_commit_dir() {
    let parsed = parse_script("run-parts .githooks/pre-commit.d\n");
    let input = DispatcherSyntaxInput {
        rel_path: ".githooks/pre-commit",
        has_modular_dir: true,
        parsed: &parsed,
    };

    let mut results = Vec::new();
    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].severity, Severity::Error);
    assert!(results[0].inventory);
}
