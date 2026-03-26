mod dependency_facts;
mod facts;
mod inputs;
mod rs_hexarch_01_crates_exists;
mod rs_hexarch_02_exact_contents;
mod rs_hexarch_03_inbound_outbound;
mod rs_hexarch_04_loose_files;
mod rs_hexarch_05_container_not_empty;
mod rs_hexarch_06_leaf_valid;
mod rs_hexarch_07_workspace_members_match_crate_dirs;
mod rs_hexarch_08_app_cargo_is_workspace;
mod rs_hexarch_09_no_extra_workspace_members;
mod rs_hexarch_10_members_within_app_boundary;
mod rs_hexarch_11_root_workspace_doesnt_include_apps;
mod rs_hexarch_12_src_banned;
mod rs_hexarch_13_dependency_direction;
mod rs_hexarch_14_dependency_inventory;
mod rs_hexarch_15_boundary_config;
mod rs_hexarch_16_patch_replace_bypass;
mod rs_hexarch_17_workspace_inherited_direction;
mod rs_hexarch_18_renamed_dependency_direction;
mod rs_hexarch_19_same_layer_cycles;
mod rs_hexarch_20_dev_dependency_direction;
mod rs_hexarch_21_domain_purity;
mod rs_hexarch_22_ports_trait_dominance;
mod rs_hexarch_23_adapter_pub_trait;
mod rs_hexarch_24_cross_app_boundary;
mod rs_hexarch_25_target_dependency_direction;
mod rs_hexarch_26_member_manifest_parse_error;
mod source_facts;

use std::collections::BTreeSet;

use glob as _;
use guardrail3_app_core as _;
use guardrail3_app_rs_family_mapper::RsHexarchRoute;
use guardrail3_domain_config::types::GuardrailConfig;
use guardrail3_domain_modules as _;
use guardrail3_domain_project_tree::ProjectTree;
use guardrail3_domain_report::CheckResult;
use guardrail3_outbound_traits as _;
use guardrail3_validation_model::{RustFamilySelection, RustValidateFamily};
use proc_macro2 as _;
use quote as _;
use semver as _;
use serde_yaml as _;

use self::facts::collect;
use self::inputs::{
    AppHexarchInput, ContainerHexarchInput, CycleHexarchInput, DependencyEdgeHexarchInput,
    DirectionalContainerHexarchInput, HexRootInput, LeafHexarchInput, MemberConfigHexarchInput,
    MemberDependencyHexarchInput, MemberManifestFailureHexarchInput, PatchHexarchInput, RootWorkspaceHexarchInput,
    SourceCrateHexarchInput, WorkspaceCoverageHexarchInput,
};

#[doc(hidden)]
pub use self::dependency_facts::DependencyFamilyFacts;

#[doc(hidden)]
pub fn collect_dependency_facts_for_tests(
    tree: &ProjectTree,
    route: &RsHexarchRoute,
) -> DependencyFamilyFacts {
    dependency_facts::collect(tree, route)
}

pub fn family_route_for_tests(tree: &ProjectTree) -> RsHexarchRoute {
    let scope = guardrail3_app_rs_placement::collect(tree);
    let config = tree
        .file_content("guardrail3.toml")
        .and_then(|content| toml::from_str::<GuardrailConfig>(content).ok());
    let selection = RustFamilySelection::new(BTreeSet::from([RustValidateFamily::Hexarch]));
    guardrail3_app_rs_family_mapper::FamilyMapper::new(
        tree,
        &scope,
        config.as_ref(),
        &selection,
        None,
    )
    .map_rs_hexarch()
}

pub fn check_test_tree(tree: &ProjectTree) -> Vec<CheckResult> {
    let route = family_route_for_tests(tree);
    check(tree, &route)
}

pub fn collect_dependency_facts_from_tree_for_tests(tree: &ProjectTree) -> DependencyFamilyFacts {
    let route = family_route_for_tests(tree);
    collect_dependency_facts_for_tests(tree, &route)
}

pub fn check(tree: &ProjectTree, route: &RsHexarchRoute) -> Vec<CheckResult> {
    let facts = collect(tree, route);
    let dependency_facts = dependency_facts::collect(tree, route);
    let source_facts = source_facts::collect(tree, &dependency_facts.members);
    let mut results = Vec::new();

    for app in &facts.apps {
        let input = AppHexarchInput::new(app);
        rs_hexarch_01_crates_exists::check(&input, &mut results);
        rs_hexarch_08_app_cargo_is_workspace::check(&input, &mut results);
        rs_hexarch_12_src_banned::check(&input, &mut results);
    }

    for root in &facts.hex_roots {
        let input = HexRootInput::new(root);
        rs_hexarch_02_exact_contents::check(&input, &mut results);
    }

    for dir in &facts.directional_containers {
        let input = DirectionalContainerHexarchInput::new(dir);
        rs_hexarch_03_inbound_outbound::check(&input, &mut results);
    }

    for container in &facts.containers {
        let input = ContainerHexarchInput::new(container);
        rs_hexarch_04_loose_files::check(&input, &mut results);
        rs_hexarch_05_container_not_empty::check(&input, &mut results);
    }

    for leaf in &facts.leaves {
        let input = LeafHexarchInput::new(leaf);
        rs_hexarch_06_leaf_valid::check(&input, &mut results);
    }

    for app in &facts.workspace_coverage {
        let input = WorkspaceCoverageHexarchInput::new(app);
        rs_hexarch_07_workspace_members_match_crate_dirs::check(&input, &mut results);
        rs_hexarch_09_no_extra_workspace_members::check(&input, &mut results);
        rs_hexarch_10_members_within_app_boundary::check(&input, &mut results);
    }

    let root_workspace = RootWorkspaceHexarchInput::new(&facts.root_workspace);
    rs_hexarch_11_root_workspace_doesnt_include_apps::check(&root_workspace, &mut results);

    for boundary in &dependency_facts.boundary_configs {
        let input = MemberConfigHexarchInput::new(boundary);
        rs_hexarch_15_boundary_config::check(&input, &mut results);
    }

    for patch in dependency_facts
        .workspaces
        .iter()
        .flat_map(|workspace| workspace.patch_entries.iter())
    {
        let input = PatchHexarchInput::new(patch);
        rs_hexarch_16_patch_replace_bypass::check(&input, &mut results);
    }

    for edge in &dependency_facts.edges {
        let input = DependencyEdgeHexarchInput::new(edge);
        rs_hexarch_13_dependency_direction::check(&input, &mut results);
        rs_hexarch_14_dependency_inventory::check(&input, &mut results);
        rs_hexarch_17_workspace_inherited_direction::check(&input, &mut results);
        rs_hexarch_18_renamed_dependency_direction::check(&input, &mut results);
        rs_hexarch_20_dev_dependency_direction::check(&input, &mut results);
        rs_hexarch_24_cross_app_boundary::check(&input, &mut results);
        rs_hexarch_25_target_dependency_direction::check(&input, &mut results);
    }

    for failure in &dependency_facts.member_manifest_failures {
        let input = MemberManifestFailureHexarchInput::new(failure);
        rs_hexarch_26_member_manifest_parse_error::check(&input, &mut results);
    }

    for cycle in &dependency_facts.cycles {
        let input = CycleHexarchInput::new(cycle);
        rs_hexarch_19_same_layer_cycles::check(&input, &mut results);
    }

    for member in &dependency_facts.members {
        let input = MemberDependencyHexarchInput::new(
            member,
            dependency_facts
                .edges
                .iter()
                .filter(|edge| edge.source_rel_dir == member.rel_dir)
                .collect(),
        );
        rs_hexarch_21_domain_purity::check(&input, &mut results);
    }

    for source in &source_facts {
        let input = SourceCrateHexarchInput::new(source);
        rs_hexarch_22_ports_trait_dominance::check(&input, &mut results);
        rs_hexarch_23_adapter_pub_trait::check(&input, &mut results);
    }

    results
}
