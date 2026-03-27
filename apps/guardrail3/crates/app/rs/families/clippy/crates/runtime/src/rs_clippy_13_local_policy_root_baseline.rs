use std::collections::BTreeSet;

use guardrail3_domain_report::{CheckResult, Severity};

use super::clippy_support::{
    EXPECTED_MACRO_BANS, THRESHOLD_EXPECTATIONS, ban_paths, expected_bool_value,
    expected_method_bans, expected_type_bans, threshold_value,
};
use super::inputs::ConfigClippyInput;

const ID: &str = "RS-CLIPPY-13";

pub fn check(input: &ConfigClippyInput<'_>, results: &mut Vec<CheckResult>) {
    if input.config.rel_dir.is_empty() {
        return;
    }

    let Some(parsed) = input.config.parsed.as_ref() else {
        if let Some(parse_error) = &input.config.parse_error {
            results.push(CheckResult {
                id: ID.to_owned(),
                severity: Severity::Error,
                title: "local clippy policy root is not parseable".to_owned(),
                message: format!(
                    "`{}` replaces inherited policy for its subtree but could not be parsed: {parse_error}",
                    input.config.rel_path
                ),
                file: Some(input.config.rel_path.clone()),
                line: None,
                inventory: false,
            });
        }
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
    for key in [
        "avoid-breaking-exported-api",
        "allow-dbg-in-tests",
        "allow-expect-in-tests",
        "allow-print-in-tests",
        "allow-unwrap-in-tests",
    ] {
        if parsed.get(key).and_then(toml::Value::as_bool) != expected_bool_value(key) {
            missing_sections.push(key);
        }
    }

    if missing_sections.is_empty() {
        results.push(
            CheckResult {
                id: ID.to_owned(),
                severity: Severity::Info,
                title: "local clippy policy root is self-contained".to_owned(),
                message: format!(
                    "`{}` contains the full managed clippy baseline for its subtree.",
                    input.config.rel_path
                ),
                file: Some(input.config.rel_path.clone()),
                line: None,
                inventory: false,
            }
            .as_inventory(),
        );
        return;
    }

    missing_sections.sort();
    missing_sections.dedup();

    results.push(CheckResult {
        id: ID.to_owned(),
        severity: Severity::Error,
        title: "local clippy policy root drops managed baseline".to_owned(),
        message: format!(
            "`{}` replaces inherited clippy policy but is incomplete. Missing or wrong managed sections: {}.",
            input.config.rel_path,
            missing_sections.join(", ")
        ),
        file: Some(input.config.rel_path.clone()),
        line: None,
        inventory: false,
    });
}

#[cfg(test)]
#[path = "rs_clippy_13_local_policy_root_baseline_tests/mod.rs"]
mod rs_clippy_13_local_policy_root_baseline_tests;
