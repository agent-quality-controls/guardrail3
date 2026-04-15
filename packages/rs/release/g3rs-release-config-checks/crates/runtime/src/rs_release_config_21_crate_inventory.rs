use g3rs_release_config_checks_types::G3RsReleaseConfigRepo;
use guardrail3_check_types::G3CheckResult;

use crate::support::info;

const ID: &str = "RS-RELEASE-CONFIG-21";

pub(crate) fn check(repo: &G3RsReleaseConfigRepo, results: &mut Vec<G3CheckResult>) {
    if repo.publishable_count == 0 {
        return;
    }

    results.push(info(
        ID,
        "Crate inventory",
        format!(
            "Repo has {} publishable crate(s) and {} non-publishable crate(s).",
            repo.publishable_count, repo.non_publishable_count
        ),
        &repo.cargo_rel_path,
    ));
}
