use cargo_toml_parser::CargoToml;
use guardrail3_check_types::G3CheckResult;

use crate::support::{
    cargo_role, error, info, lint_level, policy_lints, policy_lints_table_label, CargoRole,
    EXPECTED_CLIPPY_DENY, EXPECTED_CLIPPY_GROUPS, EXPECTED_CLIPPY_REQUIRED_ALLOW,
    EXPECTED_LIBRARY_RUST_LINTS, EXPECTED_RUST_LINTS,
};

const ID: &str = "RS-CARGO-01";

pub(crate) fn check(cargo_rel_path: &str, cargo: &CargoToml, results: &mut Vec<G3CheckResult>) {
    if matches!(cargo_role(cargo), CargoRole::Other) {
        return;
    }

    let rust_lints = policy_lints(cargo, "rust");
    let clippy_lints = policy_lints(cargo, "clippy");
    let mut missing = 0usize;

    if rust_lints.is_none() {
        missing += 1;
        let table = policy_lints_table_label(cargo, "rust");
        results.push(error(
            ID,
            "rust lint table missing",
            format!(
                "`{cargo_rel_path}` must define `{table}`. Add the required lint entries to `{table}`."
            ),
            cargo_rel_path,
        ));
    }

    if clippy_lints.is_none() {
        missing += 1;
        let table = policy_lints_table_label(cargo, "clippy");
        results.push(error(
            ID,
            "clippy lint table missing",
            format!(
                "`{cargo_rel_path}` must define `{table}`. Add the required lint entries to `{table}`."
            ),
            cargo_rel_path,
        ));
    }

    if let Some(rust_lints) = rust_lints {
        let table = policy_lints_table_label(cargo, "rust");
        for expected in EXPECTED_RUST_LINTS {
            if lint_level(rust_lints, expected.name).is_none() {
                missing += 1;
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
        }

        for expected in EXPECTED_LIBRARY_RUST_LINTS {
            if lint_level(rust_lints, expected.name).is_none() {
                missing += 1;
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
        }
    }

    if let Some(clippy_lints) = clippy_lints {
        let table = policy_lints_table_label(cargo, "clippy");
        for expected in EXPECTED_CLIPPY_GROUPS {
            if lint_level(clippy_lints, expected.name).is_none() {
                missing += 1;
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
        }

        for lint_name in EXPECTED_CLIPPY_DENY {
            if lint_level(clippy_lints, lint_name).is_none() {
                missing += 1;
                results.push(error(
                    ID,
                    format!("missing clippy lint `{lint_name}`"),
                    format!(
                        "`{cargo_rel_path}` must define `{lint_name}` in `{table}`. Add `{lint_name}` to `{table}`."
                    ),
                    cargo_rel_path,
                ));
            }
        }

        for required in EXPECTED_CLIPPY_REQUIRED_ALLOW {
            if lint_level(clippy_lints, required.name).is_none() {
                missing += 1;
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
        }
    }

    if missing == 0 {
        results.push(info(
            ID,
            "workspace lint tables present",
            format!("`{cargo_rel_path}` defines all required cargo policy lint tables and entries."),
            cargo_rel_path,
        ));
    }
}

#[cfg(test)]
#[path = "rule_tests/mod.rs"]
mod tests;
