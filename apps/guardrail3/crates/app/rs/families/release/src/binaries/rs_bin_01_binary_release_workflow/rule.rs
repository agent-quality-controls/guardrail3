use guardrail3_domain_report::{CheckResult, Severity};

use crate::facts::RepoReleaseFacts;
use crate::inputs::PublishableCrateReleaseInput;
use crate::release_support::binary_release_present;

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
            CheckResult::from_parts(
                ID.to_owned(),
                Severity::Info,
                format!("{}: binary release workflow present", krate.name),
                format!(
                    "Workflow `{}` builds release binaries and uses a GitHub release action.",
                    workflow.rel_path
                ),
                Some(workflow.rel_path.clone()),
                None,
                false,
            )
            .as_inventory(),
        ),
        None => results.push(
            CheckResult::from_parts(
                ID.to_owned(),
                Severity::Info,
                format!("{}: no binary release workflow", krate.name),
                "No workflow builds a release binary and publishes it via GitHub Releases."
                    .to_owned(),
                Some(krate.cargo_rel_path.clone()),
                None,
                false,
            )
            .as_inventory(),
        ),
    }
}

