use std::collections::{BTreeMap, BTreeSet};

use crate::compat::{G3CheckResult, G3Severity};
use crate::inputs::RustHookCommandInput;
use crate::support::{args_have_help_or_version, cargo_subcommand_tail};
use g3rs_hooks_contract_types::G3HookCommandRequirement;
use hook_shell_parser::command_query::{
    ResolvedCommand, any_resolved_command, any_resolved_command_relaxed,
};

const ID: &str = "g3rs-hooks/required-contract-command-present";

pub(crate) fn check(input: &RustHookCommandInput<'_>, results: &mut Vec<G3CheckResult>) {
    let required = required_commands_by_owner(input);

    for (requirement, owners) in required {
        let owner_text = owners.into_iter().collect::<Vec<_>>().join(", ");
        let found = command_requirement_present(input, requirement);
        if found {
            results.push(
                G3CheckResult::from_parts(
                    ID.to_owned(),
                    G3Severity::Info,
                    "hook contract command is present".to_owned(),
                    format!(
                        "`{}` executes a command satisfying `{}`. Owner families: {}.",
                        input.rel_path,
                        requirement_label(requirement),
                        owner_text
                    ),
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
                "hook contract command is missing".to_owned(),
                format!(
                    "`{}` does not execute a command satisfying `{}`. Owner families: {}. Family hook contracts require this command; comments, echo text, and docs do not count.",
                    input.rel_path,
                    requirement_label(requirement),
                    owner_text
                ),
                Some(input.rel_path.to_owned()),
                None,
                false,
            ));
        }
    }
}

fn required_commands_by_owner(
    input: &RustHookCommandInput<'_>,
) -> BTreeMap<G3HookCommandRequirement, BTreeSet<String>> {
    let mut commands = BTreeMap::new();
    for requirement in input.requirements {
        for command in &requirement.required_commands {
            let owners = commands.entry(*command).or_insert_with(BTreeSet::new);
            let _ = owners.insert(requirement.owner_family.clone());
        }
    }
    commands
}

fn command_requirement_present(
    input: &RustHookCommandInput<'_>,
    requirement: G3HookCommandRequirement,
) -> bool {
    if requirement == G3HookCommandRequirement::CargoDupesExcludeTests {
        return crate::cargo_dupes_excludes::script_contains_cargo_dupes_with_exclude_tests(
            input.parsed,
        );
    }
    let predicate = |command: &ResolvedCommand| command_satisfies_requirement(command, requirement);
    if matches!(
        requirement,
        G3HookCommandRequirement::CargoDupes | G3HookCommandRequirement::CargoDupesExcludeTests
    ) {
        return any_resolved_command_relaxed(input.parsed, predicate);
    }
    any_resolved_command(input.parsed, predicate)
}

fn command_satisfies_requirement(
    command: &ResolvedCommand,
    requirement: G3HookCommandRequirement,
) -> bool {
    match requirement {
        G3HookCommandRequirement::CargoFmtCheck => {
            cargo_subcommand_has_arg(command, "fmt", "--check")
        }
        G3HookCommandRequirement::CargoClippyDenyWarnings => cargo_clippy_denies_warnings(command),
        G3HookCommandRequirement::CargoDenyCheck => cargo_deny_check(command),
        G3HookCommandRequirement::ConcreteLockfileCommand => concrete_lockfile_command(command),
        G3HookCommandRequirement::CargoTest => cargo_subcommand(command, "test"),
        G3HookCommandRequirement::CargoMachete => {
            cargo_subcommand(command, "machete") || binary_command(command, "cargo-machete")
        }
        G3HookCommandRequirement::CargoDupes => {
            cargo_dupes_command(command).is_some_and(|args| !args_have_help_or_version(args))
        }
        G3HookCommandRequirement::CargoDupesExcludeTests => cargo_dupes_command(command)
            .is_some_and(|args| {
                !args_have_help_or_version(args) && args.iter().any(|arg| arg == "--exclude-tests")
            }),
        G3HookCommandRequirement::Gitleaks => binary_command(command, "gitleaks"),
        G3HookCommandRequirement::G3RsValidatePath => g3rs_validate_path(command),
    }
}

fn binary_command(command: &ResolvedCommand, name: &str) -> bool {
    command.command_name() == name && !args_have_help_or_version(command.args())
}

fn cargo_subcommand(command: &ResolvedCommand, subcommand: &str) -> bool {
    cargo_subcommand_tail(command, subcommand).is_some_and(|args| !args_have_help_or_version(args))
}

fn cargo_subcommand_has_arg(
    command: &ResolvedCommand,
    subcommand: &str,
    required_arg: &str,
) -> bool {
    cargo_subcommand_tail(command, subcommand).is_some_and(|args| {
        !args_have_help_or_version(args) && args.iter().any(|arg| arg == required_arg)
    })
}

fn cargo_clippy_denies_warnings(command: &ResolvedCommand) -> bool {
    let Some(args) = cargo_subcommand_tail(command, "clippy") else {
        return false;
    };
    if args_have_help_or_version(args) {
        return false;
    }
    args.windows(2).any(|window| {
        window.first().is_some_and(|arg| arg == "-D")
            && window.get(1).is_some_and(|arg| arg == "warnings")
    }) || args
        .iter()
        .any(|arg| arg == "-Dwarnings" || arg == "--deny=warnings")
}

fn cargo_deny_check(command: &ResolvedCommand) -> bool {
    if command.command_name() == "cargo-deny" {
        return command.args().first().is_some_and(|arg| arg == "check")
            && !args_have_help_or_version(command.args());
    }
    cargo_subcommand_tail(command, "deny").is_some_and(|args| {
        args.first().is_some_and(|arg| arg == "check") && !args_have_help_or_version(args)
    })
}

fn cargo_dupes_command(command: &ResolvedCommand) -> Option<&[String]> {
    if command.command_name() == "cargo-dupes" {
        return Some(command.args());
    }
    cargo_subcommand_tail(command, "dupes")
}

fn g3rs_validate_path(command: &ResolvedCommand) -> bool {
    if command.command_name() != "g3rs" {
        return false;
    }

    let args = command.args();
    if args
        .first()
        .is_some_and(|token| token.starts_with('-') || is_help_or_version_flag(token))
    {
        return false;
    }

    if args.first().map(String::as_str) != Some("validate") {
        return false;
    }

    parse_validate_args(&args[1..])
}

fn parse_validate_args(args: &[String]) -> bool {
    let mut saw_path = false;
    let mut index = 0usize;
    while let Some(arg) = args.get(index).map(String::as_str) {
        if is_help_or_version_flag(arg) {
            return false;
        }
        if let Some(path) = arg.strip_prefix("--path=") {
            if path.is_empty() || path.starts_with('-') {
                return false;
            }
            saw_path = true;
            index += 1;
            continue;
        }
        if arg == "--path" {
            let Some(value) = args.get(index + 1).map(String::as_str) else {
                return false;
            };
            if value.starts_with('-') {
                return false;
            }
            saw_path = true;
            index += 2;
            continue;
        }
        if let Some(value) = arg.strip_prefix("--family=") {
            if value.is_empty() {
                return false;
            }
            index += 1;
            continue;
        }
        if arg == "--family" {
            let Some(value) = args.get(index + 1).map(String::as_str) else {
                return false;
            };
            if value.starts_with('-') {
                return false;
            }
            index += 2;
            continue;
        }
        if arg == "--inventory" {
            index += 1;
            continue;
        }
        return false;
    }
    saw_path
}

fn concrete_lockfile_command(command: &ResolvedCommand) -> bool {
    command.command_name() == "pnpm"
        && matches!(
            command.args().first().map(String::as_str),
            Some("install" | "i")
        )
        && command.args().iter().any(|arg| arg == "--frozen-lockfile")
}

fn is_help_or_version_flag(token: &str) -> bool {
    matches!(token, "-h" | "--help" | "-V" | "--version")
}

fn requirement_label(requirement: G3HookCommandRequirement) -> &'static str {
    match requirement {
        G3HookCommandRequirement::CargoFmtCheck => "cargo fmt --check",
        G3HookCommandRequirement::CargoClippyDenyWarnings => "cargo clippy -D warnings",
        G3HookCommandRequirement::CargoDenyCheck => "cargo deny check",
        G3HookCommandRequirement::ConcreteLockfileCommand => "pnpm install --frozen-lockfile",
        G3HookCommandRequirement::CargoTest => "cargo test",
        G3HookCommandRequirement::CargoMachete => "cargo machete",
        G3HookCommandRequirement::CargoDupes => "cargo dupes",
        G3HookCommandRequirement::CargoDupesExcludeTests => "cargo dupes --exclude-tests",
        G3HookCommandRequirement::Gitleaks => "gitleaks",
        G3HookCommandRequirement::G3RsValidatePath => "g3rs validate --path",
    }
}

#[cfg(test)]
pub(crate) fn run_case(
    content: &str,
    requirements: Vec<g3rs_hooks_contract_types::G3HookRequirement>,
) -> Vec<guardrail3_check_types::G3CheckResult> {
    let parsed = hook_shell_parser::parse_script(content);
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
        is_workspace_project: true,
        requirements: &requirements,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    crate::compat::finish(results)
}

#[cfg(test)]
#[path = "rule_tests/mod.rs"]
mod rule_tests;
