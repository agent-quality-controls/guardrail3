use guardrail3_domain_report::{CheckResult, Severity};

use crate::inputs::PolicyRootCargoInput;
use crate::lint_support::{
    EXPECTED_CLIPPY_DENY, EXPECTED_CLIPPY_GROUPS, EXPECTED_LIBRARY_RUST_LINTS, EXPECTED_RUST_LINTS,
    lint_level, policy_lints, policy_lints_table_label,
};

const ID: &str = "RS-CARGO-01";

pub fn check(input: &PolicyRootCargoInput<'_>, results: &mut Vec<CheckResult>) {
    let root = input.root;
    let Some(_parsed) = root.parsed.as_ref() else {
        return;
    };

    let rust_lints = policy_lints(root, "rust");
    let clippy_lints = policy_lints(root, "clippy");
    let mut missing = 0usize;

    if rust_lints.is_none() {
        missing += 1;
        let table = policy_lints_table_label(root.kind, "rust");
        results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Error,
            "rust lint table missing".to_owned(),
            format!(
                "`{}` must define `{table}`. Add the required lint entries to `{table}`.",
                root.cargo_rel_path,
            ),
            Some(root.cargo_rel_path.clone()),
            None,
            false,
        ));
    }

    if clippy_lints.is_none() {
        missing += 1;
        let table = policy_lints_table_label(root.kind, "clippy");
        results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Error,
            title: "clippy lint table missing".to_owned(),
            message: format!(
                "`{}` must define `{table}`. Add the required lint entries to `{table}`.",
                root.cargo_rel_path,
            ),
            file: Some(root.cargo_rel_path.clone()),
            line: None,
            inventory: false,
        });
    }

    if let Some(rust_lints) = rust_lints {
        let table = policy_lints_table_label(root.kind, "rust");
        if rust_lints.as_table().is_none() {
            missing += 1;
            results.push(CheckResult::from_parts(
                ID.to_owned(),
                Severity::Error,
                "rust lint table has invalid shape".to_owned(),
                format!(
                    "`{}` defines `{table}` but it is not a TOML table. Define it as a table of lint entries.",
                    root.cargo_rel_path,
                ),
                Some(root.cargo_rel_path.clone()),
                None,
                false,
            ));
        } else {
            for expected in EXPECTED_RUST_LINTS {
                if lint_level(rust_lints, expected.name).is_none() {
                    missing += 1;
                    results.push(CheckResult {
                        id: ID.to_owned(),
                        severity: Severity::Error,
                        title: format!("missing rust lint `{}`", expected.name),
                        message: format!(
                            "`{}` must define `{}` in `{table}`. Add `{}` to `{table}`.",
                            root.cargo_rel_path, expected.name, expected.name,
                        ),
                        file: Some(root.cargo_rel_path.clone()),
                        line: None,
                        inventory: false,
                    });
                }
            }

            for expected in EXPECTED_LIBRARY_RUST_LINTS {
                if lint_level(rust_lints, expected.name).is_none() {
                    missing += 1;
                    results.push(CheckResult {
                        id: ID.to_owned(),
                        severity: Severity::Error,
                        title: format!("missing rust lint `{}`", expected.name),
                        message: format!(
                            "`{}` must define `{}` in `{table}`. Add `{}` to `{table}`.",
                            root.cargo_rel_path, expected.name, expected.name,
                        ),
                        file: Some(root.cargo_rel_path.clone()),
                        line: None,
                        inventory: false,
                    });
                }
            }
        }
    }

    if let Some(clippy_lints) = clippy_lints {
        let table = policy_lints_table_label(root.kind, "clippy");
        if clippy_lints.as_table().is_none() {
            missing += 1;
            results.push(CheckResult::from_parts(
                ID.to_owned(),
                Severity::Error,
                "clippy lint table has invalid shape".to_owned(),
                format!(
                    "`{}` defines `{table}` but it is not a TOML table. Define it as a table of lint entries.",
                    root.cargo_rel_path,
                ),
                Some(root.cargo_rel_path.clone()),
                None,
                false,
            ));
        } else {
            for expected in EXPECTED_CLIPPY_GROUPS {
                if lint_level(clippy_lints, expected.name).is_none() {
                    missing += 1;
                    results.push(CheckResult {
                        id: ID.to_owned(),
                        severity: Severity::Error,
                        title: format!("missing clippy lint group `{}`", expected.name),
                        message: format!(
                            "`{}` must define `{}` in `{table}`. Add `{}` to `{table}`.",
                            root.cargo_rel_path, expected.name, expected.name,
                        ),
                        file: Some(root.cargo_rel_path.clone()),
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
                            "`{}` must define `{lint_name}` in `{table}`. Add `{lint_name}` to `{table}`.",
                            root.cargo_rel_path,
                        ),
                        file: Some(root.cargo_rel_path.clone()),
                        line: None,
                        inventory: false,
                    });
                }
            }
        }
    }

    if missing == 0 && !root.guardrail_parse_error {
        results.push(
            CheckResult::from_parts(
                ID.to_owned(),
                Severity::Info,
                "lint completeness satisfied".to_owned(),
                format!(
                    "`{}` defines the required rust and clippy lint baseline for this {}.",
                    root.cargo_rel_path,
                    root.kind.label()
                ),
                Some(root.cargo_rel_path.clone()),
                None,
                false,
            )
            .as_inventory(),
        );
    }
}
