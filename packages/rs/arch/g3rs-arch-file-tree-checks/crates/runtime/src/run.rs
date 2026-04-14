use g3rs_arch_file_tree_checks_types::G3RsArchFileTreeChecksInput;
use guardrail3_check_types::G3CheckResult;

pub fn check(input: &G3RsArchFileTreeChecksInput) -> Vec<G3CheckResult> {
    let mut results = Vec::new();

    for node in &input.crates {
        crate::rs_arch_01_crate_has_facade::check(node, &mut results);
        crate::rs_arch_07a_structural_split::check(node, &input.rust_policy, &mut results);
    }

    for module_dir in &input.module_dirs {
        crate::rs_arch_03_mod_rs_required::check(module_dir, &mut results);
    }

    results
}
