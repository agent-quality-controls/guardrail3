use g3rs_hooks_types::{G3RsHookScriptKind, G3RsHooksSourceChecksInput};
use guardrail3_check_types::G3CheckResult;
use hook_shell_parser::command_query::{ResolvedCommand, any_resolved_command};
use hook_shell_parser::parse_script;

pub fn check(input: &G3RsHooksSourceChecksInput) -> Vec<G3CheckResult> {
    check_single(input, true)
}

pub fn check_all(inputs: &[G3RsHooksSourceChecksInput]) -> Vec<G3CheckResult> {
    let mut results = inputs
        .iter()
        .flat_map(|input| check_single(input, false))
        .collect::<Vec<_>>();

    check_required_contracts_across_selected_surface(inputs, &mut results);

    results
}

fn check_single(
    input: &G3RsHooksSourceChecksInput,
    include_required_contracts: bool,
) -> Vec<G3CheckResult> {
    let kind = match input.kind {
        G3RsHookScriptKind::PreCommit => crate::facts::HookScriptKind::PreCommit,
        G3RsHookScriptKind::Modular => crate::facts::HookScriptKind::Modular,
        G3RsHookScriptKind::G3RsVerifier => crate::facts::HookScriptKind::G3RsVerifier,
    };
    let rust_input = crate::inputs::RustHookCommandInput {
        rel_path: &input.rel_path,
        parsed: &input.parsed,
        is_workspace_project: input.is_workspace_project,
        requirements: &input.requirements,
    };
    let executable_input = crate::inputs::ExecutableCommandContextInput {
        rel_path: &input.rel_path,
        kind,
        parsed: &input.parsed,
    };
    let dispatcher_input = crate::inputs::DispatcherSyntaxInput {
        rel_path: &input.rel_path,
        has_modular_dir: input.has_modular_dir,
        parsed: &input.parsed,
    };
    let fail_open_input = crate::inputs::FailOpenWrapperInput {
        rel_path: &input.rel_path,
        parsed: &input.parsed,
        requirements: &input.requirements,
    };
    let _ = rust_input.is_workspace_project;
    let mut results = Vec::new();

    if input.kind == G3RsHookScriptKind::PreCommit {
        crate::bootstrap::dispatcher_pattern::check(&dispatcher_input, &mut results);
        crate::shell_safety::real_dispatcher_syntax_only::check(&dispatcher_input, &mut results);

        crate::gitleaks_step_present::check(&rust_input, &mut results);
        check_precommit_calls_g3rs_verifier(input, &mut results);
        crate::contract_trigger_coverage::rule::check(&rust_input, &mut results);
        if include_required_contracts {
            crate::required_contract_command_present::rule::check(&rust_input, &mut results);
        }
    }

    if input.kind == G3RsHookScriptKind::G3RsVerifier {
        check_g3rs_verifier_contract(input, &mut results);
    }

    if !input.exists {
        return crate::compat::finish(results);
    }

    crate::shell_safety::shell_error_handling::check(&executable_input, &mut results);
    crate::shell_safety::valid_shebang::check(&executable_input, &mut results);
    if input.exists && input.kind != G3RsHookScriptKind::G3RsVerifier {
        crate::shell_safety::no_unconditional_exit_zero::check(&executable_input, &mut results);
        crate::shell_safety::no_bypass_instructions::check(&executable_input, &mut results);
        crate::workflow::merge_conflict_step_present::check(&executable_input, &mut results);
        crate::workflow::file_size_step_present::check(&executable_input, &mut results);
        crate::shell_safety::executable_command_context_only::check(
            &executable_input,
            &mut results,
        );
        crate::shell_safety::concrete_lockfile_command::check(&executable_input, &mut results);
        crate::shell_safety::no_fail_open_wrappers::check(&fail_open_input, &mut results);
        crate::contract_critical_command_not_fail_open::rule::check(&fail_open_input, &mut results);
    }

    crate::compat::finish(results)
}

const PRECOMMIT_CALLS_G3RS_VERIFIER_ID: &str = "g3rs-hooks/precommit-calls-g3rs-verifier";
const G3RS_VERIFIER_EXISTS_ID: &str = "g3rs-hooks/g3rs-verifier-exists";
const G3RS_VERIFIER_COMMANDS_ID: &str = "g3rs-hooks/g3rs-verifier-required-commands";
const G3RS_VERIFIER_FORBIDDEN_TOOLS_ID: &str = "g3rs-hooks/g3rs-verifier-forbidden-tools";

fn check_precommit_calls_g3rs_verifier(
    input: &G3RsHooksSourceChecksInput,
    results: &mut Vec<crate::compat::G3CheckResult>,
) {
    push(
        any_resolved_command(&input.parsed, is_g3rs_verify_precommit_scope_command),
        PRECOMMIT_CALLS_G3RS_VERIFIER_ID,
        input.rel_path.as_str(),
        "pre-commit calls Rust verifier",
        ".githooks/pre-commit runs scripts/g3rs/verify with --mode pre-commit and --scope.",
        ".githooks/pre-commit does not call Rust verifier",
        ".githooks/pre-commit must run scripts/g3rs/verify --mode pre-commit --scope <scope>.",
        results,
    );
}

fn check_g3rs_verifier_contract(
    input: &G3RsHooksSourceChecksInput,
    results: &mut Vec<crate::compat::G3CheckResult>,
) {
    push(
        input.exists,
        G3RS_VERIFIER_EXISTS_ID,
        input.rel_path.as_str(),
        "Rust verifier script exists",
        "scripts/g3rs/verify exists.",
        "Rust verifier script missing",
        "scripts/g3rs/verify must exist.",
        results,
    );
    if !input.exists {
        return;
    }

    for requirement in REQUIRED_VERIFIER_COMMANDS {
        push(
            any_resolved_command(&input.parsed, requirement.predicate),
            G3RS_VERIFIER_COMMANDS_ID,
            input.rel_path.as_str(),
            requirement.ok_title,
            requirement.ok_message,
            requirement.missing_title,
            requirement.missing_message,
            results,
        );
    }

    push(
        !any_resolved_command(&input.parsed, is_g3ts_command),
        G3RS_VERIFIER_FORBIDDEN_TOOLS_ID,
        input.rel_path.as_str(),
        "Rust verifier does not call g3ts",
        "scripts/g3rs/verify does not call the TypeScript verifier.",
        "Rust verifier calls g3ts",
        "scripts/g3rs/verify must not call g3ts.",
        results,
    );
    push(
        !any_resolved_command(&input.parsed, is_typescript_package_manager_command),
        G3RS_VERIFIER_FORBIDDEN_TOOLS_ID,
        input.rel_path.as_str(),
        "Rust verifier does not call TypeScript package managers",
        "scripts/g3rs/verify does not call pnpm, npm, yarn, or bun.",
        "Rust verifier calls a TypeScript package manager",
        "scripts/g3rs/verify must not call pnpm, npm, yarn, or bun.",
        results,
    );
}

struct VerifierCommandRequirement {
    predicate: fn(&ResolvedCommand) -> bool,
    ok_title: &'static str,
    ok_message: &'static str,
    missing_title: &'static str,
    missing_message: &'static str,
}

const REQUIRED_VERIFIER_COMMANDS: &[VerifierCommandRequirement] = &[
    VerifierCommandRequirement {
        predicate: is_g3rs_validate_scope_command,
        ok_title: "Rust verifier runs g3rs validate",
        ok_message: "scripts/g3rs/verify runs g3rs validate --path \"$SCOPE\".",
        missing_title: "Rust verifier missing g3rs validate",
        missing_message: "scripts/g3rs/verify must run g3rs validate --path \"$SCOPE\".",
    },
    VerifierCommandRequirement {
        predicate: is_cargo_metadata_locked_command,
        ok_title: "Rust verifier runs cargo metadata",
        ok_message: "scripts/g3rs/verify runs cargo metadata --locked.",
        missing_title: "Rust verifier missing cargo metadata",
        missing_message: "scripts/g3rs/verify must run cargo metadata --locked.",
    },
    VerifierCommandRequirement {
        predicate: is_cargo_fmt_all_check_command,
        ok_title: "Rust verifier runs cargo fmt",
        ok_message: "scripts/g3rs/verify runs cargo fmt --all -- --check.",
        missing_title: "Rust verifier missing cargo fmt",
        missing_message: "scripts/g3rs/verify must run cargo fmt --all -- --check.",
    },
    VerifierCommandRequirement {
        predicate: is_cargo_clippy_warnings_command,
        ok_title: "Rust verifier runs clippy with warnings denied",
        ok_message: "scripts/g3rs/verify runs cargo clippy with -D warnings.",
        missing_title: "Rust verifier missing clippy warning denial",
        missing_message: "scripts/g3rs/verify must run cargo clippy --workspace --all-targets --all-features -- -D warnings.",
    },
    VerifierCommandRequirement {
        predicate: is_cargo_deny_check_command,
        ok_title: "Rust verifier runs cargo deny",
        ok_message: "scripts/g3rs/verify runs cargo deny check.",
        missing_title: "Rust verifier missing cargo deny",
        missing_message: "scripts/g3rs/verify must run cargo deny check.",
    },
    VerifierCommandRequirement {
        predicate: is_cargo_machete_command,
        ok_title: "Rust verifier runs cargo machete",
        ok_message: "scripts/g3rs/verify runs cargo machete.",
        missing_title: "Rust verifier missing cargo machete",
        missing_message: "scripts/g3rs/verify must run cargo machete.",
    },
    VerifierCommandRequirement {
        predicate: is_cargo_test_workspace_command,
        ok_title: "Rust verifier runs workspace tests",
        ok_message: "scripts/g3rs/verify runs cargo test --workspace.",
        missing_title: "Rust verifier missing workspace tests",
        missing_message: "scripts/g3rs/verify must run cargo test --workspace.",
    },
    VerifierCommandRequirement {
        predicate: is_cargo_mutants_check_in_place_command,
        ok_title: "Rust verifier runs cargo mutants check",
        ok_message: "scripts/g3rs/verify runs cargo mutants --check --in-place.",
        missing_title: "Rust verifier missing cargo mutants check",
        missing_message: "scripts/g3rs/verify must run cargo mutants --check --in-place.",
    },
    VerifierCommandRequirement {
        predicate: is_cargo_dupes_threshold_command,
        ok_title: "Rust verifier runs cargo dupes with thresholds",
        ok_message: "scripts/g3rs/verify runs cargo dupes check with required thresholds.",
        missing_title: "Rust verifier missing cargo dupes thresholds",
        missing_message: "scripts/g3rs/verify must run cargo dupes check --max-exact 85 --max-exact-percent 10 --exclude-tests.",
    },
];

fn push(
    found: bool,
    id: &str,
    rel_path: &str,
    ok_title: &str,
    ok_message: &str,
    missing_title: &str,
    missing_message: &str,
    results: &mut Vec<crate::compat::G3CheckResult>,
) {
    let result = if found {
        crate::compat::G3CheckResult::from_parts(
            id.to_owned(),
            crate::compat::G3Severity::Warn,
            ok_title.to_owned(),
            ok_message.to_owned(),
            Some(rel_path.to_owned()),
            None,
            false,
        )
        .into_inventory()
    } else {
        crate::compat::G3CheckResult::from_parts(
            id.to_owned(),
            crate::compat::G3Severity::Warn,
            missing_title.to_owned(),
            missing_message.to_owned(),
            Some(rel_path.to_owned()),
            None,
            false,
        )
    };
    results.push(result);
}

fn is_g3rs_verify_precommit_scope_command(command: &ResolvedCommand) -> bool {
    is_g3rs_verify_command(command)
        && args_contain_pair(command.args(), "--mode", "pre-commit")
        && args_contain_flag_with_value(command.args(), "--scope")
}

fn is_g3rs_verify_command(command: &ResolvedCommand) -> bool {
    command.command_name() == "verify"
        && command
            .command_path()
            .trim_matches('"')
            .ends_with("scripts/g3rs/verify")
}

fn is_g3rs_validate_scope_command(command: &ResolvedCommand) -> bool {
    command.command_name() == "g3rs"
        && command.args().first().map(String::as_str) == Some("validate")
        && args_contain_pair(command.args(), "--path", "$SCOPE")
}

fn is_cargo_metadata_locked_command(command: &ResolvedCommand) -> bool {
    let Some(args) = crate::support::cargo_subcommand_tail(command, "metadata") else {
        return false;
    };
    args.iter().any(|arg| arg == "--locked")
}

fn is_cargo_fmt_all_check_command(command: &ResolvedCommand) -> bool {
    let Some(args) = crate::support::cargo_subcommand_tail(command, "fmt") else {
        return false;
    };
    args.windows(2)
        .any(|window| window[0] == "--" && window[1] == "--check")
        && args.iter().any(|arg| arg == "--all")
}

fn is_cargo_clippy_warnings_command(command: &ResolvedCommand) -> bool {
    let Some(args) = crate::support::cargo_subcommand_tail(command, "clippy") else {
        return false;
    };
    args.iter().any(|arg| arg == "--workspace")
        && args.iter().any(|arg| arg == "--all-targets")
        && args.iter().any(|arg| arg == "--all-features")
        && args
            .windows(3)
            .any(|window| window[0] == "--" && window[1] == "-D" && window[2] == "warnings")
}

fn is_cargo_deny_check_command(command: &ResolvedCommand) -> bool {
    let Some(args) = crate::support::cargo_subcommand_tail(command, "deny") else {
        return false;
    };
    args.first().map(String::as_str) == Some("check")
}

fn is_cargo_machete_command(command: &ResolvedCommand) -> bool {
    crate::support::cargo_subcommand_tail(command, "machete").is_some()
}

fn is_cargo_test_workspace_command(command: &ResolvedCommand) -> bool {
    let Some(args) = crate::support::cargo_subcommand_tail(command, "test") else {
        return false;
    };
    args.iter().any(|arg| arg == "--workspace")
}

fn is_cargo_mutants_check_in_place_command(command: &ResolvedCommand) -> bool {
    let Some(args) = crate::support::cargo_subcommand_tail(command, "mutants") else {
        return false;
    };
    args.iter().any(|arg| arg == "--check") && args.iter().any(|arg| arg == "--in-place")
}

fn is_cargo_dupes_threshold_command(command: &ResolvedCommand) -> bool {
    let Some(args) = crate::support::cargo_subcommand_tail(command, "dupes") else {
        return false;
    };
    args.first().map(String::as_str) == Some("check")
        && args_contain_pair(args, "--max-exact", "85")
        && args_contain_pair(args, "--max-exact-percent", "10")
        && args.iter().any(|arg| arg == "--exclude-tests")
}

fn is_g3ts_command(command: &ResolvedCommand) -> bool {
    command.command_name() == "g3ts"
        || command
            .command_path()
            .trim_matches('"')
            .ends_with("scripts/g3ts/verify")
}

fn is_typescript_package_manager_command(command: &ResolvedCommand) -> bool {
    matches!(command.command_name(), "pnpm" | "npm" | "yarn" | "bun")
}

fn args_contain_flag_with_value(args: &[String], flag: &str) -> bool {
    args.windows(2)
        .any(|window| window[0] == flag && !window[1].starts_with('-'))
        || args.iter().any(|arg| {
            arg.strip_prefix(flag)
                .is_some_and(|value| value.starts_with('='))
        })
}

fn args_contain_pair(args: &[String], flag: &str, value: &str) -> bool {
    args.windows(2)
        .any(|window| window[0] == flag && window[1] == value)
        || args.iter().any(|arg| {
            arg.strip_prefix(flag)
                .is_some_and(|suffix| suffix == format!("={value}"))
        })
}

fn check_required_contracts_across_selected_surface(
    inputs: &[G3RsHooksSourceChecksInput],
    results: &mut Vec<G3CheckResult>,
) {
    let Some(pre_commit) = inputs
        .iter()
        .find(|input| input.kind == G3RsHookScriptKind::PreCommit)
    else {
        return;
    };
    let mut content = script_content(pre_commit);
    if pre_commit_dispatches_modular_scripts(pre_commit) {
        for input in inputs.iter().filter(|input| {
            input.kind == G3RsHookScriptKind::Modular
                && input.rel_path.starts_with(".githooks/pre-commit.d/")
        }) {
            content.push_str(script_content(input).as_str());
        }
    }
    let parsed = parse_script(&content);
    let input = crate::inputs::RustHookCommandInput {
        rel_path: pre_commit.rel_path.as_str(),
        parsed: &parsed,
        is_workspace_project: pre_commit.is_workspace_project,
        requirements: &pre_commit.requirements,
    };
    let mut contract_results = Vec::new();
    crate::required_contract_command_present::rule::check(&input, &mut contract_results);
    results.extend(crate::compat::finish(contract_results));
}

fn script_content(input: &G3RsHooksSourceChecksInput) -> String {
    let mut content = String::new();
    for line in &input.parsed.source_lines {
        content.push_str(line.raw.as_str());
        content.push('\n');
    }
    content
}

fn pre_commit_dispatches_modular_scripts(input: &G3RsHooksSourceChecksInput) -> bool {
    input.parsed.executable_lines.iter().any(|line| {
        line.is_dispatcher_syntax && dispatcher_invokes_modular_directory(&line.command_text)
    })
}

fn dispatcher_invokes_modular_directory(command_text: &str) -> bool {
    let words = hook_shell_parser::command_query::shell_words(command_text);
    let Some(command) = words.first().map(String::as_str) else {
        return false;
    };
    match command {
        "run-parts" => words
            .iter()
            .skip(1)
            .any(|word| word.trim_end_matches('/') == ".githooks/pre-commit.d"),
        "." | "source" => words
            .iter()
            .skip(1)
            .any(|word| word == ".githooks/pre-commit.d"),
        _ => false,
    }
}

#[cfg(test)]
#[path = "run_tests/mod.rs"] // reason: owned sidecar tests for run module.
mod run_tests;
