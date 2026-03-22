use crate::domain::report::{CheckResult, Severity};

use super::inputs::WorkspaceCargoInput;
use super::lint_support::{
    EXPECTED_CLIPPY_DENY, EXPECTED_CLIPPY_GROUPS, EXPECTED_LIBRARY_RUST_LINTS, EXPECTED_RUST_LINTS,
    is_weaker, lint_level, lint_priority, workspace_lints,
};

const ID: &str = "RS-CARGO-02";

pub fn check(input: &WorkspaceCargoInput<'_>, results: &mut Vec<CheckResult>) {
    let Some(parsed) = input.workspace.parsed.as_ref() else {
        if let Some(parse_error) = &input.workspace.parse_error {
            results.push(CheckResult {
                id: ID.to_owned(),
                severity: Severity::Error,
                title: "workspace Cargo.toml parse error".to_owned(),
                message: format!("Failed to parse workspace Cargo.toml: {parse_error}"),
                file: Some(input.workspace.rel_path.clone()),
                line: None,
                inventory: false,
            });
        }
        return;
    };

    let mut violations = 0usize;

    if let Some(rust_lints) = workspace_lints(parsed, "rust") {
        for expected in EXPECTED_RUST_LINTS {
            violations += check_expected_level(
                input.workspace.rel_path.as_str(),
                rust_lints,
                expected.name,
                expected.expected_level,
                None,
                results,
            );
        }

        if input.workspace.profile_name.as_deref() == Some("library") {
            for expected in EXPECTED_LIBRARY_RUST_LINTS {
                violations += check_expected_level(
                    input.workspace.rel_path.as_str(),
                    rust_lints,
                    expected.name,
                    expected.expected_level,
                    None,
                    results,
                );
            }
        }
    }

    if let Some(clippy_lints) = workspace_lints(parsed, "clippy") {
        for expected in EXPECTED_CLIPPY_GROUPS {
            violations += check_expected_level(
                input.workspace.rel_path.as_str(),
                clippy_lints,
                expected.name,
                expected.expected_level,
                expected.priority,
                results,
            );
        }

        for lint_name in EXPECTED_CLIPPY_DENY {
            violations += check_expected_level(
                input.workspace.rel_path.as_str(),
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
                title: "workspace lint levels match policy".to_owned(),
                message: "Workspace lint levels and group priorities match the expected policy."
                    .to_owned(),
                file: Some(input.workspace.rel_path.clone()),
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
        if actual_level != expected_level {
            violations += 1;
            results.push(CheckResult {
                id: ID.to_owned(),
                severity: if is_weaker(expected_level, actual_level.as_str()) {
                    Severity::Error
                } else {
                    Severity::Warn
                },
                title: format!("lint `{name}` has wrong level"),
                message: format!("Expected `{expected_level}`, got `{actual_level}`."),
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
                severity: Severity::Warn,
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
