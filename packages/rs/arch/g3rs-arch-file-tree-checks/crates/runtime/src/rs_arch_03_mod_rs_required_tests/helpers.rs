use g3rs_arch_types::G3RsArchModuleDir;
use guardrail3_check_types::G3CheckResult;

pub(super) fn module_dir(dir_rel: &str) -> G3RsArchModuleDir {
    G3RsArchModuleDir {
        dir_rel: dir_rel.to_owned(),
        mod_decl_file: String::new(),
        mod_decl_line: 0,
        is_pub: false,
        has_mod_rs: false,
        has_sibling_file: false,
        rs_file_count: 1,
    }
}

pub(super) fn run_rule(module_dir: &G3RsArchModuleDir) -> Vec<G3CheckResult> {
    let mut results = Vec::new();
    crate::rs_arch_03_mod_rs_required::check(module_dir, &mut results);
    results
}
