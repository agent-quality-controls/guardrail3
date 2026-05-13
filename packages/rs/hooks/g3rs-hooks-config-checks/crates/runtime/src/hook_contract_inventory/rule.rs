use g3rs_hooks_contract_types::G3HookRequirement;
use g3rs_hooks_types::G3RsHooksSelectedHookConfigFact;
use guardrail3_check_types::{G3CheckResult, G3Severity};

/// Emits inventory for every loaded family hook contract.
pub(crate) fn check(
    selected_hook: &G3RsHooksSelectedHookConfigFact,
    requirements: &[G3HookRequirement],
    results: &mut Vec<G3CheckResult>,
) {
    for requirement in requirements {
        results.push(
            G3CheckResult::new(
                requirement.id.clone(),
                G3Severity::Info,
                format!("{} hook contract loaded", requirement.owner_family),
                format!(
                    "{} hook contract is loaded with {} trigger pattern(s), {} required command(s), and {} critical command(s).",
                    requirement.owner_family,
                    requirement.trigger_patterns.len(),
                    requirement.required_commands.len(),
                    requirement.critical_commands.len()
                ),
                Some(selected_hook.rel_path.clone()),
                None,
            )
            .into_inventory(),
        );
    }
}

#[cfg(test)]
#[path = "rule_tests/mod.rs"] // reason: owned sidecar tests for file module.
mod rule_tests;
