use crate::compat::{G3CheckResult, G3Severity};
use hook_shell_parser::command_query::{ResolvedCommand, any_resolved_command};

use crate::inputs::ExecutableCommandContextInput;

/// `ID` constant.
const ID: &str = "g3rs-hooks/file-size-step-present";

/// `check` function.
pub(crate) fn check(input: &ExecutableCommandContextInput<'_>, results: &mut Vec<G3CheckResult>) {
    let found = any_resolved_command(input.parsed, is_file_size_command);

    if found {
        results.push(
            G3CheckResult::from_parts(
                ID.to_owned(),
                G3Severity::Info,
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
            // Reason: file-size cap is a required shared inline check; its absence must
            // gate the commit.
            G3Severity::Error,
            "file-size check step missing".to_owned(),
            "Hook does not execute a concrete file-size guardrail.".to_owned(),
            Some(input.rel_path.to_owned()),
            None,
            false,
        ));
    }
}

/// `is_file_size_command` function.
fn is_file_size_command(command: &ResolvedCommand) -> bool {
    (command.command_name() == "git" && command.command_text().contains("cat-file -s"))
        || (matches!(command.command_name(), "stat" | "wc" | "du")
            && (command.command_text().contains(" -c%s")
                || command.command_text().contains(" --bytes")
                || command.command_text().contains(" -c ")))
}
