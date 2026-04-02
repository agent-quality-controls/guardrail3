use guardrail3_domain_report::{CheckResult, Severity};

use crate::inputs::ExecutableCommandContextInput;

const ID: &str = "HOOK-SHARED-14";

pub fn check(input: &ExecutableCommandContextInput<'_>, results: &mut Vec<CheckResult>) {
    for (index, raw_line) in input.content.lines().enumerate() {
        let trimmed = raw_line.trim();
        if !trimmed.starts_with('#') || !trimmed.contains("--no-verify") {
            continue;
        }

        results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Info,
            "hook bypass instructions present".to_owned(),
            "Hook comments teach `--no-verify`, which weakens the guardrail.".to_owned(),
            Some(input.rel_path.to_owned()),
            Some(index + 1),
            false,
        ));
        return;
    }

    results.push(
        CheckResult::from_parts(
            ID.to_owned(),
            Severity::Info,
            "no hook bypass instructions".to_owned(),
            "Hook comments do not teach `--no-verify` bypasses.".to_owned(),
            Some(input.rel_path.to_owned()),
            None,
            false,
        )
        .as_inventory(),
    );
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
