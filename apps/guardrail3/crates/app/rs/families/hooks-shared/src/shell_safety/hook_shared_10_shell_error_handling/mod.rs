use guardrail3_domain_report::{CheckResult, Severity};

use crate::inputs::ExecutableCommandContextInput;

const ID: &str = "HOOK-SHARED-10";

pub fn check(input: &ExecutableCommandContextInput<'_>, results: &mut Vec<CheckResult>) {
    let has_shell_error_handling = input.content.lines().any(has_shell_error_handling_line);

    if has_shell_error_handling {
        results.push(
            CheckResult::from_parts(
                ID.to_owned(),
                Severity::Warn,
                "shell error handling present".to_owned(),
                "Hook enables shell error handling with `set -e` or equivalent.".to_owned(),
                Some(input.rel_path.to_owned()),
                None,
                false,
            )
            .as_inventory(),
        );
    } else {
        results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Warn,
            "shell error handling missing".to_owned(),
            "Hook does not enable `set -e`-style shell error handling.".to_owned(),
            Some(input.rel_path.to_owned()),
            None,
            false,
        ));
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
        kind: crate::facts::HookScriptKind::PreCommit,
        content,
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    results
}

#[cfg(test)]

mod tests;
