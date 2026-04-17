use g3rs_hooks_types::G3RsHooksSelectedHookConfigFact;
use guardrail3_check_types::{G3CheckResult, G3Severity};

const ID: &str = "RS-HOOKS-CONFIG-02";

pub(crate) fn check(
    selected_hook: &G3RsHooksSelectedHookConfigFact,
    installed_tools: &[String],
    results: &mut Vec<G3CheckResult>,
) {
    let validation_expected = crate::support::hook_requires_g3rs_validation(selected_hook);
    if !validation_expected {
        return;
    }

    let path_qualified = crate::support::hook_uses_path_qualified_g3rs(selected_hook);
    let installed = crate::support::tool_installed(installed_tools, "g3rs");

    if path_qualified || installed {
        results.push(
            G3CheckResult::new(
                ID.to_owned(),
                G3Severity::Error,
                "g3rs binary available".to_owned(),
                "g3rs is available for fail-closed Rust hook validation.".to_owned(),
                Some(selected_hook.rel_path.clone()),
                None,
            )
            .into_inventory(),
        );
    } else {
        results.push(G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Error,
            "g3rs binary missing".to_owned(),
            "Hook requires g3rs, but it is not available on PATH.".to_owned(),
            Some(selected_hook.rel_path.clone()),
            None,
        ));
    }
}

#[cfg(test)]
#[path = "rule_tests/mod.rs"] // reason: owned sidecar tests for file module.
mod rule_tests;
