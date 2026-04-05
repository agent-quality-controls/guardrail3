use cargo_toml_parser::{CargoToml, ToolLints};
use guardrail3_check_types::G3CheckResult;

use crate::support::{
    cargo_role, error, info, is_weaker, lint_level, lint_priority, policy_lints, CargoRole,
    EXPECTED_CLIPPY_DENY, EXPECTED_CLIPPY_GROUPS, EXPECTED_LIBRARY_RUST_LINTS, EXPECTED_RUST_LINTS,
    LintExpectation,
};

const ID: &str = "RS-CARGO-02";

pub(crate) fn check(cargo_rel_path: &str, cargo: &CargoToml, results: &mut Vec<G3CheckResult>) {
    if matches!(cargo_role(cargo), CargoRole::Other) {
        return;
    }

    let rust_lints = policy_lints(cargo, "rust");
    let clippy_lints = policy_lints(cargo, "clippy");
    let mut violations = 0usize;

    if let Some(rust_lints) = rust_lints {
        for expected in EXPECTED_RUST_LINTS {
            violations += check_expected(cargo_rel_path, rust_lints, expected, results);
        }
        for expected in EXPECTED_LIBRARY_RUST_LINTS {
            violations += check_expected(cargo_rel_path, rust_lints, expected, results);
        }
    }

    if let Some(clippy_lints) = clippy_lints {
        for expected in EXPECTED_CLIPPY_GROUPS {
            violations += check_expected(cargo_rel_path, clippy_lints, expected, results);
        }
        for lint_name in EXPECTED_CLIPPY_DENY {
            violations += check_expected(
                cargo_rel_path,
                clippy_lints,
                &LintExpectation {
                    name: lint_name,
                    expected_level: "deny",
                    priority: None,
                },
                results,
            );
        }
    }

    if violations == 0 && rust_lints.is_some() && clippy_lints.is_some() {
        results.push(info(
            ID,
            "lint levels match policy",
            format!("`{cargo_rel_path}` uses the expected lint levels for this Cargo policy file."),
            cargo_rel_path,
        ));
    }
}

fn check_expected(
    cargo_rel_path: &str,
    lints: &ToolLints,
    expected: &LintExpectation,
    results: &mut Vec<G3CheckResult>,
) -> usize {
    let mut violations = 0usize;

    if let Some(actual_level) = lint_level(lints, expected.name) {
        if actual_level != expected.expected_level && is_weaker(expected.expected_level, actual_level) {
            violations += 1;
            results.push(error(
                ID,
                format!("lint `{}` weakens policy", expected.name),
                format!(
                    "Expected `{}`, got weaker level `{}`. Change `{}` to `{}` in `{}`.",
                    expected.expected_level,
                    actual_level,
                    expected.name,
                    expected.expected_level,
                    cargo_rel_path
                ),
                cargo_rel_path,
            ));
        }
    }

    if let Some(expected_priority) = expected.priority {
        let actual_priority = lint_priority(lints, expected.name);
        if actual_priority != Some(expected_priority) {
            violations += 1;
            results.push(error(
                ID,
                format!("lint `{}` has wrong priority", expected.name),
                format!(
                    "Expected priority `{expected_priority}`, got `{}`. Set the priority to `{expected_priority}`.",
                    actual_priority
                        .map(|priority| priority.to_string())
                        .unwrap_or_else(|| "none".to_owned())
                ),
                cargo_rel_path,
            ));
        }
    }

    violations
}

#[cfg(test)]
#[path = "rule_tests/mod.rs"]
mod tests;
