use g3rs_hooks_types::G3RsHooksSelectedHookConfigFact;
use guardrail3_check_types::G3CheckResult;

/// Rule identifier emitted by this check.
const ID: &str = "g3rs-hooks/cargo-dupes-installed";

/// Runs the rule and appends any findings to `results`.
pub(crate) fn check(
    selected_hook: &G3RsHooksSelectedHookConfigFact,
    installed_tools: &[String],
    results: &mut Vec<G3CheckResult>,
) {
    crate::support::check_required_tool_availability(
        selected_hook,
        installed_tools,
        crate::support::RequiredToolAvailabilityCheck {
            required: crate::support::hook_requires_cargo_dupes,
            path_qualified: crate::support::hook_uses_path_qualified_cargo_dupes,
            tool: "cargo-dupes",
            messages: crate::support::ToolAvailabilityMessages {
                id: ID,
                available_title: "cargo-dupes installed",
                available_message: "cargo-dupes is available for Rust duplication checks.",
                missing_title: "cargo-dupes missing",
                missing_message: "Hook requires cargo-dupes, but it is not available on PATH.",
            },
        },
        results,
    );
}
