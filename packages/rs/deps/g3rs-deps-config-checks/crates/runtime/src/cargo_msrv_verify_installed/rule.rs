use g3rs_deps_types::G3RsDepsConfigChecksInput;
use guardrail3_check_types::G3CheckResult;

use crate::support::{error, info, is_workspace_tooling, tool_installed};

/// Rule identifier emitted by this check.
const ID: &str = "g3rs-deps/cargo-msrv-verify-installed";

/// Runs the rule and appends any findings to `results`.
pub(crate) fn check(input: &G3RsDepsConfigChecksInput, results: &mut Vec<G3CheckResult>) {
    if !is_workspace_tooling(input) {
        return;
    }

    if tool_installed(input, "cargo-msrv") {
        results.push(info(
            ID,
            "cargo-msrv installed",
            "`cargo-msrv` is available on PATH for `cargo msrv verify --rust-version <workspace rust-version> -- cargo check --locked`."
                .to_owned(),
            &input.crate_cargo_rel_path,
        ));
    } else {
        results.push(error(
            ID,
            "cargo-msrv missing",
            "`cargo-msrv` was not found on PATH. Install it so hooks can run \
             `cargo msrv verify --rust-version <workspace rust-version> -- cargo check --locked` before CI discovers rust-version drift."
                .to_owned(),
            &input.crate_cargo_rel_path,
        ));
    }
}
