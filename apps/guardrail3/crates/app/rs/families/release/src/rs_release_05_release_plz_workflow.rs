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
#[path = "rs_release_05_release_plz_workflow_tests/mod.rs"]
mod tests;
