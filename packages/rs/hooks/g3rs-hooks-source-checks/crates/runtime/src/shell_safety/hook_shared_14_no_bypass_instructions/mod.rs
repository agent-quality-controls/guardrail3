use crate::compat::{G3CheckResult, G3Severity};

use crate::inputs::ExecutableCommandContextInput;

const ID: &str = "RS-HOOKS-SOURCE-19";

pub(crate) fn check(input: &ExecutableCommandContextInput<'_>, results: &mut Vec<G3CheckResult>) {
    for line in input.parsed.source_lines() {
        let Some(comment) = crate::support::inline_comment_text(line.raw()) else {
            continue;
        };
        if !comment.contains("--no-verify") {
            continue;
        }

        results.push(G3CheckResult::from_parts(
            ID.to_owned(),
            G3Severity::Info,
            "hook bypass instructions present".to_owned(),
            "Hook comments teach `--no-verify`, which weakens the guardrail.".to_owned(),
            Some(input.rel_path.to_owned()),
            Some(line.line_no()),
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
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    crate::compat::finish(results)
}

#[cfg(test)]

mod tests;
