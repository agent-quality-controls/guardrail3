use guardrail3_check_types::{G3CheckResult, G3Severity};

use crate::support::family_label;

const ID: &str = "RS-TOPOLOGY-FILETREE-16";

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct IllegalFamilyFilePlacementInput {
    pub(crate) family: g3rs_topology_types::G3RsTopologyWorkspaceFamily,
    pub(crate) rel_path: String,
    pub(crate) reason: String,
}

pub(crate) fn check(input: &IllegalFamilyFilePlacementInput, results: &mut Vec<G3CheckResult>) {
    results.push(G3CheckResult::new(
        ID.to_owned(),
        G3Severity::Error,
        format!(
            "`{}` file `{}` is illegally placed",
            family_label(input.family),
            input.rel_path
        ),
        input.reason.clone(),
        Some(input.rel_path.clone()),
        None,
    ));
}

#[cfg(test)]
#[path = "rule_tests/mod.rs"] // reason: owned sidecar tests for file module.
mod rule_tests;
