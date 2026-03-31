mod dependency_facts;
mod facts;
mod inputs;
mod inventory;
#[path = "structure/rs_hexarch_01_crates_exists.rs"]
mod rs_hexarch_01_crates_exists;
#[path = "structure/rs_hexarch_02_exact_contents.rs"]
mod rs_hexarch_02_exact_contents;
#[path = "structure/rs_hexarch_03_inbound_outbound.rs"]
mod rs_hexarch_03_inbound_outbound;
#[path = "structure/rs_hexarch_04_loose_files.rs"]
mod rs_hexarch_04_loose_files;
#[path = "structure/rs_hexarch_05_container_not_empty.rs"]
mod rs_hexarch_05_container_not_empty;
#[path = "structure/rs_hexarch_06_leaf_valid.rs"]
mod rs_hexarch_06_leaf_valid;
#[path = "workspace_policy/rs_hexarch_08_app_cargo_is_workspace.rs"]
mod rs_hexarch_08_app_cargo_is_workspace;
#[path = "workspace_policy/rs_hexarch_10_members_within_app_boundary.rs"]
mod rs_hexarch_10_members_within_app_boundary;
#[path = "workspace_policy/rs_hexarch_11_root_workspace_doesnt_include_apps.rs"]
mod rs_hexarch_11_root_workspace_doesnt_include_apps;
#[path = "workspace_policy/rs_hexarch_12_src_banned.rs"]
mod rs_hexarch_12_src_banned;
#[path = "dependency_policy/rs_hexarch_13_dependency_direction.rs"]
mod rs_hexarch_13_dependency_direction;
#[path = "dependency_policy/rs_hexarch_14_dependency_inventory.rs"]
mod rs_hexarch_14_dependency_inventory;
#[path = "dependency_policy/rs_hexarch_15_boundary_config.rs"]
mod rs_hexarch_15_boundary_config;
#[path = "dependency_policy/rs_hexarch_16_patch_replace_bypass.rs"]
mod rs_hexarch_16_patch_replace_bypass;
#[path = "dependency_policy/rs_hexarch_17_workspace_inherited_direction.rs"]
mod rs_hexarch_17_workspace_inherited_direction;
#[path = "dependency_policy/rs_hexarch_18_renamed_dependency_direction.rs"]
mod rs_hexarch_18_renamed_dependency_direction;
#[path = "dependency_integrity/rs_hexarch_19_same_layer_cycles.rs"]
mod rs_hexarch_19_same_layer_cycles;
#[path = "dependency_integrity/rs_hexarch_20_dev_dependency_direction.rs"]
mod rs_hexarch_20_dev_dependency_direction;
#[path = "dependency_integrity/rs_hexarch_21_domain_purity.rs"]
mod rs_hexarch_21_domain_purity;
#[path = "dependency_integrity/rs_hexarch_22_ports_trait_dominance.rs"]
mod rs_hexarch_22_ports_trait_dominance;
#[path = "dependency_integrity/rs_hexarch_23_adapter_pub_trait.rs"]
mod rs_hexarch_23_adapter_pub_trait;
#[path = "dependency_integrity/rs_hexarch_24_cross_app_boundary.rs"]
mod rs_hexarch_24_cross_app_boundary;
#[path = "dependency_integrity/rs_hexarch_25_target_dependency_direction.rs"]
mod rs_hexarch_25_target_dependency_direction;
#[path = "dependency_integrity/rs_hexarch_26_member_manifest_parse_error.rs"]
mod rs_hexarch_26_member_manifest_parse_error;
#[path = "dependency_integrity/rs_hexarch_27_nested_workspace_forbidden.rs"]
mod rs_hexarch_27_nested_workspace_forbidden;
mod source_facts;

extern crate glob as _;
extern crate guardrail3_domain_modules as _;
extern crate guardrail3_outbound_traits as _;
extern crate proc_macro2 as _;
extern crate quote as _;
extern crate semver as _;
extern crate serde_yaml as _;

#[cfg(test)]
use std::collections::BTreeSet;

#[doc(hidden)]
pub use self::dependency_facts::DependencyFamilyFacts;

#[doc(hidden)]
#[cfg(test)]
pub fn collect_dependency_facts_for_tests(
    tree: &guardrail3_app_rs_family_mapper::RsProjectSurface,
    route: &guardrail3_app_rs_family_mapper::RsHexarchRoute,
) -> DependencyFamilyFacts {
    dependency_facts::collect(tree, route)
}

#[cfg(test)]
pub fn family_route_for_tests(
    tree: &guardrail3_app_rs_family_mapper::RsProjectSurface,
) -> guardrail3_app_rs_family_mapper::RsHexarchRoute {
    let scope = guardrail3_app_rs_structure::collect(tree);
    let config = tree.file_content("guardrail3.toml").and_then(|content| {
        toml::from_str::<guardrail3_domain_config::types::GuardrailConfig>(content).ok()
    });
    let selection = guardrail3_validation_model::RustFamilySelection::new(BTreeSet::from([
        guardrail3_validation_model::RustValidateFamily::Hexarch,
    ]));
    guardrail3_app_rs_family_mapper::FamilyMapper::new(
        tree,
        &scope,
        config.as_ref(),
        &selection,
        None,
    )
    .map_rs_hexarch()
}

#[cfg(test)]
pub fn check_test_tree(
    tree: &guardrail3_app_rs_family_mapper::RsProjectSurface,
) -> Vec<guardrail3_domain_report::CheckResult> {
    let route = family_route_for_tests(tree);
    check(
        &guardrail3_app_rs_family_mapper::RsProjectSurface::from_tree(tree),
        &route,
    )
}

#[cfg(test)]
pub fn collect_dependency_facts_from_tree_for_tests(
    tree: &guardrail3_app_rs_family_mapper::RsProjectSurface,
) -> DependencyFamilyFacts {
    let route = family_route_for_tests(tree);
    collect_dependency_facts_for_tests(tree, &route)
}

pub fn check(
    surface: &guardrail3_app_rs_family_mapper::RsProjectSurface,
    route: &guardrail3_app_rs_family_mapper::RsHexarchRoute,
) -> Vec<guardrail3_domain_report::CheckResult> {
    let tree = surface;
    let facts = facts::collect(tree, route);
    let dependency_facts = dependency_facts::collect(tree, route);
    let source_facts = source_facts::collect(tree, &dependency_facts.members);
    let mut results = Vec::new();

    for app in &facts.apps {
        let input = inputs::AppHexarchInput::new(app);
        rs_hexarch_01_crates_exists::check(&input, &mut results);
        rs_hexarch_08_app_cargo_is_workspace::check(&input, &mut results);
        rs_hexarch_12_src_banned::check(&input, &mut results);
    }

    for root in &facts.hex_roots {
        let input = inputs::HexRootInput::new(root);
        rs_hexarch_02_exact_contents::check(&input, &mut results);
    }

    for dir in &facts.directional_containers {
        let input = inputs::DirectionalContainerHexarchInput::new(dir);
        rs_hexarch_03_inbound_outbound::check(&input, &mut results);
    }

    for container in &facts.containers {
        let input = inputs::ContainerHexarchInput::new(container);
        rs_hexarch_04_loose_files::check(&input, &mut results);
        rs_hexarch_05_container_not_empty::check(&input, &mut results);
    }

    for leaf in &facts.leaves {
        let input = inputs::LeafHexarchInput::new(leaf);
        rs_hexarch_06_leaf_valid::check(&input, &mut results);
    }

    for app in &facts.workspace_coverage {
        let input = inputs::WorkspaceCoverageHexarchInput::new(app);
        rs_hexarch_10_members_within_app_boundary::check(&input, &mut results);
        rs_hexarch_27_nested_workspace_forbidden::check(&input, &mut results);
    }

    let root_workspace = inputs::RootWorkspaceHexarchInput::new(&facts.root_workspace);
    rs_hexarch_11_root_workspace_doesnt_include_apps::check(&root_workspace, &mut results);

    for boundary in &dependency_facts.boundary_configs {
        let input = inputs::MemberConfigHexarchInput::new(boundary);
        rs_hexarch_15_boundary_config::check(&input, &mut results);
    }

    for patch in dependency_facts
        .workspaces
        .iter()
        .flat_map(|workspace| workspace.patch_entries.iter())
    {
        let input = inputs::PatchHexarchInput::new(patch);
        rs_hexarch_16_patch_replace_bypass::check(&input, &mut results);
    }
    rs_hexarch_16_patch_replace_bypass::check_count(
        dependency_facts
            .workspaces
            .iter()
            .flat_map(|workspace| workspace.patch_entries.iter()),
        &mut results,
    );

    for edge in &dependency_facts.edges {
        let input = inputs::DependencyEdgeHexarchInput::new(edge);
        rs_hexarch_13_dependency_direction::check(&input, &mut results);
        rs_hexarch_14_dependency_inventory::check(&input, &mut results);
        rs_hexarch_17_workspace_inherited_direction::check(&input, &mut results);
        rs_hexarch_18_renamed_dependency_direction::check(&input, &mut results);
        rs_hexarch_20_dev_dependency_direction::check(&input, &mut results);
        rs_hexarch_24_cross_app_boundary::check(&input, &mut results);
        rs_hexarch_25_target_dependency_direction::check(&input, &mut results);
    }

    for failure in &dependency_facts.member_manifest_failures {
        let input = inputs::MemberManifestFailureHexarchInput::new(failure);
        rs_hexarch_26_member_manifest_parse_error::check(&input, &mut results);
    }

    for cycle in &dependency_facts.cycles {
        let input = inputs::CycleHexarchInput::new(cycle);
        rs_hexarch_19_same_layer_cycles::check(&input, &mut results);
    }
    rs_hexarch_19_same_layer_cycles::check_inventory(
        &dependency_facts.members,
        &dependency_facts.cycles,
        &mut results,
    );

    for member in &dependency_facts.members {
        let input = inputs::MemberDependencyHexarchInput::new(
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
        let input = inputs::SourceCrateHexarchInput::new(source);
        rs_hexarch_22_ports_trait_dominance::check(&input, &mut results);
        rs_hexarch_23_adapter_pub_trait::check(&input, &mut results);
    }

    rs_hexarch_26_member_manifest_parse_error::check_inventory(
        &dependency_facts.members,
        &dependency_facts.member_manifest_failures,
        &mut results,
    );

    results
}

#[cfg(test)]
#[path = "lib_tests/mod.rs"]
mod lib_tests;
