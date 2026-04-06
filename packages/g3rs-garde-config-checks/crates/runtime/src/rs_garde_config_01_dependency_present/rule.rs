use cargo_toml_parser::CargoToml;
use guardrail3_check_types::G3CheckResult;

use crate::support::{error, has_garde_dependency, info};

const ID: &str = "RS-GARDE-CONFIG-01";

pub(crate) fn check(cargo_rel_path: &str, cargo: &CargoToml, results: &mut Vec<G3CheckResult>) {
    if has_garde_dependency(cargo) {
        results.push(info(
            ID,
            "garde dependency found",
            format!(
                "garde is present in `{cargo_rel_path}` for this workspace root. Garde-specific boundary checks are active."
            ),
            cargo_rel_path,
        ));
        return;
    }

    results.push(error(
        ID,
        "garde dependency missing",
        format!(
            "Missing `garde` dependency in `{cargo_rel_path}`. Add `garde` to `[dependencies]` or `[workspace.dependencies]` in this Cargo.toml."
        ),
        cargo_rel_path,
    ));
}

#[cfg(test)]
#[path = "rule_tests/mod.rs"]
mod tests;
