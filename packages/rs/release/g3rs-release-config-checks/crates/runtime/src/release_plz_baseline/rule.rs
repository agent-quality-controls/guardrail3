use g3rs_release_types::{G3RsReleaseConfigCrate, G3RsReleaseConfigRepo};
use guardrail3_check_types::G3CheckResult;

use crate::support::{info, warn};

const ID: &str = "g3rs-release/release-plz-baseline";

pub(crate) fn check(
    repo: &G3RsReleaseConfigRepo,
    crates: &[G3RsReleaseConfigCrate],
    results: &mut Vec<G3CheckResult>,
) {
    if crate::support::repo_publishable_count(crates) == 0 {
        return;
    }

    if !repo.release_plz_exists {
        return;
    }

    let Some(release_plz) = repo.release_plz.as_ref() else {
        return;
    };
    let file = &repo.release_plz_rel_path;
    let mut issues = 0usize;

    let workspace = release_plz.workspace.as_ref();
    if workspace.is_none() {
        issues += 1;
        results.push(warn(
            ID,
            "release-plz: missing [workspace] section".to_owned(),
            "release-plz.toml should have a [workspace] section.".to_owned(),
            file,
        ));
    }

    let changelog_ok = workspace
        .and_then(|ws| ws.changelog_config.as_deref())
        .is_some_and(|value| value == "cliff.toml");
    if !changelog_ok {
        issues += 1;
        results.push(warn(
            ID,
            "release-plz: changelog_config should be \"cliff.toml\"".to_owned(),
            "Set changelog_config = \"cliff.toml\" in [workspace].".to_owned(),
            file,
        ));
    }

    let git_release_ok = workspace
        .and_then(|ws| ws.git_release_enable)
        .is_some_and(|value| value);
    if !git_release_ok {
        issues += 1;
        results.push(warn(
            ID,
            "release-plz: git_release_enable should be true".to_owned(),
            "Set git_release_enable = true in [workspace].".to_owned(),
            file,
        ));
    }

    let release_always_ok = workspace
        .and_then(|ws| ws.release_always)
        .is_some_and(|value| !value);
    if !release_always_ok {
        issues += 1;
        results.push(warn(
            ID,
            "release-plz: release_always should be false".to_owned(),
            "Set release_always = false in [workspace].".to_owned(),
            file,
        ));
    }

    let release_plz_package_names = crate::support::repo_release_plz_package_names(repo);
    for crate_name in
        crate::support::repo_publishable_crate_names(crates).difference(&release_plz_package_names)
    {
        issues += 1;
        results.push(warn(
            ID,
            format!("release-plz missing crate `{crate_name}`"),
            format!(
                "Publishable crate `{crate_name}` is missing from `release-plz.toml` `[[package]]` coverage. Add a `[[package]]` entry for `{crate_name}` in release-plz.toml."
            ),
            file,
        ));
    }

    if issues == 0 {
        results.push(info(
            ID,
            "release-plz: baseline configuration correct".to_owned(),
            String::new(),
            file,
        ));
    }
}

#[cfg(test)]
#[path = "rule_tests/mod.rs"] // reason: owned sidecar tests for file module.
mod rule_tests;

#[cfg(test)]
pub(crate) fn run_check(release_plz_toml: &str) -> Vec<guardrail3_check_types::G3CheckResult> {
    let input = crate::lib_tests::test_support::config_input_for_repo(Some(release_plz_toml), None);
    let mut results = Vec::new();
    crate::release_plz_baseline::check(&input.repo_checks[0], &input.crate_checks, &mut results);
    results
}

#[cfg(test)]
pub(crate) const GOLDEN: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/fixtures/golden_release_plz.toml"
));
