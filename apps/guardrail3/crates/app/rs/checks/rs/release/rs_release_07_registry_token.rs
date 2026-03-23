use crate::domain::report::{CheckResult, Severity};

use super::inputs::RepoReleaseInput;

const ID: &str = "RS-RELEASE-07";

pub fn check(input: &RepoReleaseInput<'_>, results: &mut Vec<CheckResult>) {
    let workflow = input
        .repo
        .workflows
        .iter()
        .find(|workflow| workflow.has_registry_token);
    match workflow {
        Some(workflow) => results.push(
            CheckResult {
                id: ID.to_owned(),
                severity: Severity::Info,
                title: "CARGO_REGISTRY_TOKEN wired in workflow".to_owned(),
                message: format!(
                    "Workflow `{}` structurally references `CARGO_REGISTRY_TOKEN`.",
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
            title: "CARGO_REGISTRY_TOKEN missing from workflows".to_owned(),
            message: "No workflow structurally wires `CARGO_REGISTRY_TOKEN` into release steps.".to_owned(),
            file: None,
            line: None,
            inventory: false,
        }),
    }
}

#[cfg(test)]
#[path = "rs_release_07_registry_token_tests.rs"]
mod tests;
