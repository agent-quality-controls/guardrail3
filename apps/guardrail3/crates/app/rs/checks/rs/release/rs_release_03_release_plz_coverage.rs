use crate::domain::report::{CheckResult, Severity};

use super::inputs::RepoReleaseInput;

const ID: &str = "RS-RELEASE-03";

pub fn check(input: &RepoReleaseInput<'_>, results: &mut Vec<CheckResult>) {
    let repo = input.repo;
    if !repo.release_plz_exists {
        return;
    }
    if repo.release_plz_parsed.is_none() {
        return;
    }
    if !repo.release_plz_has_workspace {
        results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Warn,
            title: "release-plz.toml missing [workspace]".to_owned(),
            message: "`release-plz.toml` is missing a `[workspace]` section.".to_owned(),
            file: Some(repo.release_plz_rel_path.clone()),
            line: None,
            inventory: false,
        });
        return;
    }

    let missing = repo
        .publishable_crate_names
        .difference(&repo.release_plz_package_names)
        .cloned()
        .collect::<Vec<_>>();
    if missing.is_empty() {
        results.push(
            CheckResult {
                id: ID.to_owned(),
                severity: Severity::Info,
                title: "release-plz package coverage complete".to_owned(),
                message: "All publishable crates have `[[package]]` entries in `release-plz.toml`.".to_owned(),
                file: Some(repo.release_plz_rel_path.clone()),
                line: None,
                inventory: false,
            }
            .as_inventory(),
        );
        return;
    }
    for crate_name in missing {
        results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Warn,
            title: format!("release-plz missing crate `{crate_name}`"),
            message: format!(
                "Publishable crate `{crate_name}` is missing from `release-plz.toml` `[[package]]` coverage."
            ),
            file: Some(repo.release_plz_rel_path.clone()),
            line: None,
            inventory: false,
        });
    }
}

#[cfg(test)]
#[path = "rs_release_03_release_plz_coverage_tests.rs"]
mod tests;
