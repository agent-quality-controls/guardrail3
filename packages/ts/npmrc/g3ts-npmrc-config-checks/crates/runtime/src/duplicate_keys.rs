use g3ts_npmrc_types::G3TsNpmrcChecksInput;
use guardrail3_check_types::G3CheckResult;

use crate::support::{duplicate_keys, error, info, parsed_root};

/// Stable rule identifier reported on each emitted result.
const ID: &str = "g3ts-npmrc/duplicate-keys";

/// Run the rule and append any results to `results`.
pub(crate) fn check(input: &G3TsNpmrcChecksInput, results: &mut Vec<G3CheckResult>) {
    let Some(snapshot) = parsed_root(input) else {
        return;
    };

    let duplicates = duplicate_keys(snapshot);
    if duplicates.is_empty() {
        results.push(info(
            ID,
            "root .npmrc has no duplicate keys",
            "The root .npmrc has no duplicate key collisions.".to_owned(),
            &snapshot.rel_path,
        ));
        return;
    }

    for key in duplicates {
        results.push(error(
            ID,
            "root .npmrc has duplicate key",
            format!(
                "Duplicate root .npmrc key `{key}` is not allowed; pnpm uses the last value and can mask earlier settings."
            ),
            &snapshot.rel_path,
        ));
    }
}
