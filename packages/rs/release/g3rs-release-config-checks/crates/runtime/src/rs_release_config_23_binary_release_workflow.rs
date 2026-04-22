use g3rs_release_types::{G3RsReleaseConfigCrate, G3RsReleaseConfigRepo};
use guardrail3_check_types::G3CheckResult;

use crate::support::info;

const ID: &str = "RS-RELEASE-CONFIG-23";

pub(crate) fn check(
    repo: Option<&G3RsReleaseConfigRepo>,
    crates: &[G3RsReleaseConfigCrate],
    krate: &G3RsReleaseConfigCrate,
    results: &mut Vec<G3CheckResult>,
) {
    if !crate::support::crate_publishable(krate) || !krate.is_binary {
        return;
    }

    let publishable_binary_count = crate::support::repo_publishable_binary_crate_count(crates);
    let binary_release_workflow_present = repo.is_some_and(|repo| {
        crate::support::crate_binary_release_workflow_present(repo, krate, publishable_binary_count)
    });

    let (title, message) = if binary_release_workflow_present {
        (
            format!("{}: binary release workflow present", krate.name),
            "A workflow builds release binaries and uses a GitHub release action.".to_owned(),
        )
    } else {
        (
            format!("{}: no binary release workflow", krate.name),
            "No workflow builds a release binary and publishes it via GitHub Releases.".to_owned(),
        )
    };

    results.push(info(ID, title, message, &krate.cargo_rel_path));
}
