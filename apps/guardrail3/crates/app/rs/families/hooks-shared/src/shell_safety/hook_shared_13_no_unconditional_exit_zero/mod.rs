use guardrail3_domain_report::{CheckResult, Severity};

use crate::inputs::ExecutableCommandContextInput;

const ID: &str = "HOOK-SHARED-13";

pub fn check(input: &ExecutableCommandContextInput<'_>, results: &mut Vec<CheckResult>) {
    if let Some(line) = input
        .parsed
        .executable_lines()
        .iter()
        .find(|line| line.is_exit_zero())
    {
        results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Warn,
            "unconditional exit 0 bypass present".to_owned(),
            "Hook contains an executable `exit 0`, which can mask failures.".to_owned(),
            Some(input.rel_path.to_owned()),
            Some(line.line_no()),
            false,
        ));
    } else {
        results.push(
            CheckResult::from_parts(
                ID.to_owned(),
                Severity::Warn,
                "no unconditional exit 0 bypass".to_owned(),
                "Hook does not contain an executable `exit 0` bypass.".to_owned(),
                Some(input.rel_path.to_owned()),
                None,
                false,
            )
            .as_inventory(),
        );
    }
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
