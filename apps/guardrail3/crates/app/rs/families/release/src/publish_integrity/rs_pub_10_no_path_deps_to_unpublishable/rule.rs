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
            "Dependency `{}`{} in `[{}]`{} points at a non-publishable local crate. Either make the target crate publishable or replace the path dependency with a version requirement.",
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







fn dependency_package_suffix(edge: &crate::facts::ReleaseEdgeFacts) -> String {
    (edge.dep_name != edge.dep_package_name)
        .then(|| format!(" (package `{}`)", edge.dep_package_name))
        .unwrap_or_default()
}

