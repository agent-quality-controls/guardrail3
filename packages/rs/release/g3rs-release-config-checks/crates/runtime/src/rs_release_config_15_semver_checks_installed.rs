use g3rs_release_config_checks_types::G3RsReleaseConfigRepo;
use guardrail3_check_types::G3CheckResult;

use crate::support::{info, warn};

const ID: &str = "RS-RELEASE-CONFIG-15";

pub(crate) fn check(repo: &G3RsReleaseConfigRepo, results: &mut Vec<G3CheckResult>) {
    if repo.publishable_count == 0 {
        return;
    }

    if repo.semver_checks_installed {
        results.push(info(
            ID,
            "cargo-semver-checks installed",
            String::new(),
            &repo.cargo_rel_path,
        ));
    } else {
        results.push(warn(
            ID,
            "cargo-semver-checks missing",
            "`cargo-semver-checks` is not available on PATH. Install with `cargo install cargo-semver-checks`.".to_owned(),
            &repo.cargo_rel_path,
        ));
    }
}
