use g3rs_release_types::G3RsReleaseConfigCrate;
use guardrail3_check_types::G3CheckResult;

use crate::support::{error, info};

/// `ID` constant.
const ID: &str = "g3rs-release/repository-present";

/// `check` function.
pub(crate) fn check(krate: &G3RsReleaseConfigCrate, results: &mut Vec<G3CheckResult>) {
    if !crate::support::crate_publishable(krate) {
        return;
    }

    if crate::support::crate_repository_present(krate) {
        results.push(info(
            ID,
            format!("{}: repository present", krate.name),
            String::new(),
            &krate.cargo_rel_path,
        ));
    } else {
        results.push(error(
            ID,
            format!("{}: missing repository", krate.name),
            "Publishable crates must have a repository field in [package].".to_owned(),
            &krate.cargo_rel_path,
        ));
    }
}
