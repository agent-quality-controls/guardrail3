use std::collections::{BTreeMap, BTreeSet};

use crate::compat::{G3CheckResult, G3Severity};
use crate::inputs::RustHookCommandInput;
use crate::support::{args_have_help_or_version, cargo_subcommand_tail};
use g3rs_hooks_contract_types::G3HookCommandRequirement;
use hook_shell_parser::command_query::{
    ResolvedCommand, any_resolved_command, any_resolved_command_relaxed, shell_words,
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
    if requirement == G3HookCommandRequirement::CargoClippyDenyWarnings {
        return !aliases_shadow_requirement(input.parsed, requirement)
            && crate::clippy_denies_warnings::script_contains_clippy_deny(input.parsed);
    }
    if requirement == G3HookCommandRequirement::CargoDupesExcludeTests {
        return !aliases_shadow_requirement(input.parsed, requirement)
            && crate::cargo_dupes_excludes::script_contains_cargo_dupes_with_exclude_tests(
                input.parsed,
            );
    }
    let predicate = |command: &ResolvedCommand| {
        command_satisfies_requirement(command, requirement)
            && !command_is_shadowed_by_alias(input.parsed, requirement, command)
    };
    if matches!(requirement, G3HookCommandRequirement::CargoDupes) {
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
        G3HookCommandRequirement::CargoDupesExcludeTests => false,
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
            if value.is_empty() || value.starts_with('-') {
                return false;
            }
            saw_path = true;
            index += 2;
            continue;
        }
        if arg.starts_with("--family=") || arg == "--family" {
            return false;
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
    cargo_subcommand_tail(command, "metadata").is_some_and(lockfile_args_are_concrete)
        || cargo_subcommand_tail(command, "update").is_some_and(lockfile_args_are_concrete)
}

fn lockfile_args_are_concrete(args: &[String]) -> bool {
    !args_have_help_or_version(args)
        && args.iter().any(|arg| arg == "--locked")
        && !args
            .iter()
            .any(|arg| arg == "--manifest-path" || arg.starts_with("--manifest-path="))
}

fn aliases_shadow_requirement(
    parsed: &hook_shell_parser::types::ParsedShellScript,
    requirement: G3HookCommandRequirement,
) -> bool {
    alias_shadowed_commands(requirement)
        .iter()
        .any(|command| script_defines_alias(parsed, command))
}

fn command_is_shadowed_by_alias(
    parsed: &hook_shell_parser::types::ParsedShellScript,
    requirement: G3HookCommandRequirement,
    command: &ResolvedCommand,
) -> bool {
    if command.path_qualified() || command.command_text().trim_start().starts_with("command ") {
        return false;
    }
    alias_shadowed_commands(requirement).iter().any(|name| {
        *name == command.command_name()
            && first_alias_definition_line(parsed, name)
                .is_some_and(|line_no| line_no <= command.line_no())
    })
}

fn alias_shadowed_commands(requirement: G3HookCommandRequirement) -> &'static [&'static str] {
    match requirement {
        G3HookCommandRequirement::CargoFmtCheck
        | G3HookCommandRequirement::CargoClippyDenyWarnings
        | G3HookCommandRequirement::CargoDenyCheck
        | G3HookCommandRequirement::ConcreteLockfileCommand
        | G3HookCommandRequirement::CargoTest
        | G3HookCommandRequirement::CargoMachete
        | G3HookCommandRequirement::CargoDupes
        | G3HookCommandRequirement::CargoDupesExcludeTests => {
            &["cargo", "cargo-deny", "cargo-machete", "cargo-dupes"]
        }
        G3HookCommandRequirement::Gitleaks => &["gitleaks"],
        G3HookCommandRequirement::G3RsValidatePath => &["g3rs"],
    }
}

fn script_defines_alias(
    parsed: &hook_shell_parser::types::ParsedShellScript,
    command_name: &str,
) -> bool {
    first_alias_definition_line(parsed, command_name).is_some()
}

fn first_alias_definition_line(
    parsed: &hook_shell_parser::types::ParsedShellScript,
    command_name: &str,
) -> Option<usize> {
    parsed.source_lines.iter().find_map(|line| {
        alias_line_defines_command(line.raw.as_str(), command_name).then_some(line.line_no)
    })
}

fn alias_line_defines_command(raw: &str, command_name: &str) -> bool {
    let words = shell_words(raw);
    words.first().is_some_and(|word| word == "alias")
        && words.iter().skip(1).any(|word| {
            word.split_once('=')
                .is_some_and(|(alias_name, _)| alias_name == command_name)
        })
}

fn is_help_or_version_flag(token: &str) -> bool {
    matches!(token, "-h" | "--help" | "-V" | "--version")
}

fn requirement_label(requirement: G3HookCommandRequirement) -> &'static str {
    match requirement {
        G3HookCommandRequirement::CargoFmtCheck => "cargo fmt --check",
        G3HookCommandRequirement::CargoClippyDenyWarnings => "cargo clippy -D warnings",
        G3HookCommandRequirement::CargoDenyCheck => "cargo deny check",
        G3HookCommandRequirement::ConcreteLockfileCommand => "cargo metadata --locked",
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
