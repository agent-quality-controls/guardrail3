use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::RepoReleaseInput;

const ID: &str = "RS-PUB-12";

pub fn check(input: &RepoReleaseInput<'_>, results: &mut Vec<CheckResult>) {
    results.push(
        CheckResult::from_parts(
            ID.to_owned(),
            Severity::Info,
            "Crate inventory".to_owned(),
            format!(
                "Repo has {} publishable crate(s) and {} non-publishable crate(s).",
                input.repo.publishable_count, input.repo.non_publishable_count
            ),
            Some(input.repo.cargo_rel_path.clone()),
            None,
            false,
        )
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
#[path = "rs_pub_12_crate_inventory_tests/mod.rs"]
mod rs_pub_12_crate_inventory_tests;
