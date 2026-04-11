use g3rs_hooks_config_checks_types::G3RsHooksSelectedHookConfigFact;
use guardrail3_check_types::{G3CheckResult, G3Severity};

const ID: &str = "HOOK-RS-15";

pub(crate) fn check(
    selected_hook: &G3RsHooksSelectedHookConfigFact,
    installed_tools: &[String],
    results: &mut Vec<G3CheckResult>,
) {
    let cargo_dupes_required = crate::support::hook_requires_cargo_dupes(selected_hook);
    if !cargo_dupes_required {
        return;
    }

    let path_qualified = crate::support::hook_uses_path_qualified_cargo_dupes(selected_hook);
    let installed = crate::support::tool_installed(installed_tools, "cargo-dupes");

    if path_qualified || installed {
        results.push(
            G3CheckResult::new(
                ID.to_owned(),
                G3Severity::Error,
                "cargo-dupes installed".to_owned(),
                "cargo-dupes is available for Rust duplication checks.".to_owned(),
                Some(selected_hook.rel_path.clone()),
                None,
            )
            .into_inventory(),
        );
    } else {
        results.push(G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Error,
            "cargo-dupes missing".to_owned(),
            "Hook requires cargo-dupes, but it is not available on PATH.".to_owned(),
            Some(selected_hook.rel_path.clone()),
            None,
        ));
    }
}

#[cfg(test)]
#[path = "hook_rs_15_cargo_dupes_installed_tests/mod.rs"]
mod tests;
