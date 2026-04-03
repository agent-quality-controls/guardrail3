use guardrail3_app_rs_family_mapper::RsTopologyRoute;
use guardrail3_app_rs_family_view::FamilyView;
use guardrail3_domain_report::CheckResult;

pub fn check(surface: &FamilyView, route: &RsTopologyRoute) -> Vec<CheckResult> {
    let tree = surface;
    let facts = crate::facts::collect(tree, route);
    let mut results = Vec::new();

    for input in crate::inputs::RootClassificationInput::from_facts(&facts) {
        crate::rs_topology_01_root_classification::check(&input, &mut results);
    }

    for input in crate::inputs::MisplacedRootInput::from_facts(&facts) {
        crate::rs_topology_02_no_misplaced_roots::check(&input, &mut results);
    }
    crate::rs_topology_02_no_misplaced_roots::check_success(
        facts.misplaced_root_reporting_enabled,
        facts.roots.iter().any(|root| {
            root.classification == guardrail3_app_rs_placement::RustRootClassification::Other
        }),
        &mut results,
    );

    for input in crate::inputs::AuxiliaryRootInput::from_facts(&facts) {
        crate::rs_topology_08_auxiliary_roots_declared::check(&input, &mut results);
    }

    for input in crate::inputs::DualOwnershipInput::from_facts(&facts) {
        crate::rs_topology_03_no_dual_ownership::check(&input, &mut results);
    }

    for input in crate::inputs::ZoneOverlapInput::from_facts(&facts) {
        crate::rs_topology_04_no_zone_overlap::check(&input, &mut results);
    }
    crate::rs_topology_04_no_zone_overlap::check_success(!facts.overlaps.is_empty(), &mut results);

    for input in crate::inputs::ScopedTopologyConfigInput::from_facts(&facts) {
        crate::rs_topology_05_scoped_topology_config_forbidden::check(&input, &mut results);
    }
    crate::rs_topology_05_scoped_topology_config_forbidden::check_success(
        facts.input_failures.iter().any(|failure| {
            matches!(
                failure.kind,
                crate::facts::TopologyInputFailureKind::ScopedTopologyConfig
            )
        }),
        facts.input_failures.iter().any(|failure| {
            matches!(
                failure.kind,
                crate::facts::TopologyInputFailureKind::RequiredInput
            ) && failure.rel_path == "guardrail3.toml"
        }),
        &mut results,
    );

    for input in crate::inputs::OwnerFamilyCoherenceInput::from_facts(&facts) {
        crate::rs_topology_06_owner_family_enablement_coherence::check(&input, &mut results);
    }

    for input in crate::inputs::RequiredInputFailureInput::from_facts(&facts) {
        crate::rs_topology_07_required_inputs_fail_closed::check(&input, &mut results);
    }
    crate::rs_topology_07_required_inputs_fail_closed::check_success(
        facts.input_failures.iter().any(|failure| {
            matches!(
                failure.kind,
                crate::facts::TopologyInputFailureKind::RequiredInput
            )
        }),
        &mut results,
    );

    for input in crate::inputs::TopologyIssueInput::from_facts(&facts) {
        crate::rs_topology_09_top_level_root_workspace::check(&input, &mut results);
        crate::rs_topology_10_no_loose_top_level_packages::check(&input, &mut results);
        crate::rs_topology_11_no_nested_workspaces::check(&input, &mut results);
        crate::rs_topology_12_declared_workspace_members_only::check(&input, &mut results);
        crate::rs_topology_13_member_paths_must_not_escape_root::check(&input, &mut results);
        crate::rs_topology_14_auxiliary_root_workspace::check(&input, &mut results);
    }

    for input in crate::inputs::IllegalFamilyFilePlacementInput::from_facts(&facts) {
        crate::rs_topology_16_workspace_local_file_placement::check(&input, &mut results);
    }

    results
}
