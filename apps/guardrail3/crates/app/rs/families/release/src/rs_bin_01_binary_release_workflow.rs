use guardrail3_domain_report::{CheckResult, Severity};

use super::facts::RepoReleaseFacts;
use super::inputs::PublishableCrateReleaseInput;
use super::release_support::binary_release_present;

const ID: &str = "RS-BIN-01";

pub fn check(
    input: &PublishableCrateReleaseInput<'_>,
    repos: &[RepoReleaseFacts],
    results: &mut Vec<CheckResult>,
) {
    let krate = input.krate;
    if !krate.publishable || !krate.is_binary {
        return;
    }
    let workflow = repos.iter().find_map(|repo| {
        repo.workflows.iter().find(|workflow| {
            binary_release_present(
                &workflow.analysis,
                &krate.name,
                &krate.cargo_rel_path,
                &krate.binary_target_names,
                repo.publishable_binary_crate_names.len(),
            )
        })
    });
    match workflow {
        Some(workflow) => results.push(
            CheckResult {
                id: ID.to_owned(),
                severity: Severity::Info,
                title: format!("{}: binary release workflow present", krate.name),
                message: format!(
                    "Workflow `{}` builds release binaries and uses a GitHub release action.",
                    workflow.rel_path
                ),
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
                title: format!("{}: no binary release workflow", krate.name),
                message:
                    "No workflow builds a release binary and publishes it via GitHub Releases."
                        .to_owned(),
                file: Some(krate.cargo_rel_path.clone()),
                line: None,
                inventory: false,
            }
            .as_inventory(),
        ),
    }
}

#[cfg(test)]
#[path = "rs_bin_01_binary_release_workflow_tests/mod.rs"]
mod tests;
