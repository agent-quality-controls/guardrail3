use crate::domain::report::{CheckResult, Severity};

use super::inputs::ReleaseEdgeInput;

const ID: &str = "RS-PUB-10";

pub fn check(input: &ReleaseEdgeInput<'_>, results: &mut Vec<CheckResult>) {
    let edge = input.edge;
    if !edge.has_path || edge.dep_publishable {
        return;
    }
    results.push(CheckResult {
        id: ID.to_owned(),
        severity: Severity::Error,
        title: format!("{}: path dep to non-publishable crate", edge.crate_name),
        message: format!(
            "Dependency `{}` in `[{}]`{} points at a non-publishable local crate.",
            edge.dep_name,
            edge.section_label,
            edge.target_label
                .as_ref()
                .map(|target| format!(" under target `{target}`"))
                .unwrap_or_default()
        ),
        file: Some(edge.cargo_rel_path.clone()),
        line: None,
        inventory: false,
    });
}

#[cfg(test)]
#[path = "rs_pub_10_no_path_deps_to_unpublishable_tests/mod.rs"]
mod tests;
