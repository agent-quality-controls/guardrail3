use deny_toml_parser::types::DenyToml;
use guardrail3_check_types::G3CheckResult;

use crate::support::findings::warn;

/// Constant value used by the surrounding module.
const ADVISORY_IGNORE_THRESHOLD: usize = 5;
/// Rule identifier emitted by this check.
const ID: &str = "g3rs-deny/ignore-accumulation";

/// Runs the rule and appends any findings to `results`.
pub(crate) fn check(deny_rel_path: &str, deny: &DenyToml, results: &mut Vec<G3CheckResult>) {
    let Some(advisories) = deny.advisories.as_ref() else {
        return;
    };

    if advisories.ignore.len() > ADVISORY_IGNORE_THRESHOLD {
        results.push(warn(
            ID,
            "advisory ignore list is large",
            format!(
                "`{deny_rel_path}` has {} `[advisories].ignore` entries (threshold: {}).",
                advisories.ignore.len(),
                ADVISORY_IGNORE_THRESHOLD
            ),
            deny_rel_path,
        ));
    }
}
