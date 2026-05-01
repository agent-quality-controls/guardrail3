use g3ts_package_types::G3TsPackageChecksInput;
use guardrail3_check_types::G3CheckResult;

use crate::support::{error, info, parsed_root};

const ID: &str = "g3ts-package/validate-script-present";

pub(crate) fn check(input: &G3TsPackageChecksInput, results: &mut Vec<G3CheckResult>) {
    let Some(snapshot) = parsed_root(input) else {
        return;
    };

    if snapshot.validate_script.is_some() {
        results.push(info(
            ID,
            "validate script is present",
            "The root package manifest defines the standard `validate` script.".to_owned(),
            &snapshot.rel_path,
        ));
    } else {
        results.push(error(
            ID,
            "validate script is missing",
            "The root package manifest must define the standard `validate` script.".to_owned(),
            &snapshot.rel_path,
        ));
    }
}

#[cfg(test)]
#[path = "validate_script_present_tests/mod.rs"]
mod validate_script_present_tests;
