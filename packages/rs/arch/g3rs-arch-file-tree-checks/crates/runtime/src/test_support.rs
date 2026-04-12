use g3rs_arch_types::{G3RsArchFileTreeChecksInput, G3RsArchFileTreeCrate, G3RsArchModuleDir};

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
    }
}

fn join_rel(dir: &str, child: &str) -> String {
    if dir.is_empty() {
        child.to_owned()
    } else {
        format!("{dir}/{child}")
    }
}
