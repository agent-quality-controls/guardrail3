use crate::domain::report::{CheckResult, Severity};

use super::inputs::RepoReleaseInput;

const ID: &str = "RS-RELEASE-04";

pub fn check(input: &RepoReleaseInput<'_>, results: &mut Vec<CheckResult>) {
    let repo = input.repo;
    if repo.cliff_exists {
        results.push(
            CheckResult {
                id: ID.to_owned(),
                severity: Severity::Info,
                title: "cliff.toml exists".to_owned(),
                message: "Repo root includes `cliff.toml`.".to_owned(),
                file: Some(repo.cliff_rel_path.clone()),
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
        title: "cliff.toml missing".to_owned(),
        message: "Repo root is missing `cliff.toml`.".to_owned(),
        file: Some(repo.cliff_rel_path.clone()),
        line: None,
        inventory: false,
    });
}

#[cfg(test)]
#[path = "rs_release_04_cliff_exists_tests.rs"]
mod tests;
