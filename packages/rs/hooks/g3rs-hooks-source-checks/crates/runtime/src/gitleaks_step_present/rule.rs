use crate::compat::{G3CheckResult, G3Severity};
use hook_shell_parser::command_query::{ResolvedCommand, any_resolved_command};
use hook_shell_parser::types::ParsedShellScript;

use crate::inputs::RustHookCommandInput;

/// `ID` constant.
const ID: &str = "g3rs-hooks/gitleaks-step-present";

/// `check` function.
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
            // Reason: gitleaks is a required shared inline check (see plan); its absence
            // must gate the commit, so `g3rs validate --family hooks` exits non-zero.
            G3Severity::Error,
            "gitleaks step missing".to_owned(),
            "Hook does not execute gitleaks.".to_owned(),
            Some(input.rel_path.to_owned()),
            None,
            false,
        ));
    }
}

/// `script_contains_gitleaks` function.
fn script_contains_gitleaks(parsed: &ParsedShellScript) -> bool {
    any_resolved_command(parsed, is_gitleaks_command)
}

/// `is_gitleaks_command` function.
fn is_gitleaks_command(command: &ResolvedCommand) -> bool {
    command.command_name() == "gitleaks"
        && !command
            .args()
            .iter()
            .any(|arg| is_help_or_version_flag(arg))
}

/// `is_help_or_version_flag` function.
fn is_help_or_version_flag(token: &str) -> bool {
    matches!(token, "-h" | "--help" | "-V" | "--version")
}
