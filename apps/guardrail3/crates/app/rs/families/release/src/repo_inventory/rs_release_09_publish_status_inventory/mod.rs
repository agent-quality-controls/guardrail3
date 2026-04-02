use guardrail3_domain_report::{CheckResult, Severity};

use crate::inputs::RepoReleaseInput;

const ID: &str = "RS-RELEASE-09";

pub fn check(input: &RepoReleaseInput<'_>, results: &mut Vec<CheckResult>) {
    let Some(publish) = &input.repo.publish_setting else {
        return;
    };
    results.push(
        CheckResult::from_parts(
            ID.to_owned(),
            Severity::Info,
            "Publish status inventory".to_owned(),
            format!("Root Cargo metadata sets `publish = {publish}`."),
            Some(input.repo.cargo_rel_path.clone()),
            None,
            false,
        )
        .as_inventory(),
    );
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
