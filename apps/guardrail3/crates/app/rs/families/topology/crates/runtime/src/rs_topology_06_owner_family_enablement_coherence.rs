use guardrail3_app_rs_placement::RustTopologyOwner;
use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::OwnerFamilyCoherenceInput;

const ID: &str = "RS-TOPOLOGY-06";

pub fn check(input: &OwnerFamilyCoherenceInput<'_>, results: &mut Vec<CheckResult>) {
    if !input.root.effective_enabled {
        let family_label = match input.root.owner {
            RustTopologyOwner::Hexarch => "hexarch",
            RustTopologyOwner::Libarch => "libarch",
        };
        results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Error,
            format!(
                "Rust {} root `{}` is not governed by {}",
                input.root.owner.label(),
                display_dir(&input.root.rel_dir),
                family_label
            ),
            format!(
                "`{}` classifies under `{}`, but effective `{}` enablement resolves to false. Governed Rust roots must stay coherent with their owning topology family.",
                input.root.cargo_rel_path, input.root.owner_root_rel, family_label
            ),
            Some(input.root.cargo_rel_path.clone()),
            None,
            false,
        ));
        return;
    }

    let family_label = match input.root.owner {
        RustTopologyOwner::Hexarch => "hexarch",
        RustTopologyOwner::Libarch => "libarch",
    };
    results.push(
        CheckResult::from_parts(
            ID.to_owned(),
            Severity::Info,
            format!(
                "Rust {} root `{}` stays coherent with {}",
                input.root.owner.label(),
                display_dir(&input.root.rel_dir),
                family_label
            ),
            format!(
                "`{}` classifies under `{}`, and effective `{}` enablement resolves to true.",
                input.root.cargo_rel_path, input.root.owner_root_rel, family_label
            ),
            Some(input.root.cargo_rel_path.clone()),
            None,
            false,
        )
        .as_inventory(),
    );
}

fn display_dir(rel_dir: &str) -> &str {
    if rel_dir.is_empty() { "." } else { rel_dir }
}

#[cfg(test)]
pub(crate) fn check_results(
    tree: &guardrail3_app_rs_family_mapper::RsProjectSurface,
) -> Vec<guardrail3_domain_report::CheckResult> {
    crate::check_test_tree(tree)
}

#[cfg(test)]
#[path = "rs_topology_06_owner_family_enablement_coherence_tests/mod.rs"]
mod rs_topology_06_owner_family_enablement_coherence_tests;
