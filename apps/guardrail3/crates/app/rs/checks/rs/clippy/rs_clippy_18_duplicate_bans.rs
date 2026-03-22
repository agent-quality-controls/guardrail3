use std::collections::BTreeMap;

use crate::domain::report::{CheckResult, Severity};

use super::clippy_support::parse_ban_entries;
use super::inputs::ConfigClippyInput;

const ID: &str = "RS-CLIPPY-18";

pub fn check(input: &ConfigClippyInput<'_>, results: &mut Vec<CheckResult>) {
    let Some(parsed) = input.config.parsed.as_ref() else {
        return;
    };

    for key in [
        "disallowed-methods",
        "disallowed-types",
        "disallowed-macros",
    ] {
        let mut counts = BTreeMap::new();
        for entry in parse_ban_entries(parsed, key) {
            *counts.entry(entry.path).or_insert(0usize) += 1;
        }
        for (path, count) in counts {
            if count > 1 {
                results.push(CheckResult {
                    id: ID.to_owned(),
                    severity: Severity::Warn,
                    title: "duplicate ban entry".to_owned(),
                    message: format!("`{path}` appears {count} times in `{key}`."),
                    file: Some(input.config.rel_path.clone()),
                    line: None,
                    inventory: false,
                });
            }
        }
    }
}
