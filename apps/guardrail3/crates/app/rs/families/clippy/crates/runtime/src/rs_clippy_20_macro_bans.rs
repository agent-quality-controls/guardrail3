use std::collections::BTreeSet;

use guardrail3_domain_report::{CheckResult, Severity};
#[cfg(test)]
use guardrail3_domain_project_tree::ProjectTree;

use super::clippy_support::{EXPECTED_MACRO_BANS, ban_paths};
use super::inputs::ConfigClippyInput;

const ID: &str = "RS-CLIPPY-20";

pub fn check(input: &ConfigClippyInput<'_>, results: &mut Vec<CheckResult>) {
    let Some(parsed) = input.config.parsed.as_ref() else {
        return;
    };

    let found: BTreeSet<_> = ban_paths(parsed, "disallowed-macros").into_iter().collect();
    for expected in EXPECTED_MACRO_BANS {
        if found.contains(*expected) {
            results.push(
                CheckResult {
                    id: ID.to_owned(),
                    severity: Severity::Info,
                    title: "macro ban present".to_owned(),
                    message: format!("`{expected}!` is banned."),
                    file: Some(input.config.rel_path.clone()),
                    line: None,
                    inventory: false,
                }
                .as_inventory(),
            );
        } else {
            results.push(CheckResult {
                id: ID.to_owned(),
                severity: Severity::Error,
                title: "missing macro ban".to_owned(),
                message: format!("`{expected}!` is not present in `disallowed-macros`."),
                file: Some(input.config.rel_path.clone()),
                line: None,
                inventory: false,
            });
        }
    }
}

#[cfg(test)]
pub(crate) fn run_for_tests(tree: &ProjectTree, rel_path: &str) -> Vec<CheckResult> {
    let facts = super::facts::collect_for_tests(tree);
    let mut results = Vec::new();
    check(&super::facts::config_input_for_tests(&facts, rel_path), &mut results);
    results
}

#[cfg(test)]
#[path = "rs_clippy_20_macro_bans_tests/mod.rs"]
mod rs_clippy_20_macro_bans_tests;
