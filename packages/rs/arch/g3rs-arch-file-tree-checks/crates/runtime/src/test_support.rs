use g3rs_arch_types::{
    G3RsArchFileTreeChecksInput, G3RsArchFileTreeCrate, G3RsArchModuleDir,
    G3RsArchRustPolicyState,
};
use guardrail3_rs_toml_parser::WaiverConfig;

pub(crate) fn crate_node(rel_dir: &str) -> G3RsArchFileTreeCrate {
    G3RsArchFileTreeCrate {
        rel_dir: rel_dir.to_owned(),
        cargo_rel_path: join_rel(rel_dir, "Cargo.toml"),
        has_package: true,
        has_lib_rs: false,
        has_main_rs: false,
        sibling_rs_file_count: 0,
        sibling_dir_count: 0,
        max_module_depth: 0,
        cargo_parse_error: None,
    }
}

pub(crate) fn module_dir(dir_rel: &str) -> G3RsArchModuleDir {
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

pub(crate) fn input(
    crates: Vec<G3RsArchFileTreeCrate>,
    module_dirs: Vec<G3RsArchModuleDir>,
) -> G3RsArchFileTreeChecksInput {
    G3RsArchFileTreeChecksInput {
        crates,
        module_dirs,
        rust_policy: G3RsArchRustPolicyState::Missing,
    }
}

pub(crate) fn input_with_rust_policy(
    crates: Vec<G3RsArchFileTreeCrate>,
    module_dirs: Vec<G3RsArchModuleDir>,
    rust_policy: G3RsArchRustPolicyState,
) -> G3RsArchFileTreeChecksInput {
    G3RsArchFileTreeChecksInput {
        crates,
        module_dirs,
        rust_policy,
    }
}

pub(crate) fn waiver(rule: &str, file: &str, selector: &str, reason: &str) -> WaiverConfig {
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

fn join_rel(dir: &str, child: &str) -> String {
    if dir.is_empty() {
        child.to_owned()
    } else {
        format!("{dir}/{child}")
    }
}
