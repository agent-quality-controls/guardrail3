pub fn check(
    surface: &guardrail3_app_rs_family_view::FamilyView,
    route: &guardrail3_app_rs_family_mapper::RsHexarchRoute,
) -> Vec<guardrail3_domain_report::CheckResult> {
    let tree = surface;
    let facts = crate::facts::collect(tree, route);
    let dependency_facts = crate::dependency_facts::collect(tree, route);
    let source_facts = crate::source_facts::collect(tree, &dependency_facts.members);
    let mut results = Vec::new();

    for app in &facts.apps {
        let input = crate::inputs::AppHexarchInput::new(app);
        crate::structure::rs_hexarch_01_crates_exists::check(&input, &mut results);
        crate::workspace_policy::rs_hexarch_08_app_cargo_is_workspace::check(&input, &mut results);
        crate::workspace_policy::rs_hexarch_12_src_banned::check(&input, &mut results);
    }

    for root in &facts.hex_roots {
        let input = crate::inputs::HexRootInput::new(root);
        crate::structure::rs_hexarch_02_exact_contents::check(&input, &mut results);
    }

    for dir in &facts.directional_containers {
        let input = crate::inputs::DirectionalContainerHexarchInput::new(dir);
        crate::structure::rs_hexarch_03_inbound_outbound::check(&input, &mut results);
    }

    for container in &facts.containers {
        let input = crate::inputs::ContainerHexarchInput::new(container);
        crate::structure::rs_hexarch_04_loose_files::check(&input, &mut results);
        crate::structure::rs_hexarch_05_container_not_empty::check(&input, &mut results);
    }

    for leaf in &facts.leaves {
        let input = crate::inputs::LeafHexarchInput::new(leaf);
        crate::structure::rs_hexarch_06_leaf_valid::check(&input, &mut results);
    }

    for app in &facts.workspace_coverage {
        let input = crate::inputs::WorkspaceCoverageHexarchInput::new(app);
        crate::workspace_policy::rs_hexarch_10_members_within_app_boundary::check(&input, &mut results);
        crate::dependency_integrity::rs_hexarch_27_nested_workspace_forbidden::check(&input, &mut results);
    }

    let root_workspace = crate::inputs::RootWorkspaceHexarchInput::new(&facts.root_workspace);
    crate::workspace_policy::rs_hexarch_11_root_workspace_doesnt_include_apps::check(&root_workspace, &mut results);

    for boundary in &dependency_facts.boundary_configs {
        let input = crate::inputs::MemberConfigHexarchInput::new(boundary);
        crate::dependency_policy::rs_hexarch_15_boundary_config::check(&input, &mut results);
    }

    for patch in dependency_facts
        .workspaces
        .iter()
        .flat_map(|workspace| workspace.patch_entries.iter())
    {
        let input = crate::inputs::PatchHexarchInput::new(patch);
        crate::dependency_policy::rs_hexarch_16_patch_replace_bypass::check(&input, &mut results);
    }
    crate::dependency_policy::rs_hexarch_16_patch_replace_bypass::check_count(
        dependency_facts
            .workspaces
            .iter()
            .flat_map(|workspace| workspace.patch_entries.iter()),
        &mut results,
    );

    for edge in &dependency_facts.edges {
        let input = crate::inputs::DependencyEdgeHexarchInput::new(edge);
        crate::dependency_policy::rs_hexarch_13_dependency_direction::check(&input, &mut results);
        crate::dependency_policy::rs_hexarch_14_dependency_inventory::check(&input, &mut results);
        crate::dependency_policy::rs_hexarch_17_workspace_inherited_direction::check(&input, &mut results);
        crate::dependency_policy::rs_hexarch_18_renamed_dependency_direction::check(&input, &mut results);
        crate::dependency_integrity::rs_hexarch_20_dev_dependency_direction::check(&input, &mut results);
        crate::dependency_integrity::rs_hexarch_24_cross_app_boundary::check(&input, &mut results);
        crate::dependency_integrity::rs_hexarch_25_target_dependency_direction::check(&input, &mut results);
    }

    for failure in &dependency_facts.member_manifest_failures {
        let input = crate::inputs::MemberManifestFailureHexarchInput::new(failure);
        crate::dependency_integrity::rs_hexarch_26_member_manifest_parse_error::check(&input, &mut results);
    }

    for cycle in &dependency_facts.cycles {
        let input = crate::inputs::CycleHexarchInput::new(cycle);
        crate::dependency_integrity::rs_hexarch_19_same_layer_cycles::check(&input, &mut results);
    }
    crate::dependency_integrity::rs_hexarch_19_same_layer_cycles::check_inventory(
        &dependency_facts.members,
        &dependency_facts.cycles,
        &mut results,
    );

    for member in &dependency_facts.members {
        let input = crate::inputs::MemberDependencyHexarchInput::new(
            member,
            dependency_facts
                .edges
                .iter()
                .filter(|edge| edge.source_rel_dir == member.rel_dir)
                .collect(),
        );
        crate::dependency_integrity::rs_hexarch_21_domain_purity::check(&input, &mut results);
    }

    for source in &source_facts {
        let input = crate::inputs::SourceCrateHexarchInput::new(source);
        crate::dependency_integrity::rs_hexarch_22_ports_trait_dominance::check(&input, &mut results);
        crate::dependency_integrity::rs_hexarch_23_adapter_pub_trait::check(&input, &mut results);
    }

    crate::dependency_integrity::rs_hexarch_26_member_manifest_parse_error::check_inventory(
        &dependency_facts.members,
        &dependency_facts.member_manifest_failures,
        &mut results,
    );

    results
}
