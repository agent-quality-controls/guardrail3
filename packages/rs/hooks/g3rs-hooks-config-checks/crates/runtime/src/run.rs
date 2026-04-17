use g3rs_hooks_types::G3RsHooksConfigChecksInput;
use guardrail3_check_types::G3CheckResult;

pub fn check(input: &G3RsHooksConfigChecksInput) -> Vec<G3CheckResult> {
    if !input.active {
        return Vec::new();
    }

    let Some(selected_hook) = &input.selected_hook else {
        return Vec::new();
    };

    let mut results = Vec::new();

    crate::hook_rs_06_required_tools_installed::check(
        selected_hook,
        &input.installed_tools,
        &mut results,
    );
    crate::hook_rs_14_guardrail_binary_available::check(
        selected_hook,
        &input.installed_tools,
        &mut results,
    );
    crate::hook_rs_15_cargo_dupes_installed::check(
        selected_hook,
        &input.installed_tools,
        &mut results,
    );

    results
}
