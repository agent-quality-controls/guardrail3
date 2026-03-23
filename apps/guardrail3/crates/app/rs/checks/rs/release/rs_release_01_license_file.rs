use crate::domain::report::{CheckResult, Severity};

use super::inputs::RepoReleaseInput;

const ID: &str = "RS-RELEASE-01";

pub fn check(input: &RepoReleaseInput<'_>, results: &mut Vec<CheckResult>) {
    match &input.repo.license_rel_path {
        Some(rel_path) => results.push(
            CheckResult {
                id: ID.to_owned(),
                severity: Severity::Info,
                title: "LICENSE file exists".to_owned(),
                message: format!("Repo root includes `{rel_path}`."),
                file: Some(rel_path.clone()),
                line: None,
                inventory: false,
            }
            .as_inventory(),
        ),
        None => results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Error,
            title: "LICENSE file missing".to_owned(),
            message: "Repo root is missing LICENSE material (`LICENSE`, `LICENSE-MIT`, `LICENSE-APACHE`, or `LICENSE.md`).".to_owned(),
            file: Some(input.repo.cargo_rel_path.clone()),
            line: None,
            inventory: false,
        }),
    }
}

#[cfg(test)]
#[path = "rs_release_01_license_file_tests/mod.rs"]
mod tests;
