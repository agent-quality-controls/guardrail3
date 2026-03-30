use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::RepoReleaseInput;
use super::release_support::publish_dry_run_step_present;

const ID: &str = "RS-RELEASE-06";

pub fn check(input: &RepoReleaseInput<'_>, results: &mut Vec<CheckResult>) {
    let workflow = input
        .repo
        .workflows
        .iter()
        .find(|workflow| publish_dry_run_step_present(&workflow.analysis));
    match workflow {
        Some(workflow) => results.push(
            CheckResult::from_parts(
                ID.to_owned(),
                Severity::Info,
                "Publish dry-run workflow present".to_owned(),
                format!(
                    "Workflow `{}` contains an actual `cargo publish --dry-run` step.",
                    workflow.rel_path
                ),
                Some(workflow.rel_path.clone()),
                None,
                false,
            )
            .as_inventory(),
        ),
        None => results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Warn,
            "Publish dry-run workflow missing".to_owned(),
            "No workflow contains an actual `cargo publish --dry-run` step.".to_owned(),
            None,
            None,
            false,
        )),
    }
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
pub(super) fn workflow_from_yaml(rel_path: &str, yaml: &str) -> crate::facts::WorkflowFacts {
    crate::test_fixtures::workflow_from_yaml(rel_path, yaml)
}

#[cfg(test)]
#[path = "rs_release_06_publish_dry_run_workflow_tests/mod.rs"]
mod rs_release_06_publish_dry_run_workflow_tests;
