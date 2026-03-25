use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::RepoReleaseInput;

const ID: &str = "RS-RELEASE-02";

pub fn check(input: &RepoReleaseInput<'_>, results: &mut Vec<CheckResult>) {
    let repo = input.repo;
    if repo.release_plz_exists {
        results.push(
            CheckResult {
                id: ID.to_owned(),
                severity: Severity::Info,
                title: "release-plz.toml exists".to_owned(),
                message: "Repo root includes `release-plz.toml`.".to_owned(),
                file: Some(repo.release_plz_rel_path.clone()),
                line: None,
                inventory: false,
            }
            .as_inventory(),
        );
        return;
    }
    results.push(CheckResult {
        id: ID.to_owned(),
        severity: Severity::Warn,
        title: "release-plz.toml missing".to_owned(),
        message: "Repo root is missing `release-plz.toml`.".to_owned(),
        file: Some(repo.release_plz_rel_path.clone()),
        line: None,
        inventory: false,
    });
}

#[cfg(test)]
#[path = "rs_release_02_release_plz_exists_tests/mod.rs"]
mod tests;
