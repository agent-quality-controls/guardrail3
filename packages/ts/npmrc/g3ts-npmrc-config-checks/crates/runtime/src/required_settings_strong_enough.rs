use g3ts_npmrc_types::G3TsNpmrcChecksInput;
use guardrail3_check_types::G3CheckResult;

use crate::support::{error, info, parsed_root, weakened_required_settings};

/// Stable rule identifier reported on each emitted result.
const ID: &str = "g3ts-npmrc/required-settings-strong-enough";

/// Run the rule and append any results to `results`.
pub(crate) fn check(input: &G3TsNpmrcChecksInput, results: &mut Vec<G3CheckResult>) {
    let Some(snapshot) = parsed_root(input) else {
        return;
    };

    let weakened = weakened_required_settings(snapshot);
    if weakened.is_empty() {
        results.push(info(
            ID,
            "root .npmrc baseline settings are strong enough",
            "The root .npmrc keeps the required baseline values.".to_owned(),
            &snapshot.rel_path,
        ));
        return;
    }

    for (key, actual, expected) in weakened {
        results.push(error(
            ID,
            "root .npmrc setting is weaker than baseline",
            format!("Root .npmrc setting `{key}` is `{actual}` but must be `{expected}`."),
            &snapshot.rel_path,
        ));
    }
}
