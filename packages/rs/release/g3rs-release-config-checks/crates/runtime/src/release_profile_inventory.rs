use g3rs_release_types::{G3RsReleaseConfigCrate, G3RsReleaseConfigRepo};
use guardrail3_check_types::G3CheckResult;

use crate::support::info;

/// `ID` constant.
const ID: &str = "g3rs-release/release-profile-inventory";

/// `check` function.
pub(crate) fn check(
    repo: &G3RsReleaseConfigRepo,
    crates: &[G3RsReleaseConfigCrate],
    results: &mut Vec<G3CheckResult>,
) {
    if crate::support::repo_publishable_count(crates) == 0 {
        return;
    }

    let release_profile_settings = crate::support::repo_release_profile_settings(repo);
    if release_profile_settings.is_empty() {
        return;
    }

    results.push(info(
        ID,
        "Release profile inventory",
        format!(
            "Root `[profile.release]` settings: {}.",
            release_profile_settings.join(", ")
        ),
        &repo.cargo_rel_path,
    ));
}
