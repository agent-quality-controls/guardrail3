use clippy_toml_parser::ClippyToml;
use guardrail3_check_types::G3CheckResult;

use crate::support::{REQWEST_JSON_BAN, disallowed_method_paths, info, warn};

const ID: &str = "RS-GARDE-CONFIG-04";

pub(crate) fn check(clippy_rel_path: &str, clippy: &ClippyToml, results: &mut Vec<G3CheckResult>) {
    let found = disallowed_method_paths(clippy);

    if found.contains(REQWEST_JSON_BAN) {
        results.push(info(
            ID,
            "reqwest garde ban present",
            "`reqwest::Response::json` is banned in the covering clippy configuration.",
            clippy_rel_path,
        ));
        return;
    }

    results.push(warn(
        ID,
        "missing reqwest garde ban",
        "Missing `reqwest::Response::json` from `disallowed-methods`. Add it to `disallowed-methods` in clippy.toml.",
        clippy_rel_path,
    ));
}

#[cfg(test)]
#[path = "rule_tests/mod.rs"]
mod tests;
