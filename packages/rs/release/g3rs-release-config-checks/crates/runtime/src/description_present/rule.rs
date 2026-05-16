use g3rs_release_types::G3RsReleaseConfigCrate;
use guardrail3_check_types::G3CheckResult;

use crate::support::{error, info};

/// `ID` constant.
const ID: &str = "g3rs-release/description-present";

/// `check` function.
pub(crate) fn check(krate: &G3RsReleaseConfigCrate, results: &mut Vec<G3CheckResult>) {
    if !crate::support::crate_publishable(krate) {
        return;
    }

    if crate::support::crate_description_present(krate) {
        results.push(info(
            ID,
            format!("{}: description present", krate.name),
            String::new(),
            &krate.cargo_rel_path,
        ));
    } else {
        results.push(error(
            ID,
            format!("{}: missing description", krate.name),
            "Publishable crates must have a description field in [package].".to_owned(),
            &krate.cargo_rel_path,
        ));
    }
}
