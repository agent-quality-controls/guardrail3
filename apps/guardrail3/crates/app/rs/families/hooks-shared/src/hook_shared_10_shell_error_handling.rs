use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::ExecutableCommandContextInput;

const ID: &str = "HOOK-SHARED-10";

pub fn check(input: &ExecutableCommandContextInput<'_>, results: &mut Vec<CheckResult>) {
    let has_shell_error_handling = input.content.lines().any(has_shell_error_handling_line);

    if has_shell_error_handling {
        results.push(
            CheckResult {
                id: ID.to_owned(),
                severity: Severity::Warn,
                title: "shell error handling present".to_owned(),
                message: "Hook enables shell error handling with `set -e` or equivalent."
                    .to_owned(),
                file: Some(input.rel_path.to_owned()),
                line: None,
                inventory: false,
            }
            .as_inventory(),
        );
    } else {
        results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Warn,
            title: "shell error handling missing".to_owned(),
            message: "Hook does not enable `set -e`-style shell error handling.".to_owned(),
            file: Some(input.rel_path.to_owned()),
            line: None,
            inventory: false,
        });
    }
}

fn has_shell_error_handling_line(line: &str) -> bool {
    let trimmed = line.trim();
    matches!(
        trimmed,
        "set -e" | "set -eu" | "set -eo pipefail" | "set -euo pipefail"
    )
}

#[cfg(test)]
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
#[path = "hook_shared_10_shell_error_handling_tests/mod.rs"]
mod hook_shared_10_shell_error_handling_tests;
