use clippy_toml_parser::types::ClippyToml;
use guardrail3_check_types::G3CheckResult;

use crate::support::{REQWEST_JSON_BAN, disallowed_method_paths, info, warn};

/// Rule identifier.
const ID: &str = "g3rs-garde/reqwest-json-ban";

/// Run this rule and append findings to `results`.
pub(crate) fn check(clippy_rel_path: &str, clippy: &ClippyToml, results: &mut Vec<G3CheckResult>) {
    let found = disallowed_method_paths(clippy);

    if found.contains(REQWEST_JSON_BAN) {
        results.push(info(
            ID,
            "reqwest garde ban present",
            "`reqwest::Response::json` is banned in the covering clippy configuration.",
            Some(clippy_rel_path),
        ));
        return;
    }

    results.push(warn(
        ID,
        "missing reqwest garde ban",
        "Missing `reqwest::Response::json` from `disallowed-methods`. Add it to `disallowed-methods` in clippy.toml.",
        Some(clippy_rel_path),
    ));
}

/// Emit an info finding when the rule cannot verify its inputs.
pub(crate) fn check_unverifiable(
    clippy_rel_path: Option<&str>,
    message: &str,
    results: &mut Vec<G3CheckResult>,
) {
    results.push(warn(
        ID,
        "cannot verify reqwest garde ban",
        message,
        clippy_rel_path,
    ));
}

#[cfg(test)]
#[path = "rule_tests/mod.rs"] // reason: owned sidecar tests for file module.
mod rule_tests;
