use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::ExecutableCommandContextInput;

const ID: &str = "HOOK-SHARED-14";

pub fn check(input: &ExecutableCommandContextInput<'_>, results: &mut Vec<CheckResult>) {
    for (index, raw_line) in input.content.lines().enumerate() {
        let trimmed = raw_line.trim();
        if !trimmed.starts_with('#') || !trimmed.contains("--no-verify") {
            continue;
        }

        results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Info,
            title: "hook bypass instructions present".to_owned(),
            message: "Hook comments teach `--no-verify`, which weakens the guardrail.".to_owned(),
            file: Some(input.rel_path.to_owned()),
            line: Some(index + 1),
            inventory: false,
        });
        return;
    }

    results.push(
        CheckResult {
            id: ID.to_owned(),
            severity: Severity::Info,
            title: "no hook bypass instructions".to_owned(),
            message: "Hook comments do not teach `--no-verify` bypasses.".to_owned(),
            file: Some(input.rel_path.to_owned()),
            line: None,
            inventory: false,
        }
        .as_inventory(),
    );
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
#[path = "hook_shared_14_no_bypass_instructions_tests/mod.rs"]
mod hook_shared_14_no_bypass_instructions_tests;
