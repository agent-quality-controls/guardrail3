use std::collections::BTreeMap;

use g3rs_arch_config_checks_types::G3RsArchConfigChecksInput;
use g3rs_arch_types::G3RsArchConfigCrate;
use guardrail3_check_types::G3CheckResult;

pub fn check(input: &G3RsArchConfigChecksInput) -> Vec<G3CheckResult> {
    let crate_map = input
        .crates
        .iter()
        .map(|node| (node.rel_dir.as_str(), node))
        .collect::<BTreeMap<_, _>>();
    let mut results = Vec::new();

    for node in &input.crates {
        crate::rs_arch_07b_dependency_count_split::check(node, &mut results);
        crate::rs_arch_08b_feature_contract::check(node, &mut results);
    }

    for edge in &input.dependency_edges {
        crate::rs_arch_05_no_boundary_crossing::check(edge, &mut results);
        crate::rs_arch_06_shared_flag_required::check(edge, &crate_map, &mut results);
    }

    results
}

pub(crate) type CrateMap<'a> = BTreeMap<&'a str, &'a G3RsArchConfigCrate>;
