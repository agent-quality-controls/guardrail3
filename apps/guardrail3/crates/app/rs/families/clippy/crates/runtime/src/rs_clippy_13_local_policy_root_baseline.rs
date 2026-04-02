use std::collections::BTreeSet;

#[cfg(test)]
use guardrail3_app_rs_family_view::FamilyView as ProjectTree;
use guardrail3_domain_report::{CheckResult, Severity};

use super::clippy_support::{
    EXPECTED_MACRO_BANS, THRESHOLD_EXPECTATIONS, ban_paths, expected_method_bans,
    expected_type_bans, threshold_value,
};
use super::inputs::ConfigClippyInput;

const ID: &str = "RS-CLIPPY-13";

pub fn check(input: &ConfigClippyInput<'_>, results: &mut Vec<CheckResult>) {
    if input.config.rel_dir.is_empty() {
        return;
    }
    if input.policy_context_parse_error().is_some() {
        return;
    }

    let Some(parsed) = input.config.parsed.as_ref() else {
        return;
    };

    let expected_methods: BTreeSet<_> = expected_method_bans(input.garde_enabled())
        .into_iter()
        .map(str::to_owned)
        .collect();
    let expected_types: BTreeSet<_> =
        expected_type_bans(input.profile_name(), input.garde_enabled())
            .into_iter()
            .map(str::to_owned)
            .collect();
    let expected_macros: BTreeSet<_> = EXPECTED_MACRO_BANS
        .iter()
        .map(|path| (*path).to_owned())
        .collect();
    let found_methods: BTreeSet<_> = ban_paths(parsed, "disallowed-methods")
        .into_iter()
        .collect();
    let found_types: BTreeSet<_> = ban_paths(parsed, "disallowed-types").into_iter().collect();
    let found_macros: BTreeSet<_> = ban_paths(parsed, "disallowed-macros").into_iter().collect();

    let mut missing_sections = Vec::new();
    if THRESHOLD_EXPECTATIONS
        .iter()
        .any(|threshold| threshold_value(parsed, threshold.key) != Some(threshold.expected))
    {
        missing_sections.push("thresholds");
    }
    if expected_methods
        .iter()
        .any(|path| !found_methods.contains(path))
    {
        missing_sections.push("disallowed-methods");
    }
    if expected_types
        .iter()
        .any(|path| !found_types.contains(path))
    {
        missing_sections.push("disallowed-types");
    }
    if expected_macros
        .iter()
        .any(|path| !found_macros.contains(path))
    {
        missing_sections.push("disallowed-macros");
    }
    if missing_sections.is_empty() {
        results.push(
            CheckResult::from_parts(
                ID.to_owned(),
                Severity::Info,
                "local clippy policy root is self-contained".to_owned(),
                format!(
                    "`{}` contains the full managed clippy baseline for its subtree.",
                    input.config.rel_path
                ),
                Some(input.config.rel_path.clone()),
                None,
                false,
            )
            .as_inventory(),
        );
        return;
    }

    missing_sections.sort();
    missing_sections.dedup();

    results.push(CheckResult::from_parts(
        ID.to_owned(),
        Severity::Error,
        "local clippy policy root drops managed baseline".to_owned(),
        format!(
            "`{}` replaces inherited clippy policy but is incomplete. Missing or wrong managed sections: {}.",
            input.config.rel_path,
            missing_sections.join(", ")
        ),
        Some(input.config.rel_path.clone()),
        None,
        false,
    ));
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
#[path = "rs_clippy_13_local_policy_root_baseline_tests/mod.rs"]
// reason: test-only sidecar module wiring
mod rs_clippy_13_local_policy_root_baseline_tests;
