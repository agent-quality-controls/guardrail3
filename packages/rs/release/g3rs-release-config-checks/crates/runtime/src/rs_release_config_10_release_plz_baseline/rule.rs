use g3rs_release_config_checks_types::G3RsReleaseConfigRepo;
use guardrail3_check_types::G3CheckResult;

use crate::support::{info, warn};

const ID: &str = "RS-RELEASE-CONFIG-10";

pub(crate) fn check(repo: &G3RsReleaseConfigRepo, results: &mut Vec<G3CheckResult>) {
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

    for crate_name in repo
        .publishable_crate_names
        .difference(&repo.release_plz_package_names)
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
#[path = "rule_tests/mod.rs"]
mod tests;
