use g3rs_release_types::G3RsReleaseConfigCrate;
use guardrail3_check_types::G3CheckResult;

use crate::support::error;

/// `ID` constant.
const ID: &str = "g3rs-release/accidentally-publishable";

/// `check` function.
pub(crate) fn check(krate: &G3RsReleaseConfigCrate, results: &mut Vec<G3CheckResult>) {
    if !crate::support::crate_publishable(krate) {
        return;
    }

    if crate::support::crate_description_present(krate)
        || crate::support::crate_license_present(krate)
        || crate::support::crate_repository_present(krate)
    {
        return;
    }

    results.push(error(
        ID,
        format!("{} may be accidentally publishable", krate.name),
        "Crate is publishable but has no description, license, or repository. \
         If this crate is not intended for publication, add `publish = false` to [package]."
            .to_owned(),
        &krate.cargo_rel_path,
    ));
}
