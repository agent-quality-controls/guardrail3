use clippy_toml_parser::types::ClippyToml;
use guardrail3_check_types::G3CheckResult;

use crate::support::{ADDITIONAL_METHOD_BANS, disallowed_method_paths, info, missing_bans, warn};

const ID: &str = "g3rs-garde/additional-method-bans";

pub(crate) fn check(clippy_rel_path: &str, clippy: &ClippyToml, results: &mut Vec<G3CheckResult>) {
    let found = disallowed_method_paths(clippy);
    let missing = missing_bans(&found, ADDITIONAL_METHOD_BANS);

    if missing.is_empty() {
        results.push(info(
            ID,
            "additional garde method bans present",
            "All additional garde deserialization entry-point bans are present in the covering clippy configuration.",
            Some(clippy_rel_path),
        ));
        return;
    }

    results.push(warn(
        ID,
        "missing additional garde method bans",
        format!(
            "Missing additional deserialization bans from `disallowed-methods`: {}. Add these entries to `disallowed-methods` in clippy.toml.",
            missing.join(", ")
        ),
        Some(clippy_rel_path),
    ));
}

pub(crate) fn check_unverifiable(
    clippy_rel_path: Option<&str>,
    message: &str,
    results: &mut Vec<G3CheckResult>,
) {
    results.push(warn(
        ID,
        "cannot verify additional garde method bans",
        message,
        clippy_rel_path,
    ));
}

#[cfg(test)]
#[path = "rule_tests/mod.rs"] // reason: owned sidecar tests for file module.
mod rule_tests;
