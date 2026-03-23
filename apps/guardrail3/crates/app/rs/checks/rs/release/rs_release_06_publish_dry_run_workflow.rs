use crate::domain::report::{CheckResult, Severity};

use super::inputs::RepoReleaseInput;

const ID: &str = "RS-RELEASE-06";

pub fn check(input: &RepoReleaseInput<'_>, results: &mut Vec<CheckResult>) {
    let workflow = input
        .repo
        .workflows
        .iter()
        .find(|workflow| workflow.has_publish_dry_run_step);
    match workflow {
        Some(workflow) => results.push(
            CheckResult {
                id: ID.to_owned(),
                severity: Severity::Info,
                title: "Publish dry-run workflow present".to_owned(),
                message: format!(
                    "Workflow `{}` contains an actual `cargo publish --dry-run` step.",
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
            title: "Publish dry-run workflow missing".to_owned(),
            message: "No workflow contains an actual `cargo publish --dry-run` step.".to_owned(),
            file: None,
            line: None,
            inventory: false,
        }),
    }
}

#[cfg(test)]
#[path = "rs_release_06_publish_dry_run_workflow_tests/mod.rs"]
mod tests;
