mod facts;
mod inputs;
mod rs_topology_01_root_classification;
mod rs_topology_02_no_misplaced_roots;
mod rs_topology_03_no_dual_ownership;
mod rs_topology_04_no_zone_overlap;
mod rs_topology_05_scoped_topology_config_forbidden;
mod rs_topology_06_owner_family_enablement_coherence;
mod rs_topology_07_required_inputs_fail_closed;
mod rs_topology_08_auxiliary_roots_declared;
mod rs_topology_09_top_level_root_workspace;
mod rs_topology_10_no_loose_top_level_packages;
mod rs_topology_11_no_nested_workspaces;
mod rs_topology_12_declared_workspace_members_only;
mod rs_topology_13_member_paths_must_not_escape_root;
mod rs_topology_14_auxiliary_root_workspace;
mod rs_topology_16_workspace_local_file_placement;

mod run;
pub use run::check;

#[cfg(test)]
use guardrail3_app_rs_family_view::FamilyView as ProjectTree;
#[cfg(test)]
use guardrail3_app_rs_family_mapper::FamilyMapper;
#[cfg(test)]
use guardrail3_domain_config::types::GuardrailConfig;
#[cfg(test)]
use guardrail3_validation_model::{RustFamilySelection, RustValidateFamily};
#[cfg(test)]
use std::collections::BTreeSet;
#[cfg(test)]
use guardrail3_domain_report::CheckResult;

#[cfg(test)]
pub fn check_test_tree(tree: &ProjectTree) -> Vec<CheckResult> {
    let pt = guardrail3_domain_project_tree::ProjectTree::new(tree.root_path().to_path_buf(), tree.structure().clone(), tree.content().clone());
    let structure = guardrail3_app_rs_structure::collect(pt, &[]);
    let legality = guardrail3_app_rs_legality::collect(structure);
    let config = tree
        .file_content("guardrail3.toml")
        .and_then(|content| toml::from_str::<GuardrailConfig>(content).ok());
    let selection = RustFamilySelection::new(BTreeSet::from([RustValidateFamily::Topology]));
    let route = FamilyMapper::from_legality(&legality, config.as_ref(), &selection, None).map_rs_topology();
    check(tree, &route)
}
