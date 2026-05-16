use cargo_toml_parser::types::CargoToml;
use guardrail3_check_types::G3CheckResult;

use crate::support::{error, has_garde_dependency, info};

/// Rule identifier.
const ID: &str = "g3rs-garde/dependency-present";

/// Run this rule and append findings to `results`.
pub(crate) fn check(cargo_rel_path: &str, cargo: &CargoToml, results: &mut Vec<G3CheckResult>) {
    if has_garde_dependency(cargo) {
        results.push(info(
            ID,
            "garde dependency found",
            format!(
                "garde is present in `{cargo_rel_path}` for this workspace root. Garde-specific boundary checks are active."
            ),
            Some(cargo_rel_path),
        ));
        return;
    }

    results.push(error(
        ID,
        "garde dependency missing",
        format!(
            "Missing `garde` dependency in `{cargo_rel_path}`. Add `garde` to `[dependencies]` or `[workspace.dependencies]` in this Cargo.toml."
        ),
        Some(cargo_rel_path),
    ));
}
