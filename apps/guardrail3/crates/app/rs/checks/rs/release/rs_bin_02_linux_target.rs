use crate::domain::report::{CheckResult, Severity};

use super::facts::RepoReleaseFacts;
use super::inputs::PublishableCrateReleaseInput;

const ID: &str = "RS-BIN-02";

pub fn check(
    input: &PublishableCrateReleaseInput<'_>,
    repos: &[RepoReleaseFacts],
    results: &mut Vec<CheckResult>,
) {
    let krate = input.krate;
    if !krate.publishable || !krate.is_binary {
        return;
    }
    let workflow = repos
        .iter()
        .flat_map(|repo| repo.workflows.iter())
        .find(|workflow| workflow.has_linux_target);
    match workflow {
        Some(workflow) => results.push(
            CheckResult {
                id: ID.to_owned(),
                severity: Severity::Info,
                title: format!("{}: linux release target present", krate.name),
                message: format!("Workflow `{}` includes a Linux target.", workflow.rel_path),
                file: Some(workflow.rel_path.clone()),
                line: None,
                inventory: false,
            }
            .as_inventory(),
        ),
        None => results.push(
            CheckResult {
                id: ID.to_owned(),
                severity: Severity::Info,
                title: format!("{}: no linux release target", krate.name),
                message: "No workflow includes a Linux target for binary release.".to_owned(),
                file: Some(krate.cargo_rel_path.clone()),
                line: None,
                inventory: false,
            }
            .as_inventory(),
        ),
    }
}

#[cfg(test)]
#[path = "rs_bin_02_linux_target_tests/mod.rs"]
mod tests;
