use g3rs_deps_types::G3RsDepsConfigChecksInput;
use guardrail3_check_types::G3CheckResult;

use crate::support::{allowlist_present, info, warn, workspace_is_library};

const ID: &str = "g3rs-deps/library-allowlist-present";

pub(crate) fn check(input: &G3RsDepsConfigChecksInput, results: &mut Vec<G3CheckResult>) {
    if allowlist_present(input) {
        results.push(info(
            ID,
            "dependency allowlist present",
            format!("Crate `{}` has an `allowed_deps` policy.", input.crate_name),
            &input.crate_cargo_rel_path,
        ));
        return;
    }

    if workspace_is_library(input) {
        results.push(warn(
            ID,
            "dependency allowlist missing",
            format!(
                "Crate `{}` has no `allowed_deps` policy. Add an `allowed_deps` list in guardrail3-rs.toml.",
                input.crate_name
            ),
            &input.crate_cargo_rel_path,
        ));
    }
}

#[cfg(test)]
#[path = "rule_tests/mod.rs"] // reason: owned sidecar tests for file module.
mod rule_tests;
