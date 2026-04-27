use g3rs_arch_types::types::{G3RsArchConfigCrate, G3RsArchRustPolicyState};
use guardrail3_check_types::G3CheckResult;

pub(super) fn config_crate(rel_dir: &str) -> G3RsArchConfigCrate {
    G3RsArchConfigCrate {
        rel_dir: rel_dir.to_owned(),
        cargo_rel_path: format!("{rel_dir}/Cargo.toml"),
        shared: false,
        production_dependency_count: 0,
        dev_dependency_count: 0,
        requires_feature_contract: false,
        has_default_feature: false,
        has_all_feature: false,
        all_feature_deps: Vec::new(),
        default_feature_deps: Vec::new(),
    }
}

pub(super) fn run_rule(node: &G3RsArchConfigCrate) -> Vec<G3CheckResult> {
    let mut results = Vec::new();
    crate::dependency_count_split::check(node, &G3RsArchRustPolicyState::Missing, &mut results);
    results
}
