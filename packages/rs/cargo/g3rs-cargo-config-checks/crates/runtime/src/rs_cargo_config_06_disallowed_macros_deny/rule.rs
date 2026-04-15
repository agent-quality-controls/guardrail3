use cargo_toml_parser::CargoToml;
use guardrail3_check_types::G3CheckResult;

use crate::support::{cargo_role, error, info, lint_level, policy_lints, policy_lints_table_label, CargoRole};

const ID: &str = "RS-CARGO-CONFIG-06";

pub(crate) fn check(cargo_rel_path: &str, cargo: &CargoToml, results: &mut Vec<G3CheckResult>) {
    if matches!(cargo_role(cargo), CargoRole::Other) {
        return;
    }

    let Some(clippy_lints) = policy_lints(cargo, "clippy") else {
        return;
    };

    match lint_level(clippy_lints, "disallowed_macros") {
        Some("deny" | "forbid") => {
            results.push(info(
                ID,
                "disallowed macros lint enforced",
                format!(
                    "`{}` enforces `clippy::disallowed_macros = \"deny\"`.",
                    policy_lints_table_label(cargo, "clippy")
                ),
                cargo_rel_path,
            ));
        }
        Some(other) => {
            results.push(error(
                ID,
                "disallowed macros lint weakened",
                format!(
                    "`{}` sets `disallowed_macros` to `{other}` instead of `deny`.",
                    policy_lints_table_label(cargo, "clippy")
                ),
                cargo_rel_path,
            ));
        }
        None => {
            results.push(error(
                ID,
                "disallowed macros lint missing",
                format!(
                    "`{}` must define `disallowed_macros = \"deny\"` so macro bans are enforceable.",
                    policy_lints_table_label(cargo, "clippy")
                ),
                cargo_rel_path,
            ));
        }
    }
}
