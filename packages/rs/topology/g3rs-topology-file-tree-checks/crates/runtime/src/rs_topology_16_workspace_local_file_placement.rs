use guardrail3_check_types::{G3CheckResult, G3Severity};

use crate::support::{IllegalFamilyFilePlacement, family_label};

const ID: &str = "RS-TOPOLOGY-FILETREE-16";

pub(crate) fn check(input: &IllegalFamilyFilePlacement, results: &mut Vec<G3CheckResult>) {
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
#[path = "rs_topology_16_workspace_local_file_placement_tests/mod.rs"]
mod tests;
