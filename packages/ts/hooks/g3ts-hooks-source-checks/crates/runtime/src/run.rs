use g3ts_hooks_types::{G3TsHookScriptKind, G3TsHooksSourceChecksInput};
use guardrail3_check_types::G3CheckResult;

use crate::commands::{
    command_has_arg, is_g3ts_validate_path_command, is_g3ts_verify_pre_commit_command,
    script_category_command, script_command,
};
use crate::fail_open::{critical_command_names, first_fail_open_critical_command};
use crate::results::{error, info};

#[must_use]
pub fn check(input: &G3TsHooksSourceChecksInput) -> Vec<G3CheckResult> {
    check_effective(std::slice::from_ref(input))
}

#[must_use]
pub fn check_effective(inputs: &[G3TsHooksSourceChecksInput]) -> Vec<G3CheckResult> {
    let mut results = Vec::new();
    let Some(primary) = inputs
        .iter()
        .find(|input| input.kind() == G3TsHookScriptKind::PreCommit)
        .or_else(|| inputs.first())
    else {
        return results;
    };

    pre_commit_invokes_g3ts_verifier(inputs, primary, &mut results);
    let verifier = inputs
        .iter()
        .find(|input| input.kind() == G3TsHookScriptKind::Verifier);
    verifier_exists(verifier, primary, &mut results);
    if let Some(verifier) = verifier {
        verifier_runs_g3ts_validate(verifier, primary, &mut results);
        verifier_runs_typecheck(verifier, &mut results);
        verifier_runs_lint(verifier, &mut results);
        verifier_runs_format_check(verifier, &mut results);
        verifier_runs_spelling_check(verifier, &mut results);
        verifier_runs_stylelint(verifier, primary, &mut results);
        verifier_runs_package_policy(verifier, primary, &mut results);
        verifier_runs_typecov(verifier, primary, &mut results);
        verifier_does_not_call_g3rs(verifier, &mut results);
        verifier_does_not_call_cargo(verifier, &mut results);
    }
    for input in inputs {
        no_fail_open_wrappers(input, &mut results);
        dispatcher_inventory(input, &mut results);
    }
    results
}

fn pre_commit_invokes_g3ts_verifier(
    inputs: &[G3TsHooksSourceChecksInput],
    primary: &G3TsHooksSourceChecksInput,
    results: &mut Vec<G3CheckResult>,
) {
    if inputs
        .iter()
        .filter(|input| input.kind() == G3TsHookScriptKind::PreCommit)
        .any(|input| script_command(input, is_g3ts_verify_pre_commit_command))
    {
        return;
    }
    results.push(error(
        "g3ts-hooks/pre-commit-invokes-g3ts-verifier",
        "pre-commit hook does not run the G3TS verifier",
        "The selected pre-commit hook must execute `scripts/g3ts/verify --mode pre-commit --scope <scope>`.",
        primary.rel_path(),
        None,
    ));
}

fn verifier_exists(
    verifier: Option<&G3TsHooksSourceChecksInput>,
    primary: &G3TsHooksSourceChecksInput,
    results: &mut Vec<G3CheckResult>,
) {
    if verifier.is_some() {
        return;
    }
    results.push(error(
        "g3ts-hooks/verifier-exists",
        "G3TS verifier script is missing",
        "`scripts/g3ts/verify` must exist and own TypeScript verification.",
        primary.rel_path(),
        None,
    ));
}

fn verifier_runs_g3ts_validate(
    verifier: &G3TsHooksSourceChecksInput,
    primary: &G3TsHooksSourceChecksInput,
    results: &mut Vec<G3CheckResult>,
) {
    if app_roots(primary)
        .iter()
        .all(|app_root| script_command(verifier, |command| is_g3ts_validate_path_command(command, app_root)))
    {
        return;
    }
    results.push(error(
        "g3ts-hooks/verifier-runs-g3ts-validate",
        "G3TS verifier does not run g3ts validate",
        "`scripts/g3ts/verify` must execute `g3ts validate --path \"$SCOPE\"` or an equivalent checked complete workspace validate route.",
        verifier.rel_path(),
        None,
    ));
}

fn verifier_runs_typecheck(verifier: &G3TsHooksSourceChecksInput, results: &mut Vec<G3CheckResult>) {
    if script_category_command(verifier, |command| {
        command.command_name() == "tsc"
            || package_manager_exec(command, "tsc")
            || package_manager_run(command, &["typecheck"])
            || helper_exec(command, "tsc")
            || helper_script(command, &["typecheck"])
    }) {
        return;
    }
    results.push(missing_category(verifier, "g3ts-hooks/verifier-runs-typecheck", "typecheck"));
}

fn verifier_runs_lint(verifier: &G3TsHooksSourceChecksInput, results: &mut Vec<G3CheckResult>) {
    if script_category_command(verifier, |command| {
        command.command_name() == "eslint"
            || package_manager_exec(command, "eslint")
            || package_manager_run(command, &["lint"])
            || helper_exec(command, "eslint")
            || helper_script(command, &["lint"])
    }) {
        return;
    }
    results.push(missing_category(verifier, "g3ts-hooks/verifier-runs-lint", "lint"));
}

fn verifier_runs_format_check(verifier: &G3TsHooksSourceChecksInput, results: &mut Vec<G3CheckResult>) {
    if script_category_command(verifier, |command| {
        direct_tool_with_arg(command, "prettier", "--check")
            || package_manager_exec_with_arg(command, "prettier", "--check")
            || package_manager_run(command, &["format:check", "check:format"])
            || helper_exec_with_arg(command, "prettier", "--check")
            || helper_script(command, &["format:check", "check:format"])
    }) {
        return;
    }
    results.push(missing_category(verifier, "g3ts-hooks/verifier-runs-format-check", "format check"));
}

fn verifier_runs_spelling_check(verifier: &G3TsHooksSourceChecksInput, results: &mut Vec<G3CheckResult>) {
    if script_category_command(verifier, |command| {
        command.command_name() == "cspell"
            || package_manager_exec(command, "cspell")
            || package_manager_run(command, &["spellcheck", "spelling"])
            || helper_exec(command, "cspell")
            || helper_script(command, &["spellcheck", "spelling"])
    }) {
        return;
    }
    results.push(missing_category(verifier, "g3ts-hooks/verifier-runs-spelling-check", "spelling check"));
}

fn verifier_runs_stylelint(
    verifier: &G3TsHooksSourceChecksInput,
    primary: &G3TsHooksSourceChecksInput,
    results: &mut Vec<G3CheckResult>,
) {
    if !primary.enabled_categories().stylelint() {
        return;
    }
    if script_category_command(verifier, |command| {
        command.command_name() == "stylelint"
            || package_manager_exec(command, "stylelint")
            || package_manager_run(command, &["stylelint"])
            || helper_exec(command, "stylelint")
            || helper_script(command, &["stylelint"])
    }) {
        return;
    }
    results.push(missing_category(verifier, "g3ts-hooks/verifier-runs-stylelint", "stylelint"));
}

fn verifier_runs_package_policy(
    verifier: &G3TsHooksSourceChecksInput,
    primary: &G3TsHooksSourceChecksInput,
    results: &mut Vec<G3CheckResult>,
) {
    if !primary.enabled_categories().package_policy() {
        return;
    }
    if script_category_command(verifier, |command| {
        command.command_name() == "syncpack"
            || package_manager_exec(command, "syncpack")
            || package_manager_run(command, &["package:policy", "syncpack"])
            || helper_exec(command, "syncpack")
            || helper_script(command, &["package:policy", "syncpack"])
    }) {
        return;
    }
    results.push(missing_category(verifier, "g3ts-hooks/verifier-runs-package-policy", "package policy"));
}

fn verifier_runs_typecov(
    verifier: &G3TsHooksSourceChecksInput,
    primary: &G3TsHooksSourceChecksInput,
    results: &mut Vec<G3CheckResult>,
) {
    if !primary.enabled_categories().typecov() {
        return;
    }
    if script_category_command(verifier, |command| {
        command.command_name() == "type-coverage"
            || package_manager_exec(command, "type-coverage")
            || package_manager_run(command, &["typecov"])
            || helper_exec(command, "type-coverage")
            || helper_script(command, &["typecov"])
    }) {
        return;
    }
    results.push(missing_category(verifier, "g3ts-hooks/verifier-runs-typecov", "type coverage"));
}

fn missing_category(
    verifier: &G3TsHooksSourceChecksInput,
    id: &str,
    category: &str,
) -> G3CheckResult {
    error(
        id,
        format!("G3TS verifier does not run {category}").as_str(),
        format!("`scripts/g3ts/verify` must run the TypeScript {category} category when enabled for the configured scope.").as_str(),
        verifier.rel_path(),
        None,
    )
}

fn direct_tool_with_arg(command: &hook_shell_parser::command_query::ResolvedCommand, tool: &str, arg: &str) -> bool {
    command.command_name() == tool && command_has_arg(command.args(), arg)
}

fn package_manager_exec(command: &hook_shell_parser::command_query::ResolvedCommand, tool: &str) -> bool {
    let args = command.args();
    match command.command_name() {
        "pnpm" | "yarn" => args.first().is_some_and(|arg| arg == "exec")
            && args.get(1).is_some_and(|arg| arg == tool),
        "npm" => args.first().is_some_and(|arg| arg == "exec")
            && args.get(1).is_some_and(|arg| arg == tool),
        "bun" => matches!(args.first().map(String::as_str), Some("x" | "exec"))
            && args.get(1).is_some_and(|arg| arg == tool),
        "npx" | "bunx" => args.iter().any(|arg| arg == tool),
        _ => false,
    }
}

fn package_manager_exec_with_arg(
    command: &hook_shell_parser::command_query::ResolvedCommand,
    tool: &str,
    arg: &str,
) -> bool {
    package_manager_exec(command, tool) && command_has_arg(command.args(), arg)
}

fn package_manager_run(command: &hook_shell_parser::command_query::ResolvedCommand, scripts: &[&str]) -> bool {
    let args = command.args();
    if !matches!(command.command_name(), "pnpm" | "npm" | "yarn" | "bun") {
        return false;
    }
    let script_index = if args.first().is_some_and(|arg| arg == "run") {
        1
    } else if command.command_name() == "yarn" {
        0
    } else {
        return false;
    };
    args.get(script_index)
        .is_some_and(|script| scripts.iter().any(|expected| script == expected))
}

fn helper_exec(command: &hook_shell_parser::command_query::ResolvedCommand, tool: &str) -> bool {
    command.command_name() == "run_pm_exec" && command.args().get(1).is_some_and(|arg| arg == tool)
}

fn helper_exec_with_arg(
    command: &hook_shell_parser::command_query::ResolvedCommand,
    tool: &str,
    arg: &str,
) -> bool {
    helper_exec(command, tool) && command_has_arg(command.args(), arg)
}

fn helper_script(command: &hook_shell_parser::command_query::ResolvedCommand, scripts: &[&str]) -> bool {
    command.command_name() == "run_pm_script"
        && command
            .args()
            .get(1)
            .is_some_and(|script| scripts.iter().any(|expected| script == expected))
}

fn verifier_does_not_call_g3rs(
    verifier: &G3TsHooksSourceChecksInput,
    results: &mut Vec<G3CheckResult>,
) {
    if script_command(verifier, |command| {
        command.command_name() == "g3rs"
            || command
                .command_path()
                .trim_matches('"')
                .ends_with("scripts/g3rs/verify")
    }) {
        results.push(error(
            "g3ts-hooks/verifier-does-not-call-g3rs",
            "G3TS verifier calls g3rs",
            "`scripts/g3ts/verify` must not invoke Rust guardrails.",
            verifier.rel_path(),
            None,
        ));
    }
}

fn verifier_does_not_call_cargo(
    verifier: &G3TsHooksSourceChecksInput,
    results: &mut Vec<G3CheckResult>,
) {
    if script_command(verifier, |command| command.command_name() == "cargo") {
        results.push(error(
            "g3ts-hooks/verifier-does-not-call-cargo",
            "G3TS verifier calls Cargo",
            "`scripts/g3ts/verify` must not run Cargo as part of TypeScript verification.",
            verifier.rel_path(),
            None,
        ));
    }
}

fn app_roots(input: &G3TsHooksSourceChecksInput) -> Vec<String> {
    if input.app_package_roots().is_empty() {
        return vec!["$SCOPE".to_owned(), "SCOPE".to_owned()];
    }
    let mut roots = input.app_package_roots().to_vec();
    roots.push("$SCOPE".to_owned());
    roots.push("SCOPE".to_owned());
    roots
}

fn no_fail_open_wrappers(input: &G3TsHooksSourceChecksInput, results: &mut Vec<G3CheckResult>) {
    let critical_commands = critical_command_names(input);
    if let Some((line_no, command_text)) = first_fail_open_critical_command(
        input.parsed(),
        input.parsed(),
        1,
        0,
        &mut Vec::new(),
        &critical_commands,
    ) {
        results.push(error(
            "g3ts-hooks/no-fail-open-wrappers",
            "critical hook command is fail-open",
            format!("Critical hook command `{command_text}` is softened by a fail-open wrapper. Remove `|| true`, `|| return 0`, soft command substitutions, or non-failing availability guards so the hook fails closed.")
                .as_str(),
            input.rel_path(),
            Some(line_no),
        ));
    }
}

fn dispatcher_inventory(input: &G3TsHooksSourceChecksInput, results: &mut Vec<G3CheckResult>) {
    if input.kind() == G3TsHookScriptKind::PreCommit && input.has_modular_dir() {
        results.push(info(
            "g3ts-hooks/pre-commit-dispatcher-inventory",
            "pre-commit dispatcher inventory",
            "`.githooks/pre-commit.d` exists; G3TS verifier checks inspect `scripts/g3ts/verify` directly.".to_owned(),
            input.rel_path(),
            None,
        ));
    }
}

#[cfg(test)]
#[path = "run_tests/mod.rs"]
mod run_tests;
