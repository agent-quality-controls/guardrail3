use guardrail3_app_rs_family_hooks_shared::hook_shell::ParsedShellScript;
use guardrail3_app_rs_family_hooks_shared::hook_shell::command_query::{
    ResolvedCommand, any_resolved_command,
};
use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::RustHookCommandInput;

const ID: &str = "HOOK-RS-11";

pub fn check(input: &RustHookCommandInput<'_>, results: &mut Vec<CheckResult>) {
    let found = script_contains_gitleaks(input.parsed);

    if found {
        results.push(
            CheckResult::from_parts(
                ID.to_owned(),
                Severity::Warn,
                "gitleaks step present".to_owned(),
                "Hook runs gitleaks as an executable command.".to_owned(),
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
        && !command.args().iter().any(|arg| is_help_or_version_flag(arg))
}

fn is_help_or_version_flag(token: &str) -> bool {
    matches!(token, "-h" | "--help" | "-V" | "--version")
}

#[cfg(test)]
pub(super) fn run_case(content: &str) -> Vec<CheckResult> {
    let parsed = test_support::parsed_hook(content);
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    results
}

#[cfg(test)]
#[path = "tests/steps/hook_rs_11_gitleaks_step_present_tests/mod.rs"]
mod hook_rs_11_gitleaks_step_present_tests;
