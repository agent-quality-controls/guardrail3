use std::collections::BTreeSet;

#[cfg(test)]
use guardrail3_domain_project_tree::ProjectTree;
use guardrail3_domain_report::{CheckResult, Severity};

use super::clippy_support::{
    known_top_level_keys, managed_non_threshold_keys, normalized_key_distance,
};
use super::inputs::ConfigClippyInput;

const ID: &str = "RS-CLIPPY-19";

pub fn check(input: &ConfigClippyInput<'_>, results: &mut Vec<CheckResult>) {
    let Some(parsed) = input.config.parsed.as_ref() else {
        return;
    };
    let Some(table) = parsed.as_table() else {
        return;
    };

    let known: BTreeSet<_> = known_top_level_keys()
        .into_iter()
        .chain(managed_non_threshold_keys())
        .collect();
    for key in table.keys() {
        let looks_like_managed_typo = !known.contains(key.as_str())
            && known
                .iter()
                .copied()
                .any(|managed| normalized_key_distance(key, managed) <= 2);
        if looks_like_managed_typo {
            results.push(CheckResult {
                id: ID.to_owned(),
                severity: Severity::Warn,
                title: "unrecognized clippy.toml key".to_owned(),
                message: format!(
                    "Top-level key `{key}` looks like a typo of a guardrail-managed clippy key."
                ),
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
    check(
        &super::facts::config_input_for_tests(&facts, rel_path),
        &mut results,
    );
    results
}

#[cfg(test)]
#[path = "rs_clippy_19_unknown_keys_tests/mod.rs"]
mod rs_clippy_19_unknown_keys_tests;
