use g3rs_hooks_types::G3RsHooksFileTreeChecksInput;
use guardrail3_check_types::G3CheckResult;

pub fn check(input: &G3RsHooksFileTreeChecksInput) -> Vec<G3CheckResult> {
    if !input.active {
        return Vec::new();
    }

    let mut results = Vec::new();

    crate::pre_commit_exists::check(input.pre_commit.as_ref(), &mut results);
    crate::hooks_path_configured::check(input.hooks_path.as_deref(), &mut results);
    crate::modular_directory_inventory::check(input.has_modular_dir, &mut results);
    crate::modular_scripts_inventory::check(&input.modular_scripts, &mut results);
    crate::local_override_inventory::check(&input.local_override_scripts, &mut results);
    crate::modular_scripts_executable::check(&input.modular_scripts, &mut results);
    crate::execution_trust::check(&input.trust_risks, &mut results);

    if let Some(pre_commit) = input.pre_commit.as_ref() {
        crate::pre_commit_executable::check(pre_commit, &mut results);
        crate::script_stats_inventory::check(pre_commit, &mut results);
        crate::pre_commit_file_size_inventory::check(pre_commit, &mut results);
    }

    results
}
