use crate::domain::report::{CheckResult, Severity};

use super::inputs::RepoReleaseInput;

const ID: &str = "RS-PUB-12";

pub fn check(input: &RepoReleaseInput<'_>, results: &mut Vec<CheckResult>) {
    results.push(
        CheckResult {
            id: ID.to_owned(),
            severity: Severity::Info,
            title: "Crate inventory".to_owned(),
            message: format!(
                "Repo has {} publishable crate(s) and {} non-publishable crate(s).",
                input.repo.publishable_count, input.repo.non_publishable_count
            ),
            file: Some(input.repo.cargo_rel_path.clone()),
            line: None,
            inventory: false,
        }
        .as_inventory(),
    );
}

#[cfg(test)]
#[path = "rs_pub_12_crate_inventory_tests/mod.rs"]
mod tests;
