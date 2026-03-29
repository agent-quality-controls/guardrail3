use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::RepoReleaseInput;

const ID: &str = "RS-RELEASE-08";

pub fn check(input: &RepoReleaseInput<'_>, results: &mut Vec<CheckResult>) {
    results.push(if input.repo.semver_checks_installed {
        CheckResult {
            id: ID.to_owned(),
            severity: Severity::Info,
            title: "cargo-semver-checks installed".to_owned(),
            message: "`cargo-semver-checks` is available on PATH.".to_owned(),
            file: Some(input.repo.cargo_rel_path.clone()),
            line: None,
            inventory: false,
        }
        .as_inventory()
    } else {
        CheckResult {
            id: ID.to_owned(),
            severity: Severity::Warn,
            title: "cargo-semver-checks missing".to_owned(),
            message: "`cargo-semver-checks` is not available on PATH.".to_owned(),
            file: Some(input.repo.cargo_rel_path.clone()),
            line: None,
            inventory: false,
        }
    });
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
#[path = "rs_release_08_semver_checks_installed_tests/mod.rs"]
mod rs_release_08_semver_checks_installed_tests;
