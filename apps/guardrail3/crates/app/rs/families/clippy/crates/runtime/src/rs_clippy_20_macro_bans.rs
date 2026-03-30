use std::collections::BTreeSet;

#[cfg(test)]
use guardrail3_domain_project_tree::ProjectTree;
use guardrail3_domain_report::{CheckResult, Severity};

use super::clippy_support::{display_macro_name, parse_ban_section, EXPECTED_MACRO_BANS};
use super::inputs::ConfigClippyInput;

const ID: &str = "RS-CLIPPY-20";

pub fn check(input: &ConfigClippyInput<'_>, results: &mut Vec<CheckResult>) {
    let Some(parsed) = input.config.parsed.as_ref() else {
        return;
    };

    let section = parse_ban_section(parsed, "disallowed-macros");
    for malformed in &section.malformed_messages {
        results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Error,
            "disallowed-macros section malformed".to_owned(),
            malformed.clone(),
            Some(input.config.rel_path.clone()),
            None,
            false,
        ));
    }

    let found: BTreeSet<_> = section
        .entries
        .into_iter()
        .map(|entry| entry.path)
        .collect();
    for expected in EXPECTED_MACRO_BANS {
        if found.contains(*expected) {
            results.push(
                CheckResult::from_parts(
                    ID.to_owned(),
                    Severity::Info,
                    "macro ban present".to_owned(),
                    format!("`{}!` is banned.", display_macro_name(expected)),
                    Some(input.config.rel_path.clone()),
                    None,
                    false,
                )
                .as_inventory(),
            );
        } else {
            results.push(CheckResult::from_parts(
                ID.to_owned(),
                Severity::Error,
                "missing macro ban".to_owned(),
                format!(
                    "`{}!` is not present in `disallowed-macros`.",
                    display_macro_name(expected)
                ),
                Some(input.config.rel_path.clone()),
                None,
                false,
            ));
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
#[path = "rs_clippy_20_macro_bans_tests/mod.rs"] // reason: test-only sidecar module wiring
mod rs_clippy_20_macro_bans_tests;
