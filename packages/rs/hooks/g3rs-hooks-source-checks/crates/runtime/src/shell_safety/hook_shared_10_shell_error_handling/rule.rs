use crate::compat::{G3CheckResult, G3Severity};

use crate::inputs::ExecutableCommandContextInput;

const ID: &str = "RS-HOOKS-SOURCE-16";

pub(crate) fn check(input: &ExecutableCommandContextInput<'_>, results: &mut Vec<G3CheckResult>) {
    let has_shell_error_handling = input
        .parsed
        .source_lines
        .iter()
        .any(|line| has_shell_error_handling_line(&line.raw));

    if has_shell_error_handling {
        results.push(
            G3CheckResult::from_parts(
                ID.to_owned(),
                G3Severity::Warn,
                "`.githooks/pre-commit` enables fail-closed shell options".to_owned(),
                "`.githooks/pre-commit` enables `set -e`-style shell error handling before running checks.".to_owned(),
                Some(input.rel_path.to_owned()),
                None,
                false,
            )
            .into_inventory(),
        );
    } else {
        results.push(G3CheckResult::from_parts(
            ID.to_owned(),
            G3Severity::Warn,
            "missing fail-closed shell options in `.githooks/pre-commit`".to_owned(),
            "Add `set -euo pipefail` near the top of `.githooks/pre-commit`, before any real checks run. Without `-e`, a failing command can be ignored and the hook can continue past a broken check.".to_owned(),
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
pub(crate) fn run_case(content: &str) -> Vec<guardrail3_check_types::G3CheckResult> {
    let parsed = hook_shell_parser::parse_script(content);
    let input = ExecutableCommandContextInput {
        rel_path: ".githooks/pre-commit",
        kind: crate::facts::HookScriptKind::PreCommit,
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    crate::compat::finish(results)
}

#[cfg(test)]
#[path = "rule_tests/mod.rs"] // reason: owned sidecar tests for file module.
mod rule_tests;
