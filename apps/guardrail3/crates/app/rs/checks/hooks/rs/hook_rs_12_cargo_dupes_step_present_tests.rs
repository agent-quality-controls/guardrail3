use crate::app::rs::checks::hooks::shell::parse_script;

use super::super::inputs::RustHookCommandInput;
use super::check;

#[test]
fn warns_when_cargo_dupes_is_only_prose() {
    let parsed = parse_script("echo \"cargo dupes check\"\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(!results[0].inventory);
}

#[test]
fn passes_when_cargo_dupes_command_exists() {
    let parsed = parse_script("cargo dupes check --exclude-tests\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
}
