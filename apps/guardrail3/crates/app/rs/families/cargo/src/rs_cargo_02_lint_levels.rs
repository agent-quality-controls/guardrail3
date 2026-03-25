use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::PolicyRootCargoInput;
use super::lint_support::{
    EXPECTED_CLIPPY_DENY, EXPECTED_CLIPPY_GROUPS, EXPECTED_LIBRARY_RUST_LINTS, EXPECTED_RUST_LINTS,
    is_weaker, lint_level, lint_priority, policy_lints,
};

const ID: &str = "RS-CARGO-02";

pub fn check(input: &PolicyRootCargoInput<'_>, results: &mut Vec<CheckResult>) {
    let root = input.root;
    let Some(_parsed) = root.parsed.as_ref() else {
        return;
    };

    let mut violations = 0usize;

    if let Some(rust_lints) = policy_lints(root, "rust") {
        for expected in EXPECTED_RUST_LINTS {
            violations += check_expected_level(
                &root.cargo_rel_path,
                rust_lints,
                expected.name,
                expected.expected_level,
                None,
                results,
            );
        }

        if root.profile_name.as_deref() == Some("library") {
            for expected in EXPECTED_LIBRARY_RUST_LINTS {
                violations += check_expected_level(
                    &root.cargo_rel_path,
                    rust_lints,
                    expected.name,
                    expected.expected_level,
                    None,
                    results,
                );
            }
        }
    }

    if let Some(clippy_lints) = policy_lints(root, "clippy") {
        for expected in EXPECTED_CLIPPY_GROUPS {
            violations += check_expected_level(
                &root.cargo_rel_path,
                clippy_lints,
                expected.name,
                expected.expected_level,
                expected.priority,
                results,
            );
        }

        for lint_name in EXPECTED_CLIPPY_DENY {
            violations += check_expected_level(
                &root.cargo_rel_path,
                clippy_lints,
                lint_name,
                "deny",
                None,
                results,
            );
        }
    }

    if violations == 0 {
        results.push(
            CheckResult {
                id: ID.to_owned(),
                severity: Severity::Info,
                title: "lint levels match policy".to_owned(),
                message: format!(
                    "`{}` uses the expected lint levels for this policy root.",
                    root.cargo_rel_path
                ),
                file: Some(root.cargo_rel_path.clone()),
                line: None,
                inventory: false,
            }
            .as_inventory(),
        );
    }
}

fn check_expected_level(
    file: &str,
    lints: &toml::Value,
    name: &str,
    expected_level: &str,
    expected_priority: Option<i64>,
    results: &mut Vec<CheckResult>,
) -> usize {
    let mut violations = 0usize;

    if let Some(actual_level) = lint_level(lints, name) {
        if actual_level != expected_level && is_weaker(expected_level, actual_level.as_str()) {
            violations += 1;
            results.push(CheckResult {
                id: ID.to_owned(),
                severity: Severity::Error,
                title: format!("lint `{name}` weakens policy"),
                message: format!("Expected `{expected_level}`, got weaker level `{actual_level}`."),
                file: Some(file.to_owned()),
                line: None,
                inventory: false,
            });
        }
    }

    if let Some(expected_priority) = expected_priority {
        let actual_priority = lint_priority(lints, name);
        if actual_priority != Some(expected_priority) {
            violations += 1;
            results.push(CheckResult {
                id: ID.to_owned(),
                severity: Severity::Error,
                title: format!("lint `{name}` has wrong priority"),
                message: format!(
                    "Expected priority `{expected_priority}`, got `{}`.",
                    actual_priority
                        .map(|priority| priority.to_string())
                        .unwrap_or_else(|| "none".to_owned())
                ),
                file: Some(file.to_owned()),
                line: None,
                inventory: false,
            });
        }
    }

    violations
}

#[cfg(test)]
#[path = "rs_cargo_02_lint_levels_tests/mod.rs"]
mod tests;
