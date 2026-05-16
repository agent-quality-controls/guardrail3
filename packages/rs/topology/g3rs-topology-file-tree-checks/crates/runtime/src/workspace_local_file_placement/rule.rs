use g3rs_topology_types::G3RsTopologyIllegalFamilyFilePlacementInput;
use guardrail3_check_types::{G3CheckResult, G3Severity};

use crate::support::family_label;

/// Stable identifier for this rule.
const ID: &str = "g3rs-topology/workspace-local-file-placement";

/// Runs this rule and appends its findings to `results`.
pub(crate) fn check(
    input: &G3RsTopologyIllegalFamilyFilePlacementInput,
    results: &mut Vec<G3CheckResult>,
) {
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
