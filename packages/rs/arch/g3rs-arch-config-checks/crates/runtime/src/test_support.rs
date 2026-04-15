use g3rs_arch_types::{G3RsArchConfigChecksInput, G3RsArchConfigCrate, G3RsArchDependencyEdge};

pub(crate) fn config_crate(rel_dir: &str) -> G3RsArchConfigCrate {
    G3RsArchConfigCrate {
        rel_dir: rel_dir.to_owned(),
        cargo_rel_path: join_rel(rel_dir, "Cargo.toml"),
        shared: false,
        production_dependency_count: 0,
        dev_dependency_count: 0,
        requires_feature_contract: false,
        has_default_feature: false,
        has_all_feature: false,
        all_feature_deps: Vec::new(),
        default_feature_deps: Vec::new(),
    }
}

pub(crate) fn shared_config_crate(rel_dir: &str) -> G3RsArchConfigCrate {
    let mut krate = config_crate(rel_dir);
    krate.shared = true;
    krate
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

pub(crate) fn dependency_edge(
    source_rel_dir: &str,
    target_rel_dir: &str,
    section: &str,
) -> G3RsArchDependencyEdge {
    G3RsArchDependencyEdge {
        source_rel_dir: source_rel_dir.to_owned(),
        source_cargo_rel: join_rel(source_rel_dir, "Cargo.toml"),
        dep_alias: target_rel_dir
            .rsplit_once('/')
            .map_or_else(|| target_rel_dir.to_owned(), |(_, tail)| tail.to_owned()),
        raw_path: format!("../{}", target_rel_dir.rsplit_once('/').map_or(target_rel_dir, |(_, tail)| tail)),
        resolved_target_rel: Some(target_rel_dir.to_owned()),
        target_is_crate: true,
        section: section.to_owned(),
        crossed_boundary: Some(g3rs_arch_types::G3RsArchBoundaryRef::RootWorkspace),
        is_direct_child: false,
        target_shared: false,
    }
}

pub(crate) fn shared_dependency_edge(
    source_rel_dir: &str,
    target_rel_dir: &str,
    section: &str,
) -> G3RsArchDependencyEdge {
    let mut edge = dependency_edge(source_rel_dir, target_rel_dir, section);
    edge.target_shared = true;
    edge
}

fn join_rel(dir: &str, child: &str) -> String {
    if dir.is_empty() {
        child.to_owned()
    } else {
        format!("{dir}/{child}")
    }
}
