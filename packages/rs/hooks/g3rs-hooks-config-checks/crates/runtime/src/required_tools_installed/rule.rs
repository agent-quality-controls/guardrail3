use g3rs_hooks_types::G3RsHooksSelectedHookConfigFact;
use guardrail3_check_types::{G3CheckResult, G3Severity};

const ID: &str = "g3rs-hooks/required-tools-installed";

pub(crate) fn check(
    selected_hook: &G3RsHooksSelectedHookConfigFact,
    installed_tools: &[String],
    results: &mut Vec<G3CheckResult>,
) {
    for tool in ["gitleaks", "cargo-deny", "cargo-machete"] {
        let installed = crate::support::tool_installed(installed_tools, tool)
            || crate::support::hook_uses_path_qualified_required_tool(selected_hook, tool);
        if installed {
            results.push(
                G3CheckResult::new(
                    ID.to_owned(),
                    G3Severity::Error,
                    format!("{tool} installed"),
                    format!("{tool} is available for Rust hook execution."),
                    Some(selected_hook.rel_path.clone()),
                    None,
                )
                .into_inventory(),
            );
        } else {
            results.push(G3CheckResult::new(
                ID.to_owned(),
                G3Severity::Error,
                format!("{tool} missing"),
                format!(
                    "{tool} is required by the Rust hook but is not available on PATH or via a path-qualified command."
                ),
                Some(selected_hook.rel_path.clone()),
                None,
            ));
        }
    }
}

#[cfg(test)]
#[path = "rule_tests/mod.rs"] // reason: owned sidecar tests for file module.
mod rule_tests;
