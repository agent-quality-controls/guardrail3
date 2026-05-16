use g3rs_release_types::G3RsReleaseConfigCrate;
use guardrail3_check_types::G3CheckResult;

use crate::support::{info, warn};

/// `ID` constant.
const ID: &str = "g3rs-release/docs-rs-metadata";

/// `check` function.
pub(crate) fn check(krate: &G3RsReleaseConfigCrate, results: &mut Vec<G3CheckResult>) {
    if !crate::support::crate_publishable(krate) || !krate.is_library {
        return;
    }

    if crate::support::crate_docs_rs_present(krate) {
        results.push(info(
            ID,
            format!("{}: docs.rs metadata present", krate.name),
            String::new(),
            &krate.cargo_rel_path,
        ));
    } else {
        results.push(warn(
            ID,
            format!("{}: docs.rs metadata missing", krate.name),
            "Library crates should have [package.metadata.docs.rs] for docs.rs configuration."
                .to_owned(),
            &krate.cargo_rel_path,
        ));
    }
}
