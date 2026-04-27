use clippy_toml_parser::types::ClippyToml;
use guardrail3_check_types::G3CheckResult;

use crate::support::{EXTRACTOR_TYPE_BANS, disallowed_type_paths, info, missing_bans, warn};

const ID: &str = "g3rs-garde/extractor-type-bans";

pub(crate) fn check(clippy_rel_path: &str, clippy: &ClippyToml, results: &mut Vec<G3CheckResult>) {
    let found = disallowed_type_paths(clippy);
    let missing = missing_bans(&found, EXTRACTOR_TYPE_BANS);

    if missing.is_empty() {
        results.push(info(
            ID,
            "garde extractor bans present",
            "All required Axum extractor bans are present in the covering clippy configuration.",
            Some(clippy_rel_path),
        ));
        return;
    }

    results.push(warn(
        ID,
        "missing garde extractor bans",
        format!(
            "Missing extractor type bans from `disallowed-types`: {}. Add these entries to `disallowed-types` in clippy.toml.",
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
        "cannot verify garde extractor bans",
        message,
        clippy_rel_path,
    ));
}

#[cfg(test)]
#[path = "rule_tests/mod.rs"] // reason: owned sidecar tests for file module.
mod rule_tests;
