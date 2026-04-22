use g3rs_release_types::{G3RsReleaseConfigCrate, G3RsReleaseConfigRepo};
use guardrail3_check_types::G3CheckResult;

use crate::support::info;

const ID: &str = "RS-RELEASE-CONFIG-24";

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
    let linux_release_target_present = repo.is_some_and(|repo| {
        crate::support::crate_linux_release_target_present(repo, krate, publishable_binary_count)
    });

    let (title, message) = if linux_release_target_present {
        (
            format!("{}: linux release target present", krate.name),
            "A workflow includes a Linux target.".to_owned(),
        )
    } else {
        (
            format!("{}: no linux release target", krate.name),
            "No workflow includes a Linux target for binary release.".to_owned(),
        )
    };

    results.push(info(ID, title, message, &krate.cargo_rel_path));
}
