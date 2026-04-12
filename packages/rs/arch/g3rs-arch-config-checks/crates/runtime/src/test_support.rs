use g3rs_arch_types::{G3RsArchConfigChecksInput, G3RsArchConfigCrate, G3RsArchDependencyEdge};

pub(crate) fn config_crate(rel_dir: &str) -> G3RsArchConfigCrate {
    G3RsArchConfigCrate {
        rel_dir: rel_dir.to_owned(),
        cargo_rel_path: join_rel(rel_dir, "Cargo.toml"),
        shared: false,
        dependency_count: 0,
        requires_feature_contract: false,
        has_default_feature: false,
        has_all_feature: false,
        all_feature_deps: Vec::new(),
        default_feature_deps: Vec::new(),
    }
}

pub(crate) fn input(
    crates: Vec<G3RsArchConfigCrate>,
    dependency_edges: Vec<G3RsArchDependencyEdge>,
) -> G3RsArchConfigChecksInput {
    G3RsArchConfigChecksInput {
        crates,
        dependency_edges,
    }
}

fn join_rel(dir: &str, child: &str) -> String {
    if dir.is_empty() {
        child.to_owned()
    } else {
        format!("{dir}/{child}")
    }
}
