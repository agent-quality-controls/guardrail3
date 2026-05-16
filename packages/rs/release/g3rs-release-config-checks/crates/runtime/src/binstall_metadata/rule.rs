use g3rs_release_types::G3RsReleaseConfigCrate;
use guardrail3_check_types::G3CheckResult;

use crate::support::{info, warn};

/// `ID` constant.
const ID: &str = "g3rs-release/binstall-metadata";

/// `check` function.
pub(crate) fn check(krate: &G3RsReleaseConfigCrate, results: &mut Vec<G3CheckResult>) {
    if !crate::support::crate_publishable(krate) || !krate.is_binary {
        return;
    }

    if crate::support::crate_has_binstall_metadata(krate) {
        results.push(info(
            ID,
            format!("{}: binstall metadata present", krate.name),
            String::new(),
            &krate.cargo_rel_path,
        ));
    } else {
        results.push(warn(
            ID,
            format!("{}: missing binstall metadata", krate.name),
            "Binary crates should have [package.metadata.binstall] for cargo-binstall support."
                .to_owned(),
            &krate.cargo_rel_path,
        ));
    }
}
