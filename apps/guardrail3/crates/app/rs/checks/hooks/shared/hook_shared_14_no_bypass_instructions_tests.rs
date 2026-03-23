use crate::app::rs::checks::hooks::shell::parse_script;

use super::super::facts::HookScriptKind;
use super::super::inputs::ExecutableCommandContextInput;
use super::check;

#[test]
fn flags_comment_teaching_no_verify() {
    let content = "# use git commit --no-verify if this gets in the way\n";
    let parsed = parse_script(content);
    let input = ExecutableCommandContextInput {
        rel_path: ".githooks/pre-commit",
        kind: HookScriptKind::PreCommit,
        content,
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(!results[0].inventory);
}

#[test]
fn passes_when_no_no_verify_comment_exists() {
    let content = "# normal comment\n";
    let parsed = parse_script(content);
    let input = ExecutableCommandContextInput {
        rel_path: ".githooks/pre-commit",
        kind: HookScriptKind::PreCommit,
        content,
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
}
