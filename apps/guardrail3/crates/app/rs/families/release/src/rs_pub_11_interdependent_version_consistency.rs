use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::ReleaseEdgeInput;

const ID: &str = "RS-PUB-11";

pub fn check(input: &ReleaseEdgeInput<'_>, results: &mut Vec<CheckResult>) {
    let edge = input.edge;
    if !edge.has_path || !edge.dep_publishable {
        return;
    }
    let Some(version_req) = &edge.version_req else {
        return;
    };
    if edge.version_satisfied.unwrap_or(true) {
        return;
    }
    results.push(CheckResult {
        id: ID.to_owned(),
        severity: Severity::Error,
        title: format!("{}: version mismatch with {}", edge.crate_name, edge.dep_name),
        message: format!(
            "Dependency `{}`{} in `[{}]`{} requires `{}` but actual local publishable version is `{}`.",
            edge.dep_name,
            dependency_package_suffix(edge),
            edge.section_label,
            edge.target_label
                .as_ref()
                .map(|target| format!(" under target `{target}`"))
                .unwrap_or_default(),
            version_req,
            edge.actual_version.clone().unwrap_or_else(|| "unknown".to_owned())
        ),
        file: Some(edge.cargo_rel_path.clone()),
        line: None,
        inventory: false,
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
#[path = "rs_pub_11_interdependent_version_consistency_tests/mod.rs"]
mod rs_pub_11_interdependent_version_consistency_tests;

fn dependency_package_suffix(edge: &super::facts::ReleaseEdgeFacts) -> String {
    (edge.dep_name != edge.dep_package_name)
        .then(|| format!(" (package `{}`)", edge.dep_package_name))
        .unwrap_or_default()
}
