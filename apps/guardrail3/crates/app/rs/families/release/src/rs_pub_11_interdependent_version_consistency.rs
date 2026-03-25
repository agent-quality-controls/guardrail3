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
#[path = "rs_pub_11_interdependent_version_consistency_tests/mod.rs"]
mod tests;

fn dependency_package_suffix(edge: &super::facts::ReleaseEdgeFacts) -> String {
    (edge.dep_name != edge.dep_package_name)
        .then(|| format!(" (package `{}`)", edge.dep_package_name))
        .unwrap_or_default()
}
