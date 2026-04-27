use g3rs_release_types::{G3RsReleaseConfigCrate, G3RsReleaseConfigRepo};
use guardrail3_check_types::G3CheckResult;

use crate::support::info;

const ID: &str = "g3rs-release/publish-status-inventory";

pub(crate) fn check(
    repo: &G3RsReleaseConfigRepo,
    crates: &[G3RsReleaseConfigCrate],
    results: &mut Vec<G3CheckResult>,
) {
    if crate::support::repo_publishable_count(crates) == 0 {
        return;
    }

    let Some(publish) = crate::support::repo_publish_setting(repo) else {
        return;
    };

    results.push(info(
        ID,
        "Publish status inventory",
        format!("Root Cargo metadata sets `publish = {publish}`."),
        &repo.cargo_rel_path,
    ));
}
