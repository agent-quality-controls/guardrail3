use g3rs_deps_types::G3RsDepsConfigChecksInput;
use guardrail3_check_types::G3CheckResult;

use crate::support::{error, info, is_workspace_tooling, tool_installed};

const ID: &str = "g3rs-deps/cargo-machete-installed";

pub(crate) fn check(input: &G3RsDepsConfigChecksInput, results: &mut Vec<G3CheckResult>) {
    if !is_workspace_tooling(input) {
        return;
    }

    if tool_installed(input, "cargo-machete") {
        results.push(info(
            ID,
            "cargo-machete installed",
            "`cargo-machete` is available on PATH.".to_owned(),
            &input.crate_cargo_rel_path,
        ));
    } else {
        results.push(error(
            ID,
            "cargo-machete missing",
            "`cargo-machete` was not found on PATH. Install with `cargo install cargo-machete`."
                .to_owned(),
            &input.crate_cargo_rel_path,
        ));
    }
}

#[cfg(test)]
#[path = "rule_tests/mod.rs"] // reason: owned sidecar tests for file module.
mod rule_tests;
