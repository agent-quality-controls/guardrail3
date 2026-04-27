use g3ts_npmrc_types::G3TsNpmrcChecksInput;
use guardrail3_check_types::G3CheckResult;

use crate::support::{error, info, missing_required_settings, parsed_root};

const ID: &str = "g3ts-npmrc/required-settings-present";

pub(crate) fn check(input: &G3TsNpmrcChecksInput, results: &mut Vec<G3CheckResult>) {
    let Some(snapshot) = parsed_root(input) else {
        return;
    };

    let missing = missing_required_settings(snapshot);
    if missing.is_empty() {
        results.push(info(
            ID,
            "root .npmrc baseline settings are present",
            "The root .npmrc contains the required narrow baseline settings.".to_owned(),
            &snapshot.rel_path,
        ));
        return;
    }

    results.push(error(
        ID,
        "root .npmrc baseline settings are missing",
        format!(
            "The root .npmrc is missing required settings: {}.",
            missing.join(", ")
        ),
        &snapshot.rel_path,
    ));
}
