use crate::compat::{G3CheckResult, G3Severity};
use hook_shell_parser::command_query::{ResolvedCommand, any_resolved_command};

use crate::inputs::ExecutableCommandContextInput;

const ID: &str = "HOOK-SHARED-16";

pub(crate) fn check(input: &ExecutableCommandContextInput<'_>, results: &mut Vec<G3CheckResult>) {
    let found = any_resolved_command(input.parsed, is_file_size_command);

    if found {
        results.push(
            G3CheckResult::from_parts(
                ID.to_owned(),
                G3Severity::Warn,
                "file-size check step present".to_owned(),
                "Hook contains a real file-size check step.".to_owned(),
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
            "file-size check step missing".to_owned(),
            "Hook does not execute a concrete file-size guardrail.".to_owned(),
            Some(input.rel_path.to_owned()),
            None,
            false,
        ));
    }
}

fn is_file_size_command(command: &ResolvedCommand) -> bool {
    (command.command_name() == "git" && command.command_text().contains("cat-file -s"))
        || (matches!(command.command_name(), "stat" | "wc" | "du")
            && (command.command_text().contains(" -c%s")
                || command.command_text().contains(" --bytes")
                || command.command_text().contains(" -c ")))
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
