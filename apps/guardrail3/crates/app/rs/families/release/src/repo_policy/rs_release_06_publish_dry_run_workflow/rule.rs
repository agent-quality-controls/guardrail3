use guardrail3_domain_report::{CheckResult, Severity};

use crate::inputs::RepoReleaseInput;
use crate::release_support::publish_dry_run_step_present;

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

