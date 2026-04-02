use guardrail3_domain_report::{CheckResult, Severity};

use crate::facts::RepoReleaseFacts;
use crate::inputs::PublishableCrateReleaseInput;
use crate::release_support::linux_target_present;

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
    let workflow = repos.iter().find_map(|repo| {
        repo.workflows.iter().find(|workflow| {
            linux_target_present(
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
                format!("{}: linux release target present", krate.name),
                format!("Workflow `{}` includes a Linux target.", workflow.rel_path),
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
                format!("{}: no linux release target", krate.name),
                "No workflow includes a Linux target for binary release.".to_owned(),
                Some(krate.cargo_rel_path.clone()),
                None,
                false,
            )
            .as_inventory(),
        ),
    }
}

