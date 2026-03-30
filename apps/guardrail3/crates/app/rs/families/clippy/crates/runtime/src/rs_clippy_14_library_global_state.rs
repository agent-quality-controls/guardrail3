use std::collections::BTreeSet;

#[cfg(test)]
use guardrail3_domain_project_tree::ProjectTree;
use guardrail3_domain_report::{CheckResult, Severity};

use super::clippy_support::{EXPECTED_LIBRARY_GLOBAL_STATE_TYPES, parse_ban_section};
use super::inputs::ConfigClippyInput;

const ID: &str = "RS-CLIPPY-14";

pub fn check(input: &ConfigClippyInput<'_>, results: &mut Vec<CheckResult>) {
    if input.policy_context_parse_error().is_some() {
        return;
    }
    if input.profile_name() != Some("library") {
        return;
    }
    let Some(parsed) = input.config.parsed.as_ref() else {
        return;
    };

    let section = parse_ban_section(parsed, "disallowed-types");
    let mut malformed_count = 0usize;
    for malformed in &section.malformed_messages {
        malformed_count += 1;
        results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Error,
            "disallowed-types section malformed".to_owned(),
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
    let mut missing_count = 0usize;
    for expected in EXPECTED_LIBRARY_GLOBAL_STATE_TYPES {
        if !found.contains(*expected) {
            missing_count += 1;
            results.push(CheckResult::from_parts(
                ID.to_owned(),
                Severity::Error,
                "library clippy.toml missing global-state type ban".to_owned(),
                format!("Library profile must ban `{expected}` in `disallowed-types`."),
                Some(input.config.rel_path.clone()),
                None,
                false,
            ));
        }
    }

    if malformed_count == 0 && missing_count == 0 {
        results.push(
            CheckResult::from_parts(
                ID.to_owned(),
                Severity::Info,
                "library global-state bans present".to_owned(),
                "Library profile includes all managed global-state type bans.".to_owned(),
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
#[path = "rs_clippy_14_library_global_state_tests/mod.rs"] // reason: test-only sidecar module wiring
mod rs_clippy_14_library_global_state_tests;
