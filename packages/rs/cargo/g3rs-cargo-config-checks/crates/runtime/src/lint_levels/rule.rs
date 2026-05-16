use cargo_toml_parser::{types::CargoToml, types::ToolLints};
use guardrail3_check_types::G3CheckResult;

use crate::support::{self, CargoRole, LintExpectation};

/// I D const.
const ID: &str = "g3rs-cargo/lint-levels";

/// check fn.
pub(crate) fn check(cargo_rel_path: &str, cargo: &CargoToml, results: &mut Vec<G3CheckResult>) {
    if matches!(support::cargo_role(cargo), CargoRole::Other) {
        return;
    }

    let rust_lints = support::policy_lints(cargo, "rust");
    let clippy_lints = support::policy_lints(cargo, "clippy");
    let mut violations = 0usize;

    if let Some(rust_lints) = rust_lints {
        for expected in support::EXPECTED_RUST_LINTS {
            violations = violations.saturating_add(check_expected(
                cargo_rel_path,
                rust_lints,
                expected,
                results,
            ));
        }
        for expected in support::EXPECTED_LIBRARY_RUST_LINTS {
            violations = violations.saturating_add(check_expected(
                cargo_rel_path,
                rust_lints,
                expected,
                results,
            ));
        }
    }

    if let Some(clippy_lints) = clippy_lints {
        for expected in support::EXPECTED_CLIPPY_GROUPS {
            violations = violations.saturating_add(check_expected(
                cargo_rel_path,
                clippy_lints,
                expected,
                results,
            ));
        }
        for lint_name in support::EXPECTED_CLIPPY_DENY {
            violations = violations.saturating_add(check_expected(
                cargo_rel_path,
                clippy_lints,
                &LintExpectation {
                    name: lint_name,
                    expected_level: "deny",
                    priority: None,
                },
                results,
            ));
        }
        for required in support::EXPECTED_CLIPPY_REQUIRED_ALLOW {
            violations = violations.saturating_add(check_required_allow(
                cargo_rel_path,
                clippy_lints,
                required,
                results,
            ));
        }
    }

    if violations == 0 && rust_lints.is_some() && clippy_lints.is_some() {
        results.push(support::info(
            ID,
            "lint levels match policy",
            format!("`{cargo_rel_path}` uses the expected lint levels for this Cargo policy file."),
            cargo_rel_path,
        ));
    }
}

/// check expected fn.
fn check_expected(
    cargo_rel_path: &str,
    lints: &ToolLints,
    expected: &LintExpectation,
    results: &mut Vec<G3CheckResult>,
) -> usize {
    let mut violations = 0usize;

    if let Some(actual_level) = support::lint_level(lints, expected.name) {
        if actual_level != expected.expected_level
            && support::is_weaker(expected.expected_level, actual_level)
        {
            violations = violations.saturating_add(1);
            results.push(support::error(
                ID,
                format!("lint `{}` weakens policy", expected.name),
                format!(
                    "Expected at least `{}`, got weaker level `{}`. Change `{}` to `{}` in `{}`.",
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
        if support::lint_level(lints, expected.name).is_some() {
            let actual_priority = support::lint_priority(lints, expected.name);
            if actual_priority != Some(expected_priority) {
                violations = violations.saturating_add(1);
                results.push(support::error(
                    ID,
                    format!("lint `{}` has wrong priority", expected.name),
                    format!(
                        "Expected priority `{expected_priority}`, got `{}`. Set the priority to `{expected_priority}`.",
                        actual_priority.map_or_else(|| "none".to_owned(), |priority| priority.to_string())
                    ),
                    cargo_rel_path,
                ));
            }
        }
    }

    violations
}

/// Check that a `required-allow` clippy lint is actually set to `"allow"`; returns 1 on violation.
fn check_required_allow(
    cargo_rel_path: &str,
    clippy_lints: &ToolLints,
    required: &support::RequiredAllowLint,
    results: &mut Vec<G3CheckResult>,
) -> usize {
    let Some(actual_level) = support::lint_level(clippy_lints, required.name) else {
        return 0;
    };
    if actual_level == "allow" {
        return 0;
    }
    results.push(support::error(
        ID,
        format!("lint `{}` must be allow", required.name),
        format!(
            "`{}` must be `\"allow\"` (got `\"{actual_level}\"`). Reason: {}",
            required.name, required.reason
        ),
        cargo_rel_path,
    ));
    1
}
