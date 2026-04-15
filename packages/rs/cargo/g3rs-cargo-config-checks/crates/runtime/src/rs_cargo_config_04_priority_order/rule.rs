use cargo_toml_parser::CargoToml;
use guardrail3_check_types::G3CheckResult;

use crate::support::{
    cargo_role, info, lint_level, lint_priority, policy_lints, warn, CargoRole,
    EXPECTED_CLIPPY_DENY,
};

const ID: &str = "RS-CARGO-CONFIG-04";

pub(crate) fn check(cargo_rel_path: &str, cargo: &CargoToml, results: &mut Vec<G3CheckResult>) {
    if matches!(cargo_role(cargo), CargoRole::Other) {
        return;
    }

    let Some(clippy_lints) = policy_lints(cargo, "clippy") else {
        return;
    };

    let mut violations = 0usize;
    for lint_name in EXPECTED_CLIPPY_DENY {
        if lint_priority(clippy_lints, lint_name).is_some_and(|priority| priority < 0) {
            violations += 1;
            results.push(warn(
                ID,
                format!("specific lint `{lint_name}` has negative priority"),
                format!(
                    "Specific clippy deny `{lint_name}` should keep default priority so groups do not override it. Remove the negative `priority` from `{lint_name}` or set it to `0` or higher."
                ),
                cargo_rel_path,
            ));
        }
    }

    let targeted_entries_present = EXPECTED_CLIPPY_DENY
        .iter()
        .all(|lint_name| lint_level(clippy_lints, lint_name).is_some());

    if violations == 0 && targeted_entries_present {
        results.push(info(
            ID,
            "specific lint priorities are safe",
            "Specific clippy deny lints do not use negative priority.",
            cargo_rel_path,
        ));
    }
}
