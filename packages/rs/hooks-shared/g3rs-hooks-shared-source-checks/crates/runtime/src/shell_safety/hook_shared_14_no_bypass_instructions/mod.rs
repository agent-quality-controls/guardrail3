use crate::compat::{G3CheckResult, G3Severity};

use crate::inputs::ExecutableCommandContextInput;

const ID: &str = "HOOK-SHARED-14";

pub(crate) fn check(input: &ExecutableCommandContextInput<'_>, results: &mut Vec<G3CheckResult>) {
    for (index, raw_line) in input.content.lines().enumerate() {
        let trimmed = raw_line.trim();
        if !trimmed.starts_with('#') || !trimmed.contains("--no-verify") {
            continue;
        }

        results.push(G3CheckResult::from_parts(
            ID.to_owned(),
            G3Severity::Info,
            "hook bypass instructions present".to_owned(),
            "Hook comments teach `--no-verify`, which weakens the guardrail.".to_owned(),
            Some(input.rel_path.to_owned()),
            Some(index + 1),
            false,
        ));
        return;
    }

    results.push(
        G3CheckResult::from_parts(
            ID.to_owned(),
            G3Severity::Info,
            "no hook bypass instructions".to_owned(),
            "Hook comments do not teach `--no-verify` bypasses.".to_owned(),
            Some(input.rel_path.to_owned()),
            None,
            false,
        )
        .into_inventory(),
    );
}

#[cfg(test)]
pub(crate) fn run_case(content: &str) -> Vec<guardrail3_check_types::G3CheckResult> {
    let parsed = hook_shell_parser::parse_script(content);
    let input = ExecutableCommandContextInput {
        rel_path: ".githooks/pre-commit",
        kind: crate::facts::HookScriptKind::PreCommit,
        content,
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    crate::compat::finish(results)
}

#[cfg(test)]

mod tests;
