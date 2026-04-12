use g3rs_deps_config_checks_types::G3RsDepsConfigChecksInput;
use guardrail3_check_types::G3CheckResult;

use crate::support::{info, is_workspace_tooling, tool_installed, warn};

const ID: &str = "RS-DEPS-CONFIG-08";

pub(crate) fn check(input: &G3RsDepsConfigChecksInput, results: &mut Vec<G3CheckResult>) {
    if !is_workspace_tooling(input) {
        return;
    }

    if tool_installed(input, "cargo-dupes") {
        results.push(info(
            ID,
            "cargo-dupes installed",
            "`cargo-dupes` is available on PATH.".to_owned(),
            &input.crate_cargo_rel_path,
        ));
    } else {
        results.push(warn(
            ID,
            "cargo-dupes missing",
            "`cargo-dupes` was not found on PATH. Install with `cargo install cargo-dupes`."
                .to_owned(),
            &input.crate_cargo_rel_path,
        ));
    }
}

#[cfg(test)]
#[path = "rule_tests/mod.rs"]
mod tests;
