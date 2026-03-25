use crate::hook_shell::parse_script;
use guardrail3_domain_report::Severity;

use super::super::facts::HookScriptKind;
use super::super::inputs::ExecutableCommandContextInput;
use super::check;

#[test]
fn warns_when_file_size_only_appears_in_comment() {
    let content = "# git cat-file -s :$file\n";
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
fn passes_when_assignment_runs_git_cat_file_size() {
    let content = r#"file_size=$(git cat-file -s ":$file")"#;
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

#[test]
fn warns_when_only_max_file_size_threshold_is_referenced() {
    let content = r#"
MAX_FILE_SIZE=1048576
if [ "$file_size" -gt "$MAX_FILE_SIZE" ]; then
    exit 1
fi
"#;
    let parsed = parse_script(content);
    let input = ExecutableCommandContextInput {
        rel_path: ".githooks/pre-commit",
        kind: HookScriptKind::PreCommit,
        content,
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results[0].severity, Severity::Warn);
    assert!(!results[0].inventory);
}

#[test]
fn warns_when_git_cat_file_size_is_only_echoed() {
    let content = r#"echo "git cat-file -s :$file""#;
    let parsed = parse_script(content);
    let input = ExecutableCommandContextInput {
        rel_path: ".githooks/pre-commit",
        kind: HookScriptKind::PreCommit,
        content,
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results[0].severity, Severity::Warn);
    assert!(!results[0].inventory);
}
