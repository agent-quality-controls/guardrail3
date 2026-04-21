use crate::compat::{G3CheckResult, G3Severity};
use hook_shell_parser::command_query::{ResolvedCommand, any_resolved_command};

use crate::inputs::RustHookCommandInput;
use crate::support::{args_have_help_or_version, cargo_subcommand_tail};

const ID: &str = "RS-HOOKS-SOURCE-06";

pub(crate) fn check(input: &RustHookCommandInput<'_>, results: &mut Vec<G3CheckResult>) {
    let found = any_resolved_command(input.parsed, is_cargo_test_command);

    if found {
        results.push(
            G3CheckResult::from_parts(
                ID.to_owned(),
                G3Severity::Warn,
                "cargo test step present".to_owned(),
                "Hook runs cargo test.".to_owned(),
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
            "cargo test step missing".to_owned(),
            "Hook does not execute cargo test.".to_owned(),
            Some(input.rel_path.to_owned()),
            None,
            false,
        ));
    }
}

fn is_cargo_test_command(command: &ResolvedCommand) -> bool {
    cargo_subcommand_tail(command, "test").is_some_and(|args| !args_have_help_or_version(args))
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
