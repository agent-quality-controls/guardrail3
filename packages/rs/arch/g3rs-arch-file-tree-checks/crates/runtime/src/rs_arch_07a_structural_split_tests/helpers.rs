use g3rs_arch_types::types::{G3RsArchFileTreeCrate, G3RsArchRustPolicyState};
use guardrail3_check_types::G3CheckResult;
use guardrail3_rs_toml_parser::WaiverConfig;

pub(super) fn crate_node(rel_dir: &str) -> G3RsArchFileTreeCrate {
    G3RsArchFileTreeCrate {
        rel_dir: rel_dir.to_owned(),
        cargo_rel_path: format!("{rel_dir}/Cargo.toml"),
        has_package: true,
        has_lib_rs: false,
        has_main_rs: false,
        sibling_rs_file_count: 0,
        sibling_dir_count: 0,
        max_module_depth: 0,
        cargo_parse_error: None,
    }
}

pub(super) fn waiver(rule: &str, file: &str, selector: &str, reason: &str) -> WaiverConfig {
    let parsed = guardrail3_rs_toml_parser::parse(&format!(
        "[[waivers]]\nrule = \"{rule}\"\nfile = \"{file}\"\nselector = \"{selector}\"\nreason = \"{reason}\"\n"
    ))
    .expect("waiver fixture should parse");

    parsed
        .waivers
        .into_iter()
        .next()
        .expect("waiver fixture should contain one waiver")
}

pub(super) fn run_rule(
    node: &G3RsArchFileTreeCrate,
    rust_policy: &G3RsArchRustPolicyState,
) -> Vec<G3CheckResult> {
    let mut results = Vec::new();
    crate::rs_arch_07a_structural_split::check(node, rust_policy, &mut results);
    results
}
