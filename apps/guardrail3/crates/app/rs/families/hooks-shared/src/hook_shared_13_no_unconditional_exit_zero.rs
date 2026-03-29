use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::ExecutableCommandContextInput;

const ID: &str = "HOOK-SHARED-13";

pub fn check(input: &ExecutableCommandContextInput<'_>, results: &mut Vec<CheckResult>) {
    if let Some(line) = input
        .parsed
        .executable_lines
        .iter()
        .find(|line| line.is_exit_zero)
    {
        results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Warn,
            title: "unconditional exit 0 bypass present".to_owned(),
            message: "Hook contains an executable `exit 0`, which can mask failures.".to_owned(),
            file: Some(input.rel_path.to_owned()),
            line: Some(line.line_no),
            inventory: false,
        });
    } else {
        results.push(
            CheckResult {
                id: ID.to_owned(),
                severity: Severity::Warn,
                title: "no unconditional exit 0 bypass".to_owned(),
                message: "Hook does not contain an executable `exit 0` bypass.".to_owned(),
                file: Some(input.rel_path.to_owned()),
                line: None,
                inventory: false,
            }
            .as_inventory(),
        );
    }
}

#[cfg(test)]
#[allow(dead_code)]
pub(crate) fn run_case(content: &str) -> Vec<CheckResult> {
    let parsed = crate::hook_shell::parse_script(content);
    let input = ExecutableCommandContextInput {
        rel_path: ".githooks/pre-commit",
        kind: super::facts::HookScriptKind::PreCommit,
        content,
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    results
}

#[cfg(test)]
#[path = "hook_shared_13_no_unconditional_exit_zero_tests/mod.rs"]
mod hook_shared_13_no_unconditional_exit_zero_tests;
