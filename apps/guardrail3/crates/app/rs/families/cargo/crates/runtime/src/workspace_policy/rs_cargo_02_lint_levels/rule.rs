use guardrail3_domain_report::{CheckResult, Severity};

use crate::inputs::PolicyRootCargoInput;
use crate::lint_support::{
    EXPECTED_CLIPPY_DENY, EXPECTED_CLIPPY_GROUPS, EXPECTED_LIBRARY_RUST_LINTS, EXPECTED_RUST_LINTS,
    LintEntryValidity, is_weaker, lint_entry_validity, lint_level, lint_priority, policy_lints,
};

const ID: &str = "RS-CARGO-02";

pub fn check(input: &PolicyRootCargoInput<'_>, results: &mut Vec<CheckResult>) {
    let root = input.root;
    let Some(_parsed) = root.parsed.as_ref() else {
        return;
    };
    let rust_lints = policy_lints(root, "rust");
    let clippy_lints = policy_lints(root, "clippy");

    let mut violations = 0usize;

    if let Some(rust_lints) = rust_lints {
        violations += validate_family_shape(&root.cargo_rel_path, "rust", rust_lints, results);
        violations += validate_explicit_levels(&root.cargo_rel_path, "rust", rust_lints, results);

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

    if let Some(clippy_lints) = clippy_lints {
        violations += validate_family_shape(&root.cargo_rel_path, "clippy", clippy_lints, results);
        violations +=
            validate_explicit_levels(&root.cargo_rel_path, "clippy", clippy_lints, results);

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

    if violations == 0
        && rust_lints.is_some()
        && clippy_lints.is_some()
        && !root.guardrail_parse_error
    {
        results.push(
            CheckResult::from_parts(
                ID.to_owned(),
                Severity::Info,
                "lint levels match policy".to_owned(),
                format!(
                    "`{}` uses the expected lint levels for this policy root.",
                    root.cargo_rel_path
                ),
                Some(root.cargo_rel_path.clone()),
                None,
                false,
            )
            .as_inventory(),
        );
    }
}

fn validate_family_shape(
    file: &str,
    family: &str,
    lints: &toml::Value,
    results: &mut Vec<CheckResult>,
) -> usize {
    if lints.as_table().is_some() {
        return 0;
    }

    results.push(CheckResult::from_parts(
        ID.to_owned(),
        Severity::Error,
        format!("{family} lint table has invalid shape"),
        format!("`{file}` must define `[lints.{family}]` as a table of lint entries."),
        Some(file.to_owned()),
        None,
        false,
    ));
    1
}

fn validate_explicit_levels(
    file: &str,
    family: &str,
    lints: &toml::Value,
    results: &mut Vec<CheckResult>,
) -> usize {
    let Some(table) = lints.as_table() else {
        return 0;
    };

    let mut violations = 0usize;
    for (lint_name, value) in table {
        match lint_entry_validity(value) {
            LintEntryValidity::Valid => continue,
            LintEntryValidity::InvalidLevel => {
                violations += 1;
                results.push(CheckResult::from_parts(
                    ID.to_owned(),
                    Severity::Error,
                    format!("lint `{lint_name}` has invalid level"),
                    format!(
                        "`{file}` defines `{lint_name}` in `{family}` with an invalid lint level. Use one of `allow`, `warn`, `deny`, or `forbid`."
                    ),
                    Some(file.to_owned()),
                    None,
                    false,
                ));
            }
            LintEntryValidity::InvalidPriority => {
                violations += 1;
                results.push(CheckResult::from_parts(
                    ID.to_owned(),
                    Severity::Error,
                    format!("lint `{lint_name}` has invalid priority"),
                    format!(
                        "`{file}` defines `{lint_name}` in `{family}` with a non-integer `priority` value."
                    ),
                    Some(file.to_owned()),
                    None,
                    false,
                ));
            }
        }
    }

    violations
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
    if lints
        .get(name)
        .is_some_and(|value| lint_entry_validity(value) != LintEntryValidity::Valid)
    {
        return 0;
    }

    if let Some(actual_level) = lint_level(lints, name) {
        if actual_level != expected_level && is_weaker(expected_level, actual_level.as_str()) {
            violations += 1;
            results.push(CheckResult::from_parts(
                ID.to_owned(),
                Severity::Error,
                format!("lint `{name}` weakens policy"),
                format!("Expected `{expected_level}`, got weaker level `{actual_level}`. Change `{name}` to `{expected_level}` in `{file}`."),
                Some(file.to_owned()),
                None,
                false,
            ));
        }
    }

    if let Some(expected_priority) = expected_priority {
        let actual_priority = lint_priority(lints, name);
        if actual_priority != Some(expected_priority) {
            violations += 1;
            results.push(CheckResult::from_parts(
                ID.to_owned(),
                Severity::Error,
                format!("lint `{name}` has wrong priority"),
                format!(
                    "Expected priority `{expected_priority}`, got `{}`. Set the priority to `{expected_priority}`.",
                    actual_priority
                        .map(|priority| priority.to_string())
                        .unwrap_or_else(|| "none".to_owned())
                ),
                Some(file.to_owned()),
                None,
                false,
            ));
        }
    }

    violations
}
