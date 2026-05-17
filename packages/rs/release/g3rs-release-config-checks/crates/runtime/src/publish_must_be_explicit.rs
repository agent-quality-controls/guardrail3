use g3rs_release_types::G3RsReleaseConfigCrate;
use guardrail3_check_types::G3CheckResult;

use crate::support::error;

/// `ID` constant.
const ID: &str = "g3rs-release/publish-must-be-explicit";

/// `check` function.
pub(crate) fn check(krate: &G3RsReleaseConfigCrate, results: &mut Vec<G3CheckResult>) {
    if crate::support::crate_publish_declared(krate) {
        return;
    }

    results.push(error(
        ID,
        format!("{}: publish must be explicit", krate.name),
        format!(
            "Crate `{}` does not set `[package].publish`. Add `publish = true` if this crate publishes or `publish = false` if it does not.",
            krate.name
        ),
        &krate.cargo_rel_path,
    ));
}
