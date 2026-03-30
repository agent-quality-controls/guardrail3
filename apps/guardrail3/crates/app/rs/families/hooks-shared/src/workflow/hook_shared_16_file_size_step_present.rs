use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::ExecutableCommandContextInput;

const ID: &str = "HOOK-SHARED-16";

pub fn check(input: &ExecutableCommandContextInput<'_>, results: &mut Vec<CheckResult>) {
    let found = input.parsed.executable_lines().iter().any(|line| {
        (line.command_name() == "git" && line.command_text().contains("git cat-file -s"))
            || (matches!(line.command_name(), "stat" | "wc" | "du")
                && (line.command_text().contains(" -c%s")
                    || line.command_text().contains(" --bytes")
                    || line.command_text().contains(" -c ")))
    });

    if found {
        results.push(
            CheckResult::from_parts(
                ID.to_owned(),
                Severity::Warn,
                "file-size check step present".to_owned(),
                "Hook contains a real file-size check step.".to_owned(),
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
            "file-size check step missing".to_owned(),
            "Hook does not execute a concrete file-size guardrail.".to_owned(),
            Some(input.rel_path.to_owned()),
            None,
            false,
        ));
    }
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
#[path = "hook_shared_16_file_size_step_present_tests/mod.rs"]
mod hook_shared_16_file_size_step_present_tests;
