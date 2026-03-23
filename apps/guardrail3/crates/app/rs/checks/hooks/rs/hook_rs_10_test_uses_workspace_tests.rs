use crate::app::rs::checks::hooks::shell::parse_script;
use crate::domain::report::Severity;

use super::super::inputs::RustHookCommandInput;
use super::check;

#[test]
fn reports_info_when_workspace_flag_missing() {
    let parsed = parse_script("cargo test\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results[0].severity, Severity::Info);
    assert!(!results[0].inventory);
}

#[test]
fn passes_when_workspace_flag_exists() {
    let parsed = parse_script("cargo test --workspace\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
}

#[test]
fn does_not_count_echoed_workspace_command() {
    let parsed = parse_script("echo \"cargo test --workspace\"\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results[0].severity, Severity::Info);
    assert!(!results[0].inventory);
}
