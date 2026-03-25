mod rust_root_placement;
mod facts;
mod inputs;
mod rs_arch_01_root_classification;
mod rs_arch_02_no_misplaced_roots;
mod rs_arch_03_no_dual_ownership;
mod rs_arch_04_no_zone_overlap;
mod rs_arch_05_enablement_coherence;

use guardrail3_domain_report::CheckResult;
use guardrail3_domain_project_tree::ProjectTree;

use self::facts::collect;
use self::inputs::{
    DualOwnershipInput, EnablementCoherenceInput, MisplacedRootInput, RootClassificationInput,
    ZoneOverlapInput,
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

    for input in DualOwnershipInput::from_facts(&facts) {
        rs_arch_03_no_dual_ownership::check(&input, &mut results);
    }

    for input in ZoneOverlapInput::from_facts(&facts) {
        rs_arch_04_no_zone_overlap::check(&input, &mut results);
    }

    for input in EnablementCoherenceInput::from_facts(&facts) {
        rs_arch_05_enablement_coherence::check(&input, &mut results);
    }

    results
}

#[cfg(test)]
mod test_support;
