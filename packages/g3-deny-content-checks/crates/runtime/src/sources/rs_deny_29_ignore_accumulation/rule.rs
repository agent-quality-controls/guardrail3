use deny_toml_parser::DenyToml;
use guardrail3_check_types::G3CheckResult;

use crate::support::warn;

const ADVISORY_IGNORE_THRESHOLD: usize = 5;
const ID: &str = "RS-DENY-29";

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

#[cfg(test)]
#[path = "rule_tests/mod.rs"]
mod tests;
