use g3rs_release_types::{G3RsReleaseConfigCrate, G3RsReleaseConfigRepo};
use guardrail3_check_types::G3CheckResult;

use crate::support::{info, warn};

const ID: &str = "RS-RELEASE-CONFIG-15";

pub(crate) fn check(
    repo: &G3RsReleaseConfigRepo,
    crates: &[G3RsReleaseConfigCrate],
    results: &mut Vec<G3CheckResult>,
) {
    if crate::support::repo_publishable_count(crates) == 0 {
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
