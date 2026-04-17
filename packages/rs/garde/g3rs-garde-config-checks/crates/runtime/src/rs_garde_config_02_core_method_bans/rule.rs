use clippy_toml_parser::types::ClippyToml;
use guardrail3_check_types::G3CheckResult;

use crate::support::{CORE_METHOD_BANS, disallowed_method_paths, info, missing_bans, warn};

const ID: &str = "RS-GARDE-CONFIG-02";

pub(crate) fn check(clippy_rel_path: &str, clippy: &ClippyToml, results: &mut Vec<G3CheckResult>) {
    let found = disallowed_method_paths(clippy);
    let missing = missing_bans(&found, CORE_METHOD_BANS);

    if missing.is_empty() {
        results.push(info(
            ID,
            "core garde method bans present",
            "All core serde/toml/yaml deserialization bans are present in the covering clippy configuration.",
            Some(clippy_rel_path),
        ));
        return;
    }

    results.push(warn(
        ID,
        "missing core garde method bans",
        format!(
            "Missing core deserialization bans from `disallowed-methods`: {}. Add these entries to `disallowed-methods` in clippy.toml.",
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
        "cannot verify core garde method bans",
        message,
        clippy_rel_path,
    ));
}

#[cfg(test)]
#[path = "rule_tests/mod.rs"]
mod tests;
