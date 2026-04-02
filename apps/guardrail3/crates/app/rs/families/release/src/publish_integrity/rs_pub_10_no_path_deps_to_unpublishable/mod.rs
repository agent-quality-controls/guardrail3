use guardrail3_domain_report::{CheckResult, Severity};

use crate::inputs::ReleaseEdgeInput;

const ID: &str = "RS-PUB-10";

pub fn check(input: &ReleaseEdgeInput<'_>, results: &mut Vec<CheckResult>) {
    let edge = input.edge;
    if !edge.has_path || edge.dep_publishable {
        return;
    }
    results.push(CheckResult::from_parts(
        ID.to_owned(),
        Severity::Error,
        format!("{}: path dep to non-publishable crate", edge.crate_name),
        format!(
            "Dependency `{}`{} in `[{}]`{} points at a non-publishable local crate.",
            edge.dep_name,
            dependency_package_suffix(edge),
            edge.section_label,
            edge.target_label
                .as_ref()
                .map(|target| format!(" under target `{target}`"))
                .unwrap_or_default()
        ),
        Some(edge.cargo_rel_path.clone()),
        None,
        false,
    ));
}

#[cfg(test)]
pub(crate) fn run_tree(
    tree: &guardrail3_app_rs_family_view::FamilyView,
    tc: &dyn guardrail3_outbound_traits::ToolChecker,
    thorough: bool,
) -> Vec<guardrail3_domain_report::CheckResult> {
    crate::test_fixtures::run_tree(tree, tc, thorough)
}

#[cfg(test)]
pub(crate) fn run_tree_with_validation_scope(
    tree: &guardrail3_app_rs_family_view::FamilyView,
    tc: &dyn guardrail3_outbound_traits::ToolChecker,
    thorough: bool,
    validation_scope: &str,
) -> Vec<guardrail3_domain_report::CheckResult> {
    crate::test_fixtures::run_tree_with_validation_scope(tree, tc, thorough, validation_scope)
}

#[cfg(test)]
pub(crate) fn edge_facts() -> crate::facts::ReleaseEdgeFacts {
    crate::test_fixtures::edge_facts()
}

#[cfg(test)]
pub(crate) fn edge_input(
    edge: &crate::facts::ReleaseEdgeFacts,
) -> crate::inputs::ReleaseEdgeInput<'_> {
    crate::test_fixtures::edge_input(edge)
}
#[cfg(test)]
pub(crate) fn dependency_edges(
    parsed: &toml::Value,
    workspace_dependencies: &toml::map::Map<String, toml::Value>,
) -> Vec<crate::release_support::dependencies::DependencyEdgeFacts> {
    crate::release_support::dependencies::dependency_edges(parsed, workspace_dependencies)
}

#[cfg(test)]
pub(super) use test_support::{StubToolChecker, dir_entry, project_tree, temp_root};

#[cfg(test)]

mod tests;

fn dependency_package_suffix(edge: &crate::facts::ReleaseEdgeFacts) -> String {
    (edge.dep_name != edge.dep_package_name)
        .then(|| format!(" (package `{}`)", edge.dep_package_name))
        .unwrap_or_default()
}
