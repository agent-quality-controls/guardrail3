use deny_toml_parser::types::DenyToml;
use guardrail3_check_types::G3CheckResult;

use crate::support::expectations::expected_confidence_threshold;
use crate::support::findings::{inventory, warn};

/// Rule identifier emitted by this check.
const ID: &str = "g3rs-deny/confidence-threshold";

/// Runs the rule and appends any findings to `results`.
pub(crate) fn check(deny_rel_path: &str, deny: &DenyToml, results: &mut Vec<G3CheckResult>) {
    let Some(licenses) = deny.licenses.as_ref() else {
        return;
    };

    let expected = expected_confidence_threshold();
    match licenses.confidence_threshold {
        Some(value) if value < expected => results.push(warn(
            ID,
            "confidence-threshold weaker than baseline",
            format!("`{deny_rel_path}` sets `confidence-threshold = {value}`."),
            deny_rel_path,
        )),
        Some(value) if value > expected => results.push(inventory(
            ID,
            "confidence-threshold stricter than baseline",
            format!("`{deny_rel_path}` sets `confidence-threshold = {value}`."),
            deny_rel_path,
        )),
        Some(_) => {}
        None => results.push(warn(
            ID,
            "confidence-threshold missing or invalid",
            format!("`{deny_rel_path}` must set `confidence-threshold >= 0.8`."),
            deny_rel_path,
        )),
    }
}
