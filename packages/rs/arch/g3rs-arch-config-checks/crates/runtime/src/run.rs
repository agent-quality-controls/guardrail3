use g3rs_arch_types::G3RsArchConfigChecksInput;
use guardrail3_check_types::G3CheckResult;

pub fn check(input: &G3RsArchConfigChecksInput) -> Vec<G3CheckResult> {
    let mut results = Vec::new();

    for node in &input.crates {
        crate::dependency_count_split::check(node, &input.rust_policy, &mut results);
        crate::feature_contract::check(node, &mut results);
    }

    for edge in &input.dependency_edges {
        crate::no_boundary_crossing::check(edge, &mut results);
        crate::shared_flag_required::check(edge, &mut results);
    }

    results
}
