use g3rs_arch_types::{G3RsArchCrateNode, G3RsArchFileTreeChecksInput, G3RsArchModuleDir};

pub(crate) fn crate_node(rel_dir: &str) -> G3RsArchCrateNode {
    G3RsArchCrateNode {
        rel_dir: rel_dir.to_owned(),
        cargo_rel_path: join_rel(rel_dir, "Cargo.toml"),
        package_name: Some(rel_dir.replace('/', "_")),
        has_package: true,
        has_workspace: false,
        has_lib_rs: false,
        has_main_rs: false,
        lib_rs_rel: None,
        parent_rel_dir: None,
        shared: false,
        has_default_feature: false,
        has_all_feature: false,
        all_feature_deps: Vec::new(),
        default_feature_deps: Vec::new(),
        dependency_count: 0,
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
    crate_nodes: Vec<G3RsArchCrateNode>,
    module_dirs: Vec<G3RsArchModuleDir>,
) -> G3RsArchFileTreeChecksInput {
    G3RsArchFileTreeChecksInput {
        crate_nodes,
        module_dirs,
    }
}

fn join_rel(dir: &str, child: &str) -> String {
    if dir.is_empty() {
        child.to_owned()
    } else {
        format!("{dir}/{child}")
    }
}
