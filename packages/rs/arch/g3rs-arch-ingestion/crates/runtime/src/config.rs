use std::collections::BTreeMap;

use toml::Value;

use g3rs_arch_types::types::{
    G3RsArchBoundaryRef, G3RsArchConfigChecksInput, G3RsArchConfigCrate, G3RsArchCrateNode,
    G3RsArchDependencyEdge, G3RsArchFacadeSurface,
};
use g3rs_workspace_crawl::G3RsWorkspaceCrawl;

use crate::error::G3RsArchIngestionError;
use crate::view::CrawlView;
use crate::workspace::{collect_crate_nodes, is_inside, normalize_path, parent_of};

pub(crate) fn ingest_for_config_checks(
    crawl: &G3RsWorkspaceCrawl,
) -> Result<Vec<G3RsArchConfigChecksInput>, G3RsArchIngestionError> {
    let view = CrawlView::new(crawl);
    let crate_nodes = collect_crate_nodes(&view)?;
    let facade_surfaces = crate::source::collect_facade_surfaces(&view, &crate_nodes);
    let dependency_edges = collect_dependency_edges(&view, &crate_nodes)?;

    Ok(vec![G3RsArchConfigChecksInput {
        crates: collect_config_crates(&crate_nodes, &facade_surfaces),
        dependency_edges,
        rust_policy: crate::file_tree::ingest_rust_policy(&view),
    }])
}

fn collect_config_crates(
    crate_nodes: &[G3RsArchCrateNode],
    facade_surfaces: &[G3RsArchFacadeSurface],
) -> Vec<G3RsArchConfigCrate> {
    let requires_feature_contract = facade_surfaces
        .iter()
        .filter(|surface| surface.is_lib_rs && surface.pub_export_count > 0)
        .map(|surface| surface.rel_path.as_str())
        .collect::<std::collections::BTreeSet<_>>();

    crate_nodes
        .iter()
        .map(|node| G3RsArchConfigCrate {
            rel_dir: node.rel_dir.clone(),
            cargo_rel_path: node.cargo_rel_path.clone(),
            shared: node.shared,
            production_dependency_count: node.dependency_counts.production,
            dev_dependency_count: node.dependency_counts.dev,
            requires_feature_contract: node
                .lib_rs_rel
                .as_deref()
                .is_some_and(|rel| requires_feature_contract.contains(rel)),
            has_default_feature: node.feature_contract.has_default_feature,
            has_all_feature: node.feature_contract.has_all_feature,
            all_feature_deps: node.feature_contract.all_feature_deps.clone(),
            default_feature_deps: node.feature_contract.default_feature_deps.clone(),
        })
        .collect()
}

fn collect_dependency_edges(
    view: &CrawlView<'_>,
    crate_nodes: &[G3RsArchCrateNode],
) -> Result<Vec<G3RsArchDependencyEdge>, G3RsArchIngestionError> {
    let mut edges = Vec::new();
    let node_map = crate_nodes
        .iter()
        .map(|node| (node.rel_dir.as_str(), node))
        .collect::<BTreeMap<_, _>>();

    for node in crate_nodes {
        if node.cargo_parse_error.is_some() {
            continue;
        }
        let content = view.read_file(&node.cargo_rel_path).map_err(|err| {
            G3RsArchIngestionError::Unreadable {
                path: view
                    .abs_path(&node.cargo_rel_path)
                    .unwrap_or_else(|| std::path::PathBuf::from(&node.cargo_rel_path)),
                reason: err.to_string(),
            }
        })?;
        let Ok(parsed) = toml::from_str::<Value>(&content) else {
            continue;
        };

        for section in ["dependencies", "dev-dependencies", "build-dependencies"] {
            if let Some(deps) = parsed.get(section).and_then(Value::as_table) {
                collect_section_edges(
                    &node.rel_dir,
                    &node.cargo_rel_path,
                    section,
                    deps,
                    crate_nodes,
                    &node_map,
                    &mut edges,
                );
            }
        }

        if let Some(targets) = parsed.get("target").and_then(Value::as_table) {
            for target_cfg in targets.values() {
                let Some(target_table) = target_cfg.as_table() else {
                    continue;
                };
                for section in ["dependencies", "dev-dependencies", "build-dependencies"] {
                    if let Some(deps) = target_table.get(section).and_then(Value::as_table) {
                        collect_section_edges(
                            &node.rel_dir,
                            &node.cargo_rel_path,
                            section,
                            deps,
                            crate_nodes,
                            &node_map,
                            &mut edges,
                        );
                    }
                }
            }
        }
    }

    Ok(edges)
}

fn collect_section_edges(
    source_rel_dir: &str,
    source_cargo_rel: &str,
    section: &str,
    deps: &toml::map::Map<String, Value>,
    crate_nodes: &[G3RsArchCrateNode],
    node_map: &BTreeMap<&str, &G3RsArchCrateNode>,
    edges: &mut Vec<G3RsArchDependencyEdge>,
) {
    for (alias, value) in deps {
        let Some(raw_path) = extract_path(value) else {
            continue;
        };
        let resolved_target_rel = Some(normalize_path(source_rel_dir, &raw_path));
        let target_is_crate = resolved_target_rel
            .as_ref()
            .is_some_and(|rel| node_map.contains_key(rel.as_str()));
        if !target_is_crate {
            continue;
        }
        let crossed_boundary = resolved_target_rel
            .as_ref()
            .and_then(|target_rel| boundary_violation(crate_nodes, source_rel_dir, target_rel));
        let is_direct_child = resolved_target_rel
            .as_ref()
            .is_some_and(|target_rel| is_direct_child(crate_nodes, source_rel_dir, target_rel));
        let target_shared = resolved_target_rel
            .as_ref()
            .and_then(|target_rel| node_map.get(target_rel.as_str()))
            .is_some_and(|node| node.shared);

        edges.push(G3RsArchDependencyEdge {
            source_rel_dir: source_rel_dir.to_owned(),
            source_cargo_rel: source_cargo_rel.to_owned(),
            dep_alias: alias.clone(),
            raw_path,
            resolved_target_rel,
            target_is_crate,
            section: section.to_owned(),
            crossed_boundary,
            is_direct_child,
            target_shared,
        });
    }
}

fn extract_path(value: &Value) -> Option<String> {
    match value {
        Value::Table(table) => table.get("path").and_then(Value::as_str).map(str::to_owned),
        _ => None,
    }
}

fn boundary_violation(
    crate_nodes: &[G3RsArchCrateNode],
    source_rel_dir: &str,
    target_rel_dir: &str,
) -> Option<G3RsArchBoundaryRef> {
    let mut current = target_rel_dir;
    loop {
        let Some((parent, _)) = current.rsplit_once('/') else {
            let has_root = crate_nodes.iter().any(|node| node.rel_dir.is_empty());
            if has_root && !target_rel_dir.is_empty() && !source_rel_dir.is_empty() {
                return Some(G3RsArchBoundaryRef::RootWorkspace);
            }
            return None;
        };
        if crate_nodes.iter().any(|node| node.rel_dir == parent)
            && parent != target_rel_dir
            && parent != source_rel_dir
            && !is_inside(source_rel_dir, parent)
        {
            return Some(G3RsArchBoundaryRef::Crate(parent.to_owned()));
        }
        current = parent;
    }
}

fn is_direct_child(crate_nodes: &[G3RsArchCrateNode], parent_rel: &str, child_rel: &str) -> bool {
    if !is_inside(child_rel, parent_rel) {
        return false;
    }
    parent_of(crate_nodes, child_rel).is_some_and(|rel| rel == parent_rel)
}

#[cfg(test)]
#[path = "config_tests/mod.rs"]
mod config_tests;
