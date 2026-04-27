use crate::compat::{G3CheckResult, G3Severity};
use hook_shell_parser::command_query::{ResolvedCommand, any_resolved_command};

use crate::inputs::RustHookCommandInput;
use crate::support::{args_have_help_or_version, cargo_subcommand_tail};

const ID: &str = "g3rs-hooks/cargo-deny-step-present";

pub(crate) fn check(input: &RustHookCommandInput<'_>, results: &mut Vec<G3CheckResult>) {
    let found = any_resolved_command(input.parsed, is_cargo_deny_check_command);

    if found {
        results.push(
            G3CheckResult::from_parts(
                ID.to_owned(),
                G3Severity::Warn,
                "cargo deny check step present".to_owned(),
                "Hook runs cargo deny check.".to_owned(),
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
            "cargo deny check step missing".to_owned(),
            "Hook does not execute cargo deny check.".to_owned(),
            Some(input.rel_path.to_owned()),
            None,
            false,
        ));
    }
}

fn is_cargo_deny_check_command(command: &ResolvedCommand) -> bool {
    match command.command_name() {
        "cargo" => cargo_subcommand_tail(command, "deny").is_some_and(|args| {
            args.first().map(String::as_str) == Some("check") && !args_have_help_or_version(args)
        }),
        "cargo-deny" => {
            let args = command.args();
            args.first().map(String::as_str) == Some("check") && !args_have_help_or_version(args)
        }
        _ => false,
    }
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
#[path = "rule_tests/mod.rs"] // reason: owned sidecar tests for file module.
mod rule_tests;
