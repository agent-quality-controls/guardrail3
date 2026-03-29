use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::RepoReleaseInput;

const ID: &str = "RS-RELEASE-09";

pub fn check(input: &RepoReleaseInput<'_>, results: &mut Vec<CheckResult>) {
    let Some(publish) = &input.repo.publish_setting else {
        return;
    };
    results.push(
        CheckResult {
            id: ID.to_owned(),
            severity: Severity::Info,
            title: "Publish status inventory".to_owned(),
            message: format!("Root Cargo metadata sets `publish = {publish}`."),
            file: Some(input.repo.cargo_rel_path.clone()),
            line: None,
            inventory: false,
        }
        .as_inventory(),
    );
}
#[cfg(test)]
pub(super) fn repo_facts() -> crate::facts::RepoReleaseFacts {
    crate::test_fixtures::repo_facts()
}

#[cfg(test)]
pub(super) fn repo_input(
    repo: &crate::facts::RepoReleaseFacts,
) -> crate::inputs::RepoReleaseInput<'_> {
    crate::test_fixtures::repo_input(repo)
}

#[cfg(test)]
#[path = "rs_release_09_publish_status_inventory_tests/mod.rs"]
mod rs_release_09_publish_status_inventory_tests;
