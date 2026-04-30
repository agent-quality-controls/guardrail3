use g3ts_hooks_contract_types::{G3TsHookCommandRequirement, G3TsHookTriggerPattern};
use g3ts_hooks_types::{G3TsHookScriptKind, G3TsHooksSourceChecksInput};
use guardrail3_check_types::G3CheckResult;

use crate::commands::{
    any_script_command, app_roots, command_mentions_guardrail_ts_config, command_mentions_pattern,
    is_app_validate_command, is_g3ts_validate_path_command,
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
    let Some(primary) = inputs.first() else {
        return results;
    };
    g3ts_validate_staged_present(inputs, primary, &mut results);
    app_validate_step_present(inputs, primary, &mut results);
    guardrail_config_changes_trigger_validation(inputs, primary, &mut results);
    contract_trigger_patterns_covered(inputs, primary, &mut results);
    for input in inputs {
        no_fail_open_wrappers(input, &mut results);
        dispatcher_inventory(input, &mut results);
    }
    results
}

fn g3ts_validate_staged_present(
    inputs: &[G3TsHooksSourceChecksInput],
    primary: &G3TsHooksSourceChecksInput,
    results: &mut Vec<G3CheckResult>,
) {
    if app_roots(primary).iter().all(|app_root| {
        any_script_command(inputs, |command| {
            is_g3ts_validate_path_command(command, app_root)
        })
    }) {
        return;
    }
    results.push(error(
        "g3ts-hooks/g3ts-validate-staged-present",
        "pre-commit hook does not run g3ts validate",
        "The selected pre-commit hook must execute `g3ts validate --path ...` so TypeScript guardrails run before commits. Echoed text, comments, and aliases are not enough.",
        primary.rel_path(),
        None,
    ));
}

fn app_validate_step_present(
    inputs: &[G3TsHooksSourceChecksInput],
    primary: &G3TsHooksSourceChecksInput,
    results: &mut Vec<G3CheckResult>,
) {
    if primary
        .requirements()
        .iter()
        .flat_map(g3ts_hooks_contract_types::G3TsHookRequirement::required_commands)
        .any(|requirement| requirement == &G3TsHookCommandRequirement::AppValidateScript)
        && !app_roots(primary).iter().all(|app_root| {
            any_script_command(inputs, |command| is_app_validate_command(command, app_root))
        })
    {
        results.push(error(
            "g3ts-hooks/ts-app-validate-step-present",
            "hook does not run the app validate script",
            "A TypeScript hook contract requires the app-level `validate` script to run before commits. Add a real package-manager command such as `pnpm --filter <app> run validate`; comments and echoed text do not satisfy this rule.",
            primary.rel_path(),
            None,
        ));
    }
}

fn guardrail_config_changes_trigger_validation(
    inputs: &[G3TsHooksSourceChecksInput],
    primary: &G3TsHooksSourceChecksInput,
    results: &mut Vec<G3CheckResult>,
) {
    if any_script_command(inputs, command_mentions_guardrail_ts_config)
        && app_roots(primary).iter().all(|app_root| {
            any_script_command(inputs, |command| {
                is_g3ts_validate_path_command(command, app_root)
            })
        })
    {
        return;
    }
    results.push(error(
        "g3ts-hooks/ts-guardrail-config-changes-trigger-validation",
        "guardrail3-ts.toml changes do not trigger g3ts",
        "The pre-commit hook must explicitly include `guardrail3-ts.toml` in its changed-file routing and run `g3ts validate --path ...` when that file changes.",
        primary.rel_path(),
        None,
    ));
}

fn contract_trigger_patterns_covered(
    inputs: &[G3TsHooksSourceChecksInput],
    primary: &G3TsHooksSourceChecksInput,
    results: &mut Vec<G3CheckResult>,
) {
    let missing = primary
        .requirements()
        .iter()
        .flat_map(g3ts_hooks_contract_types::G3TsHookRequirement::trigger_patterns)
        .filter_map(|pattern| match pattern {
            G3TsHookTriggerPattern::Glob(pattern) => Some(pattern.as_str()),
        })
        .filter(|pattern| {
            !any_script_command(inputs, |command| command_mentions_pattern(command, pattern))
        })
        .collect::<Vec<_>>();

    if missing.is_empty() {
        return;
    }

    results.push(error(
        "g3ts-hooks/contract-trigger-patterns-covered",
        "hook does not route declared TypeScript trigger patterns",
        format!(
            "The hook contract declares trigger patterns that are not mentioned by executable hook routing commands: {}. Add staged-file routing for these patterns and run the required validation commands from that route.",
            missing.join(", ")
        )
        .as_str(),
        primary.rel_path(),
        None,
    ));
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
            "`.githooks/pre-commit.d` exists; command-presence checks inspect direct modular scripts as well as the dispatcher.".to_owned(),
            input.rel_path(),
            None,
        ));
    }
}

#[cfg(test)]
#[path = "run_tests/mod.rs"]
mod run_tests;
