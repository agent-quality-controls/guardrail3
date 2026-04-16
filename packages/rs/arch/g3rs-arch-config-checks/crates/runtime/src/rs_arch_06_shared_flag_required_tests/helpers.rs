use std::collections::BTreeMap;

use g3rs_arch_types::{G3RsArchConfigCrate, G3RsArchDependencyEdge};
use guardrail3_check_types::G3CheckResult;

fn config_crate(rel_dir: &str) -> G3RsArchConfigCrate {
    G3RsArchConfigCrate {
        rel_dir: rel_dir.to_owned(),
        cargo_rel_path: format!("{rel_dir}/Cargo.toml"),
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

pub(super) fn run_rule(edge: &G3RsArchDependencyEdge) -> Vec<G3CheckResult> {
    let source = config_crate(&edge.source_rel_dir);
    let target = config_crate(edge.resolved_target_rel.as_deref().unwrap_or(""));
    let crate_map = [
        (source.rel_dir.as_str(), &source),
        (target.rel_dir.as_str(), &target),
    ]
    .into_iter()
    .collect::<BTreeMap<_, _>>();
    let mut results = Vec::new();

    crate::rs_arch_06_shared_flag_required::check(edge, &crate_map, &mut results);

    results
}
