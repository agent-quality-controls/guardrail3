use g3rs_deps_types::G3RsDepsConfigChecksInput;
use guardrail3_check_types::G3CheckResult;

use crate::support::{error, info, is_workspace_tooling, tool_installed};

const ID: &str = "RS-DEPS-CONFIG-09";

pub(crate) fn check(input: &G3RsDepsConfigChecksInput, results: &mut Vec<G3CheckResult>) {
    if !is_workspace_tooling(input) {
        return;
    }

    if tool_installed(input, "gitleaks") {
        results.push(info(
            ID,
            "gitleaks installed",
            "`gitleaks` is available on PATH.".to_owned(),
            &input.crate_cargo_rel_path,
        ));
    } else {
        results.push(error(
            ID,
            "gitleaks missing",
            "`gitleaks` was not found on PATH. Install with `brew install gitleaks` or download from GitHub."
                .to_owned(),
            &input.crate_cargo_rel_path,
        ));
    }
}

#[cfg(test)]
#[path = "rule_tests/mod.rs"] // reason: owned sidecar tests for file module.
mod rule_tests;
