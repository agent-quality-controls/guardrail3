use guardrail3_domain_report::{CheckResult, Severity};

use crate::inputs::ReleaseEdgeInput;

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
    results.push(CheckResult::from_parts(
    ID.to_owned(),
    Severity::Error,
    format!("{}: version mismatch with {}", edge.crate_name, edge.dep_name),
    format!(
            "Dependency `{}`{} in `[{}]`{} requires `{}` but actual local publishable version is `{}`. Update the version requirement to match the local crate's version.",
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

