use g3rs_release_config_checks_types::G3RsReleaseConfigRepo;
use guardrail3_check_types::G3CheckResult;

use crate::support::info;

const ID: &str = "RS-RELEASE-CONFIG-17";

pub(crate) fn check(repo: &G3RsReleaseConfigRepo, results: &mut Vec<G3CheckResult>) {
    if repo.publishable_count == 0 {
        return;
    }

    if repo.release_profile_settings.is_empty() {
        return;
    }

    results.push(info(
        ID,
        "Release profile inventory",
        format!(
            "Root `[profile.release]` settings: {}.",
            repo.release_profile_settings.join(", ")
        ),
        &repo.cargo_rel_path,
    ));
}
