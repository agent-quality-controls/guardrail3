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

#[cfg(test)]
pub(super) fn run_tree(
    tree: &guardrail3_app_rs_family_view::FamilyView,
    tc: &dyn guardrail3_outbound_traits::ToolChecker,
    thorough: bool,
) -> Vec<guardrail3_domain_report::CheckResult> {
    crate::test_fixtures::run_tree(tree, tc, thorough)
}

#[cfg(test)]
pub(super) fn run_tree_with_validation_scope(
    tree: &guardrail3_app_rs_family_view::FamilyView,
    tc: &dyn guardrail3_outbound_traits::ToolChecker,
    thorough: bool,
    validation_scope: &str,
) -> Vec<guardrail3_domain_report::CheckResult> {
    crate::test_fixtures::run_tree_with_validation_scope(tree, tc, thorough, validation_scope)
}
#[cfg(test)]
pub(super) fn crate_facts(name: &str) -> crate::facts::PublishableCrateFacts {
    crate::test_fixtures::crate_facts(name)
}

#[cfg(test)]
pub(super) fn crate_input(
    krate: &crate::facts::PublishableCrateFacts,
) -> crate::inputs::PublishableCrateReleaseInput<'_> {
    crate::test_fixtures::crate_input(krate)
}

#[cfg(test)]
pub(super) fn repo_facts() -> crate::facts::RepoReleaseFacts {
    crate::test_fixtures::repo_facts()
}
#[cfg(test)]
pub(super) fn workflow_from_yaml(rel_path: &str, yaml: &str) -> crate::facts::WorkflowFacts {
    crate::test_fixtures::workflow_from_yaml(rel_path, yaml)
}
#[cfg(test)]
pub(super) use test_support::{StubToolChecker, dir_entry, project_tree, temp_root};

#[cfg(test)]

mod rs_bin_01_binary_release_workflow_tests;
