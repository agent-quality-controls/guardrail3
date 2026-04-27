use deny_toml_parser::types::DenyToml;
use guardrail3_check_types::G3CheckResult;

use crate::support::findings::warn;

const ADVISORY_IGNORE_THRESHOLD: usize = 5;
const ID: &str = "g3rs-deny/ignore-accumulation";

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
mod rule_tests;
