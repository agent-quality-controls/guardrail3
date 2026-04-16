use g3rs_arch_types::G3RsArchDependencyEdge;
use guardrail3_check_types::G3CheckResult;

pub(super) fn dependency_edge(
    source_rel_dir: &str,
    target_rel_dir: &str,
    section: &str,
) -> G3RsArchDependencyEdge {
    G3RsArchDependencyEdge {
        source_rel_dir: source_rel_dir.to_owned(),
        source_cargo_rel: format!("{source_rel_dir}/Cargo.toml"),
        dep_alias: target_rel_dir
            .rsplit_once('/')
            .map_or_else(|| target_rel_dir.to_owned(), |(_, tail)| tail.to_owned()),
        raw_path: format!(
            "../{}",
            target_rel_dir
                .rsplit_once('/')
                .map_or(target_rel_dir, |(_, tail)| tail)
        ),
        resolved_target_rel: Some(target_rel_dir.to_owned()),
        target_is_crate: true,
        section: section.to_owned(),
        crossed_boundary: Some(g3rs_arch_types::G3RsArchBoundaryRef::RootWorkspace),
        is_direct_child: false,
        target_shared: false,
    }
}

pub(super) fn allow_shared_target(mut edge: G3RsArchDependencyEdge) -> G3RsArchDependencyEdge {
    edge.target_shared = true;
    edge
}

pub(super) fn run_rule(edge: &G3RsArchDependencyEdge) -> Vec<G3CheckResult> {
    let mut results = Vec::new();
    crate::rs_arch_05_no_boundary_crossing::check(edge, &mut results);
    results
}
