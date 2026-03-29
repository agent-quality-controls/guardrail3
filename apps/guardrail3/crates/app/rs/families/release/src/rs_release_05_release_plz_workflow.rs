use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::RepoReleaseInput;
use super::release_support::release_plz_step_present;

const ID: &str = "RS-RELEASE-05";

pub fn check(input: &RepoReleaseInput<'_>, results: &mut Vec<CheckResult>) {
    let workflow = input
        .repo
        .workflows
        .iter()
        .find(|workflow| release_plz_step_present(&workflow.analysis));
    match workflow {
        Some(workflow) => results.push(
            CheckResult {
                id: ID.to_owned(),
                severity: Severity::Info,
                title: "Release-plz workflow present".to_owned(),
                message: format!(
                    "Workflow `{}` contains an actual release-plz step.",
                    workflow.rel_path
                ),
                file: Some(workflow.rel_path.clone()),
                line: None,
                inventory: false,
            }
            .as_inventory(),
        ),
        None => results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Warn,
            title: "Release-plz workflow missing".to_owned(),
            message: "No workflow contains an actual release-plz execution step.".to_owned(),
            file: None,
            line: None,
            inventory: false,
        }),
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
#[path = "rs_release_05_release_plz_workflow_tests/mod.rs"]
mod rs_release_05_release_plz_workflow_tests;
