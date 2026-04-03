use guardrail3_domain_report::{CheckResult, Severity};

use crate::inputs::RepoReleaseInput;
use crate::release_support::registry_token_present;

const ID: &str = "RS-RELEASE-07";

pub fn check(input: &RepoReleaseInput<'_>, results: &mut Vec<CheckResult>) {
    let workflow = input
        .repo
        .workflows
        .iter()
        .find(|workflow| registry_token_present(&workflow.analysis));
    match workflow {
        Some(workflow) => results.push(
            CheckResult::from_parts(
                ID.to_owned(),
                Severity::Info,
                "CARGO_REGISTRY_TOKEN wired in workflow".to_owned(),
                format!(
                    "Workflow `{}` structurally references `CARGO_REGISTRY_TOKEN`.",
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
            "CARGO_REGISTRY_TOKEN missing from workflows".to_owned(),
            "No workflow structurally wires `CARGO_REGISTRY_TOKEN` into release steps. Add `CARGO_REGISTRY_TOKEN` as a secret in the release workflow.".to_owned(),
            None,
            None,
            false,
        )),
    }
}

