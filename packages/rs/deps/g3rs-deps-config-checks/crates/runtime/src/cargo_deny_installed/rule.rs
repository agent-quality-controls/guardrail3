use g3rs_deps_types::G3RsDepsConfigChecksInput;
use guardrail3_check_types::G3CheckResult;

use crate::support::{error, info, is_workspace_tooling, tool_installed};

/// Rule identifier emitted by this check.
const ID: &str = "g3rs-deps/cargo-deny-installed";

/// Runs the rule and appends any findings to `results`.
pub(crate) fn check(input: &G3RsDepsConfigChecksInput, results: &mut Vec<G3CheckResult>) {
    if !is_workspace_tooling(input) {
        return;
    }

    if tool_installed(input, "cargo-deny") {
        results.push(info(
            ID,
            "cargo-deny installed",
            "`cargo-deny` is available on PATH.".to_owned(),
            &input.crate_cargo_rel_path,
        ));
    } else {
        results.push(error(
            ID,
            "cargo-deny missing",
            "`cargo-deny` was not found on PATH. Install with `cargo install cargo-deny`."
                .to_owned(),
            &input.crate_cargo_rel_path,
        ));
    }
}
