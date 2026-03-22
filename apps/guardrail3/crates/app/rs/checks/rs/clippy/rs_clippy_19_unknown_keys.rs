use std::collections::BTreeSet;

use crate::domain::report::{CheckResult, Severity};

use super::clippy_support::{
    known_top_level_keys, looks_like_managed_typo, managed_non_threshold_keys,
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
        if !known.contains(key.as_str()) && looks_like_managed_typo(key) {
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
