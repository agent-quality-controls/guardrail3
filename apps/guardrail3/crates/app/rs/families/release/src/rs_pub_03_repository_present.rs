use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::PublishableCrateReleaseInput;

const ID: &str = "RS-PUB-03";

pub fn check(input: &PublishableCrateReleaseInput<'_>, results: &mut Vec<CheckResult>) {
    if !input.krate.publishable {
        return;
    }
    results.push(if input.krate.repository_present {
        CheckResult {
            id: ID.to_owned(),
            severity: Severity::Info,
            title: format!("{}: repository present", input.krate.name),
            message: "Cargo.toml sets `[package].repository`.".to_owned(),
            file: Some(input.krate.cargo_rel_path.clone()),
            line: None,
            inventory: false,
        }
        .as_inventory()
    } else {
        CheckResult {
            id: ID.to_owned(),
            severity: Severity::Error,
            title: format!("{}: missing repository", input.krate.name),
            message: "Publishable crates must set `[package].repository`.".to_owned(),
            file: Some(input.krate.cargo_rel_path.clone()),
            line: None,
            inventory: false,
        }
    });
}


#[cfg(test)]
#[allow(dead_code)]
pub(super) fn run_tree(
    tree: &guardrail3_domain_project_tree::ProjectTree,
    tc: &dyn guardrail3_outbound_traits::ToolChecker,
    thorough: bool,
) -> Vec<guardrail3_domain_report::CheckResult> {
    crate::test_fixtures::run_tree(tree, tc, thorough)
}

#[cfg(test)]
#[allow(dead_code)]
pub(super) fn run_family(
    root: &std::path::Path,
    thorough: bool,
) -> Vec<guardrail3_domain_report::CheckResult> {
    crate::test_fixtures::run_family(root, thorough)
}

#[cfg(test)]
#[allow(dead_code)]
pub(super) fn copy_fixture() -> tempfile::TempDir {
    crate::test_fixtures::copy_fixture()
}

#[cfg(test)]
#[allow(dead_code)]
pub(super) fn errors_by_id<'a>(
    results: &'a [guardrail3_domain_report::CheckResult],
    id: &str,
) -> Vec<&'a guardrail3_domain_report::CheckResult> {
    results
        .iter()
        .filter(|result| result.id == id)
        .collect()
}

#[cfg(test)]
#[allow(dead_code)]
pub(super) fn crate_facts(name: &str) -> crate::facts::PublishableCrateFacts {
    crate::test_fixtures::crate_facts(name)
}

#[cfg(test)]
#[allow(dead_code)]
pub(super) fn crate_input(
    krate: &crate::facts::PublishableCrateFacts,
) -> crate::inputs::PublishableCrateReleaseInput<'_> {
    crate::test_fixtures::crate_input(krate)
}

#[cfg(test)]
#[allow(dead_code)]
pub(super) fn repo_facts() -> crate::facts::RepoReleaseFacts {
    crate::test_fixtures::repo_facts()
}

#[cfg(test)]
#[allow(dead_code)]
pub(super) fn repo_input(
    repo: &crate::facts::RepoReleaseFacts,
) -> crate::inputs::RepoReleaseInput<'_> {
    crate::test_fixtures::repo_input(repo)
}

#[cfg(test)]
#[allow(dead_code)]
pub(super) fn edge_facts() -> crate::facts::ReleaseEdgeFacts {
    crate::test_fixtures::edge_facts()
}

#[cfg(test)]
#[allow(dead_code)]
pub(super) fn edge_input(
    edge: &crate::facts::ReleaseEdgeFacts,
) -> crate::inputs::ReleaseEdgeInput<'_> {
    crate::test_fixtures::edge_input(edge)
}

#[cfg(test)]
#[allow(dead_code)]
pub(super) fn workflow_from_yaml(rel_path: &str, yaml: &str) -> crate::facts::WorkflowFacts {
    crate::test_fixtures::workflow_from_yaml(rel_path, yaml)
}

#[cfg(test)]
#[allow(dead_code)]
pub(super) fn dependency_edges(
    parsed: &toml::Value,
    workspace_dependencies: &toml::map::Map<String, toml::Value>,
) -> Vec<crate::release_support::DependencyEdgeFacts> {
    crate::release_support::dependency_edges(parsed, workspace_dependencies)
}

#[cfg(test)]
#[allow(unused_imports)]
pub(super) use test_support::{StubToolChecker, dir_entry, project_tree, temp_root, write_file};

#[cfg(test)]
#[path = "rs_pub_03_repository_present_tests/mod.rs"]
mod rs_pub_03_repository_present_tests;
