use std::collections::BTreeSet;

#[cfg(test)]
use guardrail3_domain_project_tree::ProjectTree;
use guardrail3_domain_report::{CheckResult, Severity};

use super::clippy_support::{ban_paths, expected_method_bans};
use super::inputs::ConfigClippyInput;

const ID: &str = "RS-CLIPPY-06";

pub fn check(input: &ConfigClippyInput<'_>, results: &mut Vec<CheckResult>) {
    if input.policy_context_parse_error().is_some() {
        return;
    }
    let Some(parsed) = input.config.parsed.as_ref() else {
        return;
    };

    let expected: BTreeSet<_> = expected_method_bans(input.garde_enabled())
        .into_iter()
        .collect();
    for found in ban_paths(parsed, "disallowed-methods") {
        if !expected.contains(found.as_str()) {
            results.push(
                CheckResult {
                    id: ID.to_owned(),
                    severity: Severity::Info,
                    title: "extra method ban".to_owned(),
                    message: format!("Additional method ban `{found}` beyond baseline."),
                    file: Some(input.config.rel_path.clone()),
                    line: None,
                    inventory: false,
                }
                .as_inventory(),
            );
        }
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
#[path = "rs_clippy_06_extra_method_ban_tests/mod.rs"]
mod rs_clippy_06_extra_method_ban_tests;
