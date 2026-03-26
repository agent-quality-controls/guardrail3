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

use guardrail3_domain_project_tree::ProjectTree;
use guardrail3_domain_report::CheckResult;

use self::facts::collect;
use self::inputs::{
    AuxiliaryRootInput, DualOwnershipInput, MisplacedRootInput, OwnerFamilyCoherenceInput,
    RequiredInputFailureInput, RootClassificationInput, ScopedArchConfigInput, ZoneOverlapInput,
};

pub fn check(tree: &ProjectTree) -> Vec<CheckResult> {
    let facts = collect(tree);
    let mut results = Vec::new();

    for input in RootClassificationInput::from_facts(&facts) {
        rs_arch_01_root_classification::check(&input, &mut results);
    }

    for input in MisplacedRootInput::from_facts(&facts) {
        rs_arch_02_no_misplaced_roots::check(&input, &mut results);
    }

    for input in AuxiliaryRootInput::from_facts(&facts) {
        rs_arch_08_auxiliary_roots_declared::check(&input, &mut results);
    }

    for input in DualOwnershipInput::from_facts(&facts) {
        rs_arch_03_no_dual_ownership::check(&input, &mut results);
    }

    for input in ZoneOverlapInput::from_facts(&facts) {
        rs_arch_04_no_zone_overlap::check(&input, &mut results);
    }

    for input in ScopedArchConfigInput::from_facts(&facts) {
        rs_arch_05_scoped_arch_config_forbidden::check(&input, &mut results);
    }

    for input in OwnerFamilyCoherenceInput::from_facts(&facts) {
        rs_arch_06_owner_family_enablement_coherence::check(&input, &mut results);
    }

    for input in RequiredInputFailureInput::from_facts(&facts) {
        rs_arch_07_required_inputs_fail_closed::check(&input, &mut results);
    }

    results
}

#[cfg(test)]
mod test_support;
