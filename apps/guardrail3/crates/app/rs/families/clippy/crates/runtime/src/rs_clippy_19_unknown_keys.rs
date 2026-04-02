use std::collections::BTreeSet;

#[cfg(test)]
use guardrail3_app_rs_family_view::FamilyView as ProjectTree;
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

    let mut typo_count = 0usize;
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
            typo_count += 1;
            results.push(CheckResult::from_parts(
                ID.to_owned(),
                Severity::Warn,
                "unrecognized clippy.toml key".to_owned(),
                format!(
                    "Top-level key `{key}` looks like a typo of a guardrail-managed clippy key."
                ),
                Some(input.config.rel_path.clone()),
                None,
                false,
            ));
        }
    }

    if typo_count == 0 {
        results.push(
            CheckResult::from_parts(
                ID.to_owned(),
                Severity::Info,
                "no suspicious managed-key typos".to_owned(),
                "No top-level keys look like typos of guardrail-managed clippy keys.".to_owned(),
                Some(input.config.rel_path.clone()),
                None,
                false,
            )
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
#[path = "rs_clippy_19_unknown_keys_tests/mod.rs"] // reason: test-only sidecar module wiring
mod rs_clippy_19_unknown_keys_tests;
