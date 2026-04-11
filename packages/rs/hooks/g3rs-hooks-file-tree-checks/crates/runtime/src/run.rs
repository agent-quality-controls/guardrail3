use g3rs_hooks_file_tree_checks_types::G3RsHooksFileTreeChecksInput;
use guardrail3_check_types::G3CheckResult;

pub fn check(input: &G3RsHooksFileTreeChecksInput) -> Vec<G3CheckResult> {
    let mut results = Vec::new();

    crate::hook_shared_01_pre_commit_exists::check(input.pre_commit.as_ref(), &mut results);
    crate::hook_shared_02_hooks_path_configured::check(input.hooks_path.as_deref(), &mut results);
    crate::hook_shared_03_modular_directory_inventory::check(input.has_modular_dir, &mut results);
    crate::hook_shared_07_modular_scripts_inventory::check(&input.modular_scripts, &mut results);
    crate::hook_shared_09_local_override_inventory::check(
        &input.local_override_scripts,
        &mut results,
    );
    crate::hook_shared_12_modular_scripts_executable::check(&input.modular_scripts, &mut results);
    crate::hook_shared_17_execution_trust::check(&input.trust_risks, &mut results);

    if let Some(pre_commit) = input.pre_commit.as_ref() {
        crate::hook_shared_05_pre_commit_executable::check(pre_commit, &mut results);
        crate::hook_shared_06_script_stats_inventory::check(pre_commit, &mut results);
        crate::hook_shared_08_pre_commit_file_size_inventory::check(pre_commit, &mut results);
    }

    results
}
