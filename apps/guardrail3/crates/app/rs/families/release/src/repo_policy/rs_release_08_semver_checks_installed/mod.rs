use guardrail3_domain_report::{CheckResult, Severity};

use crate::inputs::RepoReleaseInput;

const ID: &str = "RS-RELEASE-08";

pub fn check(input: &RepoReleaseInput<'_>, results: &mut Vec<CheckResult>) {
    results.push(if input.repo.semver_checks_installed {
        CheckResult::from_parts(
            ID.to_owned(),
            Severity::Info,
            "cargo-semver-checks installed".to_owned(),
            "`cargo-semver-checks` is available on PATH.".to_owned(),
            Some(input.repo.cargo_rel_path.clone()),
            None,
            false,
        )
        .as_inventory()
    } else {
        CheckResult::from_parts(
            ID.to_owned(),
            Severity::Warn,
            "cargo-semver-checks missing".to_owned(),
            "`cargo-semver-checks` is not available on PATH.".to_owned(),
            Some(input.repo.cargo_rel_path.clone()),
            None,
            false,
        )
    });
}
#[cfg(test)]
pub(crate) fn repo_facts() -> crate::facts::RepoReleaseFacts {
    crate::test_fixtures::repo_facts()
}

#[cfg(test)]
pub(crate) fn repo_input(
    repo: &crate::facts::RepoReleaseFacts,
) -> crate::inputs::RepoReleaseInput<'_> {
    crate::test_fixtures::repo_input(repo)
}

#[cfg(test)]

mod tests;
