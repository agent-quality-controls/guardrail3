use clippy_toml_parser::ClippyToml;
use guardrail3_check_types::G3CheckResult;

use crate::support::{EXTRACTOR_TYPE_BANS, disallowed_type_paths, info, missing_bans, warn};

const ID: &str = "RS-GARDE-CONFIG-03";

pub(crate) fn check(clippy_rel_path: &str, clippy: &ClippyToml, results: &mut Vec<G3CheckResult>) {
    let found = disallowed_type_paths(clippy);
    let missing = missing_bans(&found, EXTRACTOR_TYPE_BANS);

    if missing.is_empty() {
        results.push(info(
            ID,
            "garde extractor bans present",
            "All required Axum extractor bans are present in the covering clippy configuration.",
            clippy_rel_path,
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
        clippy_rel_path,
    ));
}

#[cfg(test)]
#[path = "rule_tests/mod.rs"]
mod tests;
