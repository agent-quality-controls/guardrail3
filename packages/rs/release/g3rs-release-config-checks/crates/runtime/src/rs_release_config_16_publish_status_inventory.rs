use g3rs_release_config_checks_types::G3RsReleaseConfigRepo;
use guardrail3_check_types::G3CheckResult;

use crate::support::info;

const ID: &str = "RS-RELEASE-CONFIG-16";

pub(crate) fn check(repo: &G3RsReleaseConfigRepo, results: &mut Vec<G3CheckResult>) {
    if repo.publishable_count == 0 {
        return;
    }

    let Some(publish) = &repo.publish_setting else {
        return;
    };

    results.push(info(
        ID,
        "Publish status inventory",
        format!("Root Cargo metadata sets `publish = {publish}`."),
        &repo.cargo_rel_path,
    ));
}
