use g3rs_hooks_types::G3RsHooksSelectedHookConfigFact;
use guardrail3_check_types::G3CheckResult;

/// Rule identifier emitted by this check.
const ID: &str = "g3rs-hooks/guardrail-binary-available";
/// Tool availability contract checked by this rule.
const CHECK: crate::support::RequiredToolAvailabilityCheck<'static> =
    crate::support::RequiredToolAvailabilityCheck {
        required: crate::support::hook_requires_g3rs_validation,
        path_qualified: crate::support::hook_uses_path_qualified_g3rs,
        tool: "g3rs",
        messages: crate::support::ToolAvailabilityMessages {
            id: ID,
            available_title: "g3rs binary available",
            available_message: "g3rs is available for fail-closed Rust hook validation.",
            missing_title: "g3rs binary missing",
            missing_message: "Hook requires g3rs, but it is not available on PATH.",
        },
    };

/// Runs the rule and appends any findings to `results`.
pub(crate) fn check(
    selected_hook: &G3RsHooksSelectedHookConfigFact,
    installed_tools: &[String],
    results: &mut Vec<G3CheckResult>,
) {
    crate::support::check_required_tool_availability(
        selected_hook,
        installed_tools,
        CHECK,
        results,
    );
}
