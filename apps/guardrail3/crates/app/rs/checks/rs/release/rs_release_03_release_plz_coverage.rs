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

    let Some(workspace) = repo
        .release_plz_parsed
        .as_ref()
        .and_then(|parsed| parsed.get("workspace"))
    else {
        return;
    };

    if workspace
        .get("changelog_config")
        .and_then(toml::Value::as_str)
        != Some("cliff.toml")
    {
        results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Warn,
            title: "release-plz.toml missing canonical changelog_config".to_owned(),
            message:
                "`release-plz.toml` should set `[workspace].changelog_config = \"cliff.toml\"`."
                    .to_owned(),
            file: Some(repo.release_plz_rel_path.clone()),
            line: None,
            inventory: false,
        });
    }
    if workspace
        .get("git_release_enable")
        .and_then(toml::Value::as_bool)
        != Some(true)
    {
        results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Warn,
            title: "release-plz.toml missing git_release_enable = true".to_owned(),
            message: "`release-plz.toml` should set `[workspace].git_release_enable = true`."
                .to_owned(),
            file: Some(repo.release_plz_rel_path.clone()),
            line: None,
            inventory: false,
        });
    }
    if workspace
        .get("release_always")
        .and_then(toml::Value::as_bool)
        != Some(false)
    {
        results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Warn,
            title: "release-plz.toml missing release_always = false".to_owned(),
            message: "`release-plz.toml` should set `[workspace].release_always = false`."
                .to_owned(),
            file: Some(repo.release_plz_rel_path.clone()),
            line: None,
            inventory: false,
        });
    }

    let missing = repo
        .publishable_crate_names
        .difference(&repo.release_plz_package_names)
        .cloned()
        .collect::<Vec<_>>();
    if missing.is_empty()
        && !results
            .iter()
            .any(|result| result.id == ID && !result.inventory)
    {
        results.push(
            CheckResult {
                id: ID.to_owned(),
                severity: Severity::Info,
                title: "release-plz baseline and package coverage complete".to_owned(),
                message:
                    "`release-plz.toml` has the canonical workspace baseline and covers all publishable crates."
                        .to_owned(),
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
#[path = "rs_release_03_release_plz_coverage_tests/mod.rs"]
mod tests;
