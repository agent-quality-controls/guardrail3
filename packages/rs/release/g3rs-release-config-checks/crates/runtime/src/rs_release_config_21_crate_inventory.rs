use g3rs_release_types::{G3RsReleaseConfigCrate, G3RsReleaseConfigRepo};
use guardrail3_check_types::G3CheckResult;

use crate::support::info;

const ID: &str = "RS-RELEASE-CONFIG-21";

pub(crate) fn check(
    repo: &G3RsReleaseConfigRepo,
    crates: &[G3RsReleaseConfigCrate],
    results: &mut Vec<G3CheckResult>,
) {
    let publishable_count = crate::support::repo_publishable_count(crates);
    if publishable_count == 0 {
        return;
    }

    results.push(info(
        ID,
        "Crate inventory",
        format!(
            "Repo has {} publishable crate(s) and {} non-publishable crate(s).",
            publishable_count,
            crate::support::repo_non_publishable_count(crates)
        ),
        &repo.cargo_rel_path,
    ));
}
