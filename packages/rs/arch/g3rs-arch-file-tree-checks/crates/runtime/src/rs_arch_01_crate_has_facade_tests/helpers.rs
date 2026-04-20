use g3rs_arch_types::types::G3RsArchFileTreeCrate;
use guardrail3_check_types::G3CheckResult;

pub(super) fn crate_node(rel_dir: &str) -> G3RsArchFileTreeCrate {
    G3RsArchFileTreeCrate {
        rel_dir: rel_dir.to_owned(),
        cargo_rel_path: format!("{rel_dir}/Cargo.toml"),
        has_package: true,
        has_lib_rs: false,
        has_main_rs: false,
        max_sibling_rs_file_count: 0,
        max_sibling_dir_count: 0,
        max_module_depth: 0,
        cargo_parse_error: None,
    }
}

pub(super) fn run_rule(node: &G3RsArchFileTreeCrate) -> Vec<G3CheckResult> {
    let mut results = Vec::new();
    crate::rs_arch_01_crate_has_facade::check(node, &mut results);
    results
}
