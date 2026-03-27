use std::collections::BTreeSet;

use guardrail3_domain_report::{CheckResult, Severity};

use super::clippy_support::{ban_paths, expected_type_bans};
use super::inputs::ConfigClippyInput;

const ID: &str = "RS-CLIPPY-07";

pub fn check(input: &ConfigClippyInput<'_>, results: &mut Vec<CheckResult>) {
    let Some(parsed) = input.config.parsed.as_ref() else {
        return;
    };

    let expected: BTreeSet<_> = expected_type_bans(input.profile_name(), input.garde_enabled())
        .into_iter()
        .collect();
    for found in ban_paths(parsed, "disallowed-types") {
        if !expected.contains(found.as_str()) {
            results.push(
                CheckResult {
                    id: ID.to_owned(),
                    severity: Severity::Info,
                    title: "extra type ban".to_owned(),
                    message: format!("Additional type ban `{found}` beyond baseline."),
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
#[path = "rs_clippy_07_extra_type_ban_tests/mod.rs"]
mod rs_clippy_07_extra_type_ban_tests;
