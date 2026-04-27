use g3rs_arch_types::G3RsArchFileTreeChecksInput;
use guardrail3_check_types::G3CheckResult;

pub fn check(input: &G3RsArchFileTreeChecksInput) -> Vec<G3CheckResult> {
    let mut results = Vec::new();

    for node in &input.crates {
        crate::crate_has_facade::check(node, &mut results);
        crate::structural_split::check(node, &input.rust_policy, &mut results);
    }

    for module_dir in &input.module_dirs {
        crate::mod_rs_required::check(module_dir, &mut results);
    }

    results
}
