use g3ts_hooks_types::G3TsHooksConfigChecksInput;
use guardrail3_check_types::{G3CheckResult, G3Severity};
use hook_shell_parser::command_query::{ResolvedCommand, any_resolved_command};

/// Runs all G3TS hooks config checks and returns aggregated results.
#[must_use]
pub fn check(input: &G3TsHooksConfigChecksInput) -> Vec<G3CheckResult> {
    if !input.active() {
        return Vec::new();
    }

    let mut results = Vec::new();
    g3ts_binary_available(input, &mut results);
    results
}

/// Emits a result when the selected hook invokes `g3ts` but the binary is not installed.
fn g3ts_binary_available(input: &G3TsHooksConfigChecksInput, results: &mut Vec<G3CheckResult>) {
    let Some(selected_hook) = input.selected_hook() else {
        return;
    };
    let hook_invokes_g3ts = any_resolved_command(selected_hook.parsed(), is_g3ts_command);
    if hook_invokes_g3ts && !input.installed_tools().iter().any(|tool| tool == "g3ts") {
        results.push(G3CheckResult::new(
            "g3ts-hooks/g3ts-binary-available".to_owned(),
            G3Severity::Error,
            "g3ts binary is not available".to_owned(),
            "The selected pre-commit hook invokes `g3ts`, but `g3ts` is not present on PATH for this validation run. Install the local G3TS CLI before relying on the hook contract.".to_owned(),
            Some(selected_hook.rel_path().to_owned()),
            None,
        ));
    }
}

/// Returns true when `command` resolves to the `g3ts` CLI.
fn is_g3ts_command(command: &ResolvedCommand) -> bool {
    command.command_name() == "g3ts"
}
