mod facts;
mod inputs;
mod rs_arch_01_root_classification;
mod rs_arch_02_no_misplaced_roots;
mod rs_arch_03_no_dual_ownership;
mod rs_arch_04_no_zone_overlap;
mod rs_arch_05_scoped_arch_config_forbidden;
mod rs_arch_06_owner_family_enablement_coherence;
mod rs_arch_07_required_inputs_fail_closed;
mod rs_arch_08_auxiliary_roots_declared;

use std::collections::BTreeSet;

use guardrail3_app_rs_family_mapper::{FamilyMapper, RsArchRoute};
use guardrail3_domain_config::types::GuardrailConfig;
use guardrail3_domain_project_tree::ProjectTree;
use guardrail3_domain_report::CheckResult;
use guardrail3_validation_model::{RustFamilySelection, RustValidateFamily};

use self::facts::collect;
use self::inputs::{
    AuxiliaryRootInput, DualOwnershipInput, MisplacedRootInput, OwnerFamilyCoherenceInput,
    RequiredInputFailureInput, RootClassificationInput, ScopedArchConfigInput, ZoneOverlapInput,
};

pub fn check(tree: &ProjectTree, route: &RsArchRoute) -> Vec<CheckResult> {
    let facts = collect(tree, route);
    let mut results = Vec::new();

    for input in RootClassificationInput::from_facts(&facts) {
        rs_arch_01_root_classification::check(&input, &mut results);
    }

    for input in MisplacedRootInput::from_facts(&facts) {
        rs_arch_02_no_misplaced_roots::check(&input, &mut results);
    }
    rs_arch_02_no_misplaced_roots::check_success(
        facts.misplaced_root_reporting_enabled,
        facts.roots.iter().any(|root| {
            root.classification == guardrail3_app_rs_placement::RustRootClassification::Other
        }),
        &mut results,
    );

    for input in AuxiliaryRootInput::from_facts(&facts) {
        rs_arch_08_auxiliary_roots_declared::check(&input, &mut results);
    }

    for input in DualOwnershipInput::from_facts(&facts) {
        rs_arch_03_no_dual_ownership::check(&input, &mut results);
    }

    for input in ZoneOverlapInput::from_facts(&facts) {
        rs_arch_04_no_zone_overlap::check(&input, &mut results);
    }
    rs_arch_04_no_zone_overlap::check_success(!facts.overlaps.is_empty(), &mut results);

    for input in ScopedArchConfigInput::from_facts(&facts) {
        rs_arch_05_scoped_arch_config_forbidden::check(&input, &mut results);
    }
    rs_arch_05_scoped_arch_config_forbidden::check_success(
        facts.input_failures.iter().any(|failure| {
            matches!(
                failure.kind,
                self::facts::ArchInputFailureKind::ScopedArchConfig
            )
        }),
        facts.input_failures.iter().any(|failure| {
            matches!(
                failure.kind,
                self::facts::ArchInputFailureKind::RequiredInput
            ) && failure.rel_path == "guardrail3.toml"
        }),
        &mut results,
    );

    for input in OwnerFamilyCoherenceInput::from_facts(&facts) {
        rs_arch_06_owner_family_enablement_coherence::check(&input, &mut results);
    }

    for input in RequiredInputFailureInput::from_facts(&facts) {
        rs_arch_07_required_inputs_fail_closed::check(&input, &mut results);
    }
    rs_arch_07_required_inputs_fail_closed::check_success(
        facts.input_failures.iter().any(|failure| {
            matches!(
                failure.kind,
                self::facts::ArchInputFailureKind::RequiredInput
            )
        }),
        &mut results,
    );

    results
}

pub fn check_test_tree(tree: &ProjectTree) -> Vec<CheckResult> {
    let scope = guardrail3_app_rs_placement::collect(tree);
    let config = tree
        .file_content("guardrail3.toml")
        .and_then(|content| toml::from_str::<GuardrailConfig>(content).ok());
    let selection = RustFamilySelection::new(BTreeSet::from([RustValidateFamily::Arch]));
    let route = FamilyMapper::new(tree, &scope, config.as_ref(), &selection, None).map_rs_arch();
    check(tree, &route)
}
