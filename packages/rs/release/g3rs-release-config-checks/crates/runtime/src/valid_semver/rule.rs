use g3rs_release_types::G3RsReleaseConfigCrate;
use guardrail3_check_types::G3CheckResult;

use crate::support::{error, info};

/// `ID` constant.
const ID: &str = "g3rs-release/valid-semver";

/// `check` function.
pub(crate) fn check(krate: &G3RsReleaseConfigCrate, results: &mut Vec<G3CheckResult>) {
    if !crate::support::crate_publishable(krate) {
        return;
    }

    match crate::support::crate_version_string(krate) {
        Some(_) if crate::support::crate_version_valid(krate) => {
            results.push(info(
                ID,
                format!("{}: valid semver", krate.name),
                String::new(),
                &krate.cargo_rel_path,
            ));
        }
        Some(version) => {
            results.push(error(
                ID,
                format!("{}: invalid version", krate.name),
                format!("Version \"{version}\" is not valid semver (expected major.minor.patch)."),
                &krate.cargo_rel_path,
            ));
        }
        None => {
            results.push(error(
                ID,
                format!("{}: invalid version", krate.name),
                "Publishable crates must have a version field in [package].".to_owned(),
                &krate.cargo_rel_path,
            ));
        }
    }
}
