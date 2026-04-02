use std::collections::BTreeSet;

#[cfg(test)]
use guardrail3_app_rs_family_view::FamilyView as ProjectTree;
use guardrail3_domain_report::{CheckResult, Severity};

use super::clippy_support::{expected_type_bans, parse_ban_section};
use super::inputs::ConfigClippyInput;

const ID: &str = "RS-CLIPPY-07";

pub fn check(input: &ConfigClippyInput<'_>, results: &mut Vec<CheckResult>) {
    if input.policy_context_parse_error().is_some() {
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

    let expected: BTreeSet<_> = expected_type_bans(input.profile_name(), input.garde_enabled())
        .into_iter()
        .collect();
    let mut extra_count = 0usize;
    for found in section.entries.into_iter().map(|entry| entry.path) {
        if !expected.contains(found.as_str()) {
            extra_count += 1;
            results.push(
                CheckResult::from_parts(
                    ID.to_owned(),
                    Severity::Info,
                    "extra type ban".to_owned(),
                    format!("Additional type ban `{found}` beyond baseline."),
                    Some(input.config.rel_path.clone()),
                    None,
                    false,
                )
                .as_inventory(),
            );
        }
    }

    if malformed_count == 0 && extra_count == 0 {
        results.push(
            CheckResult::from_parts(
                ID.to_owned(),
                Severity::Info,
                "no extra type bans".to_owned(),
                "No additional type bans beyond the managed baseline.".to_owned(),
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

mod rs_clippy_07_extra_type_ban_tests;
