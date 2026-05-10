use g3ts_jscpd_types::G3TsJscpdChecksInput;
use guardrail3_check_types::G3CheckResult;

use crate::support::{error, info, missing_required_ignores, parsed_root};

/// Stable rule identifier reported on each emitted result.
const ID: &str = "g3ts-jscpd/required-ignores";

/// Run the rule and append any results to `results`.
pub(crate) fn check(input: &G3TsJscpdChecksInput, results: &mut Vec<G3CheckResult>) {
    let Some(snapshot) = parsed_root(input) else {
        return;
    };

    let missing = missing_required_ignores(snapshot);
    if missing.is_empty() {
        results.push(info(
            ID,
            "jscpd required ignore patterns present",
            "The root `.jscpd.json` includes the required ignore-pattern baseline.".to_owned(),
            &snapshot.rel_path,
        ));
        return;
    }

    results.push(error(
        ID,
        "jscpd required ignore patterns missing",
        format!(
            "Root `.jscpd.json` is missing required ignore patterns: {}.",
            missing.join(", ")
        ),
        &snapshot.rel_path,
    ));
}
