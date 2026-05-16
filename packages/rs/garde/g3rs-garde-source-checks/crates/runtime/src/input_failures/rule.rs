use guardrail3_check_types::G3CheckResult;

use crate::support::{InputFailureSite, error};

/// Rule identifier emitted by this check.
const ID: &str = "g3rs-garde/input-failures";

/// Runs the rule and appends any findings to `results`.
pub(crate) fn check(site: &InputFailureSite, results: &mut Vec<G3CheckResult>) {
    results.push(error(
        ID,
        "garde-family input failure",
        site.message.clone(),
        site.rel_path.as_str(),
        None,
    ));
}
