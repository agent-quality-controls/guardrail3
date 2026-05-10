use cargo_toml_parser::types::{CargoToml, ToolLints};
use guardrail3_check_types::G3CheckResult;

use crate::support::{
    CargoRole, EXPECTED_CLIPPY_DENY, EXPECTED_CLIPPY_GROUPS, EXPECTED_CLIPPY_REQUIRED_ALLOW,
    EXPECTED_LIBRARY_RUST_LINTS, EXPECTED_RUST_LINTS, cargo_role, error, info, lint_level,
    policy_lints, policy_lints_table_label,
};

/// I D const.
const ID: &str = "g3rs-cargo/workspace-lints";

/// check fn.
pub(crate) fn check(cargo_rel_path: &str, cargo: &CargoToml, results: &mut Vec<G3CheckResult>) {
    if matches!(cargo_role(cargo), CargoRole::Other) {
        return;
    }

    let rust_lints = policy_lints(cargo, "rust");
    let clippy_lints = policy_lints(cargo, "clippy");
    let mut missing: usize = 0;

    missing = missing.saturating_add(report_missing_table(
        cargo,
        cargo_rel_path,
        "rust",
        "rust lint table missing",
        rust_lints.is_none(),
        results,
    ));
    missing = missing.saturating_add(report_missing_table(
        cargo,
        cargo_rel_path,
        "clippy",
        "clippy lint table missing",
        clippy_lints.is_none(),
        results,
    ));

    if let Some(rust_lints) = rust_lints {
        missing = missing.saturating_add(check_rust_entries(
            cargo_rel_path,
            cargo,
            rust_lints,
            results,
        ));
    }

    if let Some(clippy_lints) = clippy_lints {
        missing = missing.saturating_add(check_clippy_entries(
            cargo_rel_path,
            cargo,
            clippy_lints,
            results,
        ));
    }

    if missing == 0 {
        results.push(info(
            ID,
            "workspace lint tables present",
            format!(
                "`{cargo_rel_path}` defines all required cargo policy lint tables and entries."
            ),
            cargo_rel_path,
        ));
    }
}

/// Emit a missing-table finding when `is_missing` is true; returns the increment for `missing`.
fn report_missing_table(
    cargo: &CargoToml,
    cargo_rel_path: &str,
    family: &str,
    title: &str,
    is_missing: bool,
    results: &mut Vec<G3CheckResult>,
) -> usize {
    if !is_missing {
        return 0;
    }
    let table = policy_lints_table_label(cargo, family);
    results.push(error(
        ID,
        title.to_owned(),
        format!(
            "`{cargo_rel_path}` must define `{table}`. Add the required lint entries to `{table}`."
        ),
        cargo_rel_path,
    ));
    1
}

/// Verify every required rust lint entry is present; returns the count of missing entries.
fn check_rust_entries(
    cargo_rel_path: &str,
    cargo: &CargoToml,
    rust_lints: &ToolLints,
    results: &mut Vec<G3CheckResult>,
) -> usize {
    let table = policy_lints_table_label(cargo, "rust");
    let mut missing: usize = 0;
    for expected in EXPECTED_RUST_LINTS
        .iter()
        .chain(EXPECTED_LIBRARY_RUST_LINTS)
    {
        if lint_level(rust_lints, expected.name).is_some() {
            continue;
        }
        missing = missing.saturating_add(1);
        results.push(error(
            ID,
            format!("missing rust lint `{}`", expected.name),
            format!(
                "`{cargo_rel_path}` must define `{}` in `{table}`. Add `{}` to `{table}`.",
                expected.name, expected.name
            ),
            cargo_rel_path,
        ));
    }
    missing
}

/// Verify every required clippy group/lint/allow entry is present; returns the count missing.
fn check_clippy_entries(
    cargo_rel_path: &str,
    cargo: &CargoToml,
    clippy_lints: &ToolLints,
    results: &mut Vec<G3CheckResult>,
) -> usize {
    let table = policy_lints_table_label(cargo, "clippy");
    let mut missing: usize = 0;
    for expected in EXPECTED_CLIPPY_GROUPS {
        if lint_level(clippy_lints, expected.name).is_some() {
            continue;
        }
        missing = missing.saturating_add(1);
        results.push(error(
            ID,
            format!("missing clippy group `{}`", expected.name),
            format!(
                "`{cargo_rel_path}` must define `{}` in `{table}`. Add `{}` to `{table}`.",
                expected.name, expected.name
            ),
            cargo_rel_path,
        ));
    }
    for lint_name in EXPECTED_CLIPPY_DENY {
        if lint_level(clippy_lints, lint_name).is_some() {
            continue;
        }
        missing = missing.saturating_add(1);
        results.push(error(
            ID,
            format!("missing clippy lint `{lint_name}`"),
            format!(
                "`{cargo_rel_path}` must define `{lint_name}` in `{table}`. Add `{lint_name}` to `{table}`."
            ),
            cargo_rel_path,
        ));
    }
    for required in EXPECTED_CLIPPY_REQUIRED_ALLOW {
        if lint_level(clippy_lints, required.name).is_some() {
            continue;
        }
        missing = missing.saturating_add(1);
        results.push(error(
            ID,
            format!("missing clippy allow `{}`", required.name),
            format!(
                "`{cargo_rel_path}` must define `{}` in `{table}`. {}",
                required.name, required.reason
            ),
            cargo_rel_path,
        ));
    }
    missing
}

#[cfg(test)]
#[path = "rule_tests/mod.rs"] // reason: owned sidecar tests for file module.
mod rule_tests;
