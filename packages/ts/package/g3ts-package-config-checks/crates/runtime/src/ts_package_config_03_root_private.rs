use g3ts_package_types::G3TsPackageChecksInput;
use guardrail3_check_types::G3CheckResult;

use crate::support::{error, info, parsed_root};

const ID: &str = "TS-PACKAGE-CONFIG-03";

pub(crate) fn check(input: &G3TsPackageChecksInput, results: &mut Vec<G3CheckResult>) {
    let Some(snapshot) = parsed_root(input) else {
        return;
    };

    if snapshot.private_field == Some(true) {
        results.push(info(
            ID,
            "root package.json is private",
            "The root package manifest sets `private: true`.".to_owned(),
            &snapshot.rel_path,
        ));
        return;
    }

    results.push(error(
        ID,
        "root package.json is publishable",
        "The root package manifest must set `private: true`.".to_owned(),
        &snapshot.rel_path,
    ));
}
