use hook_shell_parser::ParsedShellScript;
use hook_shell_parser::command_query::{
    ResolvedCommand, any_resolved_command,
};
use crate::compat::{G3CheckResult, G3Severity};

use super::inputs::RustHookCommandInput;

const ID: &str = "HOOK-RS-11";

pub(crate) fn check(input: &RustHookCommandInput<'_>, results: &mut Vec<G3CheckResult>) {
    let found = script_contains_gitleaks(input.parsed);

    if found {
        results.push(
            G3CheckResult::from_parts(
                ID.to_owned(),
                G3Severity::Warn,
                "gitleaks step present".to_owned(),
                "Hook runs gitleaks as an executable command.".to_owned(),
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
            "gitleaks step missing".to_owned(),
            "Hook does not execute gitleaks.".to_owned(),
            Some(input.rel_path.to_owned()),
            None,
            false,
        ));
    }
}

fn script_contains_gitleaks(parsed: &ParsedShellScript<'_>) -> bool {
    any_resolved_command(parsed, is_gitleaks_command)
}

fn is_gitleaks_command(command: &ResolvedCommand) -> bool {
    command.command_name() == "gitleaks"
        && !command
            .args()
            .iter()
            .any(|arg| is_help_or_version_flag(arg))
}

fn is_help_or_version_flag(token: &str) -> bool {
    matches!(token, "-h" | "--help" | "-V" | "--version")
}

#[cfg(test)]
pub(crate) fn run_case(content: &str) -> Vec<guardrail3_check_types::G3CheckResult> {
    let parsed = hook_shell_parser::parse_script(content);
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
        is_workspace_project: true,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    crate::compat::finish(results)
}

#[cfg(test)]

mod tests;
