use crate::app::rs::checks::hooks::shell::parse_script;
use crate::domain::report::Severity;

use super::super::inputs::RustHookCommandInput;
use super::check;

#[test]
fn warns_when_only_comment_mentions_guardrail_validation() {
    let parsed = parse_script("# guardrail3 rs validate --staged .\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results[0].severity, Severity::Warn);
}

#[test]
fn passes_when_executable_guardrail_validation_exists() {
    let parsed = parse_script("guardrail3 rs validate --staged .\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results[0].severity, Severity::Info);
}

#[test]
fn warns_when_validate_and_staged_are_split_across_commands() {
    let parsed = parse_script("guardrail3 rs validate .\nguardrail3 fmt --staged .\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results[0].severity, Severity::Warn);
}
