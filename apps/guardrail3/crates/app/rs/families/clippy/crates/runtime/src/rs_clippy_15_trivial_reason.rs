#[cfg(test)]
use guardrail3_domain_project_tree::ProjectTree;
use guardrail3_domain_report::{CheckResult, Severity};

use super::clippy_support::{is_placeholder_reason, parse_ban_entries};
use super::inputs::ConfigClippyInput;

const ID: &str = "RS-CLIPPY-15";

pub fn check(input: &ConfigClippyInput<'_>, results: &mut Vec<CheckResult>) {
    let Some(parsed) = input.config.parsed.as_ref() else {
        return;
    };

    let mut placeholder_count = 0usize;

    for key in [
        "disallowed-methods",
        "disallowed-types",
        "disallowed-macros",
    ] {
        for entry in parse_ban_entries(parsed, key) {
            if let Some(reason) = entry.reason.as_deref()
                && is_placeholder_reason(reason)
            {
                placeholder_count += 1;
                results.push(CheckResult {
                    id: ID.to_owned(),
                    severity: Severity::Warn,
                    title: "ban entry has placeholder reason".to_owned(),
                    message: format!(
                        "`{}` in `{key}` has a trivial or placeholder `reason`.",
                        entry.path
                    ),
                    file: Some(input.config.rel_path.clone()),
                    line: None,
                    inventory: false,
                });
            }
        }
    }

    if placeholder_count == 0 {
        results.push(
            CheckResult {
                id: ID.to_owned(),
                severity: Severity::Info,
                title: "ban reasons are substantive".to_owned(),
                message: "All managed ban entries use substantive non-placeholder `reason` text."
                    .to_owned(),
                file: Some(input.config.rel_path.clone()),
                line: None,
                inventory: false,
            }
            .as_inventory(),
        );
    }
}

#[cfg(test)]
pub(crate) fn run_for_tests(tree: &ProjectTree, rel_path: &str) -> Vec<CheckResult> {
    let facts = super::facts::collect_for_tests(tree);
    let mut results = Vec::new();
    check(
        &super::facts::config_input_for_tests(&facts, rel_path),
        &mut results,
    );
    results
}

#[cfg(test)]
#[path = "rs_clippy_15_trivial_reason_tests/mod.rs"] // reason: test-only sidecar module wiring
mod rs_clippy_15_trivial_reason_tests;
