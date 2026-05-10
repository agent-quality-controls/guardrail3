use g3ts_npmrc_types::G3TsNpmrcChecksInput;
use guardrail3_check_types::G3CheckResult;

use crate::support::{extra_settings, info, parsed_root};

/// Stable rule identifier reported on each emitted result.
const ID: &str = "g3ts-npmrc/extra-settings-inventory";

/// Run the rule and append any results to `results`.
pub(crate) fn check(input: &G3TsNpmrcChecksInput, results: &mut Vec<G3CheckResult>) {
    let Some(snapshot) = parsed_root(input) else {
        return;
    };

    let extra = extra_settings(snapshot);
    if extra.is_empty() {
        results.push(info(
            ID,
            "root .npmrc keeps only baseline settings",
            "The root .npmrc does not add extra settings beyond the current baseline.".to_owned(),
            &snapshot.rel_path,
        ));
        return;
    }

    for (key, value) in extra {
        results.push(info(
            ID,
            "root .npmrc has extra setting",
            format!(
                "Extra root .npmrc setting `{key}={value}` is outside the current baseline. Keep it only if it is intentional."
            ),
            &snapshot.rel_path,
        ));
    }
}
