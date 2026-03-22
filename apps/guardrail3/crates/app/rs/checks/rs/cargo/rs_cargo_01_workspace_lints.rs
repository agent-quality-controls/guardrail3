use crate::domain::report::{CheckResult, Severity};

use super::inputs::WorkspaceCargoInput;
use super::lint_support::{
    EXPECTED_CLIPPY_DENY, EXPECTED_CLIPPY_GROUPS, EXPECTED_LIBRARY_RUST_LINTS,
    EXPECTED_RUST_LINTS, lint_level, workspace_lints,
};

const ID: &str = "RS-CARGO-01";

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

    let rust_lints = workspace_lints(parsed, "rust");
    let clippy_lints = workspace_lints(parsed, "clippy");
    let mut missing = 0usize;

    if rust_lints.is_none() {
        missing += 1;
        results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Error,
            title: "workspace rust lints missing".to_owned(),
            message: "Missing `[workspace.lints.rust]`.".to_owned(),
            file: Some(input.workspace.rel_path.clone()),
            line: None,
            inventory: false,
        });
    }

    if clippy_lints.is_none() {
        missing += 1;
        results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Error,
            title: "workspace clippy lints missing".to_owned(),
            message: "Missing `[workspace.lints.clippy]`.".to_owned(),
            file: Some(input.workspace.rel_path.clone()),
            line: None,
            inventory: false,
        });
    }

    if let Some(rust_lints) = rust_lints {
        for expected in EXPECTED_RUST_LINTS {
            if lint_level(rust_lints, expected.name).is_none() {
                missing += 1;
                results.push(CheckResult {
                    id: ID.to_owned(),
                    severity: Severity::Error,
                    title: format!("missing workspace rust lint `{}`", expected.name),
                    message: format!(
                        "Expected `{}` in `[workspace.lints.rust]`.",
                        expected.name
                    ),
                    file: Some(input.workspace.rel_path.clone()),
                    line: None,
                    inventory: false,
                });
            }
        }

        if input.workspace.profile_name.as_deref() == Some("library") {
            for expected in EXPECTED_LIBRARY_RUST_LINTS {
                if lint_level(rust_lints, expected.name).is_none() {
                    missing += 1;
                    results.push(CheckResult {
                        id: ID.to_owned(),
                        severity: Severity::Error,
                        title: format!("missing library rust lint `{}`", expected.name),
                        message: format!(
                            "Library profile expects `{}` in `[workspace.lints.rust]`.",
                            expected.name
                        ),
                        file: Some(input.workspace.rel_path.clone()),
                        line: None,
                        inventory: false,
                    });
                }
            }
        }
    }

    if let Some(clippy_lints) = clippy_lints {
        for expected in EXPECTED_CLIPPY_GROUPS {
            if lint_level(clippy_lints, expected.name).is_none() {
                missing += 1;
                results.push(CheckResult {
                    id: ID.to_owned(),
                    severity: Severity::Error,
                    title: format!("missing clippy lint group `{}`", expected.name),
                    message: format!(
                        "Expected `{}` in `[workspace.lints.clippy]`.",
                        expected.name
                    ),
                    file: Some(input.workspace.rel_path.clone()),
                    line: None,
                    inventory: false,
                });
            }
        }

        for lint_name in EXPECTED_CLIPPY_DENY {
            if lint_level(clippy_lints, lint_name).is_none() {
                missing += 1;
                results.push(CheckResult {
                    id: ID.to_owned(),
                    severity: Severity::Error,
                    title: format!("missing clippy deny lint `{lint_name}`"),
                    message: format!(
                        "Expected `{lint_name} = \"deny\"` in `[workspace.lints.clippy]`."
                    ),
                    file: Some(input.workspace.rel_path.clone()),
                    line: None,
                    inventory: false,
                });
            }
        }
    }

    if missing == 0 {
        results.push(
            CheckResult {
                id: ID.to_owned(),
                severity: Severity::Info,
                title: "workspace lint completeness satisfied".to_owned(),
                message: "Workspace rust and clippy lint inventories are complete.".to_owned(),
                file: Some(input.workspace.rel_path.clone()),
                line: None,
                inventory: false,
            }
            .as_inventory(),
        );
    }
}
