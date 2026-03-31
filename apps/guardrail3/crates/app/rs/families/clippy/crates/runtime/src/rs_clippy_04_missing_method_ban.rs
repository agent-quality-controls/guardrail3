use std::collections::BTreeSet;

#[cfg(test)]
use guardrail3_app_rs_family_mapper::RsProjectSurface as ProjectTree;
use guardrail3_domain_report::{CheckResult, Severity};

use super::clippy_support::{expected_method_bans, parse_ban_section};
use super::inputs::ConfigClippyInput;

const ID: &str = "RS-CLIPPY-04";

pub fn check(input: &ConfigClippyInput<'_>, results: &mut Vec<CheckResult>) {
    if input.policy_context_parse_error().is_some() {
        return;
    }
    let Some(parsed) = input.config.parsed.as_ref() else {
        return;
    };

    let section = parse_ban_section(parsed, "disallowed-methods");
    for malformed in &section.malformed_messages {
        results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Error,
            "disallowed-methods section malformed".to_owned(),
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
    for expected in expected_method_bans(input.garde_enabled()) {
        if found.contains(expected) {
            results.push(
                CheckResult::from_parts(
                    ID.to_owned(),
                    Severity::Info,
                    "method ban present".to_owned(),
                    format!("`{expected}` is banned."),
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
                "missing method ban".to_owned(),
                format!("`{expected}` is not present in `disallowed-methods`."),
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
#[path = "rs_clippy_04_missing_method_ban_tests/mod.rs"] // reason: test-only sidecar module wiring
mod rs_clippy_04_missing_method_ban_tests;
