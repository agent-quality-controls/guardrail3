use crate::compat::{G3CheckResult, G3Severity};
use hook_shell_parser::command_query::{ResolvedCommand, any_resolved_command};

use crate::inputs::RustHookCommandInput;
use crate::support::{args_have_help_or_version, cargo_subcommand_tail};

const ID: &str = "g3rs-hooks/fmt-step-present";

pub(crate) fn check(input: &RustHookCommandInput<'_>, results: &mut Vec<G3CheckResult>) {
    let found = any_resolved_command(input.parsed, is_cargo_fmt_check_command);

    push_presence_result(
        found,
        input.rel_path,
        "cargo fmt --check step present",
        "Hook runs cargo fmt in check mode.",
        "cargo fmt --check step missing",
        "Hook does not execute `cargo fmt ... --check`.",
        results,
    );
}

fn push_presence_result(
    found: bool,
    rel_path: &str,
    ok_title: &str,
    ok_message: &str,
    missing_title: &str,
    missing_message: &str,
    results: &mut Vec<G3CheckResult>,
) {
    if found {
        results.push(
            G3CheckResult::from_parts(
                ID.to_owned(),
                G3Severity::Warn,
                ok_title.to_owned(),
                ok_message.to_owned(),
                Some(rel_path.to_owned()),
                None,
                false,
            )
            .into_inventory(),
        );
    } else {
        results.push(G3CheckResult::from_parts(
            ID.to_owned(),
            G3Severity::Warn,
            missing_title.to_owned(),
            missing_message.to_owned(),
            Some(rel_path.to_owned()),
            None,
            false,
        ));
    }
}

fn is_cargo_fmt_check_command(command: &ResolvedCommand) -> bool {
    let Some(args) = cargo_subcommand_tail(command, "fmt") else {
        return false;
    };

    !args_have_help_or_version(args) && args.iter().any(|arg| arg == "--check")
}

#[cfg(test)]
pub(crate) fn run_case(content: &str) -> Vec<guardrail3_check_types::G3CheckResult> {
    let parsed = hook_shell_parser::parse_script(content);
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
        is_workspace_project: true,
        requirements: &[],
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    crate::compat::finish(results)
}

#[cfg(test)]
#[path = "rule_tests/mod.rs"] // reason: owned sidecar tests for file module.
mod rule_tests;
