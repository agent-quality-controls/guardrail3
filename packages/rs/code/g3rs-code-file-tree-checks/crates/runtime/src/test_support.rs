use g3rs_code_file_tree_checks_types::{G3RsCodeFileTreeChecksInput, G3RsCodeStructuralCapRoot};

pub(crate) fn root(rel_dir: &str) -> G3RsCodeStructuralCapRoot {
    G3RsCodeStructuralCapRoot {
        root_rel_dir: rel_dir.to_owned(),
        cargo_rel_path: join_rel(rel_dir, "Cargo.toml"),
        max_module_depth: 0,
        max_sibling_dirs: 0,
        max_sibling_rs_files: 0,
    }
}

pub(crate) fn input(roots: Vec<G3RsCodeStructuralCapRoot>) -> G3RsCodeFileTreeChecksInput {
    G3RsCodeFileTreeChecksInput { roots }
}

fn join_rel(dir: &str, child: &str) -> String {
    if dir.is_empty() {
        child.to_owned()
    } else {
        format!("{dir}/{child}")
    }
}
