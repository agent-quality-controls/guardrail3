use g3_deps_content_checks_types::G3DepsContentChecksInput;
use guardrail3_check_types::G3CheckResult;

use crate::support::{allowlist_present, crate_name, info, warn, workspace_is_library};

const ID: &str = "RS-DEPS-08";

pub(crate) fn check(input: &G3DepsContentChecksInput, results: &mut Vec<G3CheckResult>) {
    let crate_name = crate_name(&input.crate_cargo_rel_path, &input.crate_cargo);

    if allowlist_present(input) {
        results.push(info(
            ID,
            "dependency allowlist present",
            format!("Crate `{crate_name}` has an `allowed_deps` policy."),
            &input.crate_cargo_rel_path,
        ));
        return;
    }

    if workspace_is_library(input) {
        results.push(warn(
            ID,
            "dependency allowlist missing",
            format!(
                "Crate `{crate_name}` has no `allowed_deps` policy. Add an `allowed_deps` list for this workspace in guardrail3-rs.toml."
            ),
            &input.crate_cargo_rel_path,
        ));
    }
}

#[cfg(test)]
#[path = "rule_tests/mod.rs"]
mod tests;
