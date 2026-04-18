use std::collections::{BTreeMap, BTreeSet};

use g3rs_apparch_types::{G3RsApparchCrate, G3RsApparchDependencyEdge};
use guardrail3_check_types::{G3CheckResult, G3Severity};

const ID: &str = "RS-APPARCH-CONFIG-06";

pub(crate) fn check(
    crates: &[G3RsApparchCrate],
    dependency_edges: &[G3RsApparchDependencyEdge],
    results: &mut Vec<G3CheckResult>,
) {
    let crates_by_path = crates
        .iter()
        .map(|krate| (krate.cargo_rel_path.clone(), krate))
        .collect::<BTreeMap<_, _>>();
    let mut adjacency = BTreeMap::<String, Vec<String>>::new();
    let mut self_loops = BTreeSet::new();

    for edge in dependency_edges.iter().filter(|edge| !edge.kind.is_dev()) {
        let Some(source) = crates_by_path.get(&edge.from_cargo_rel_path).copied() else {
            continue;
        };
        let Some(target) = crates_by_path.get(&edge.to_cargo_rel_path).copied() else {
            continue;
        };
        let (Some(source_layer), Some(target_layer)) = (source.layer, target.layer) else {
            continue;
        };
        if source_layer != target_layer {
            continue;
        }
        adjacency
            .entry(source.cargo_rel_path.clone())
            .or_default()
            .push(target.cargo_rel_path.clone());
        let _ = adjacency.entry(target.cargo_rel_path.clone()).or_default();
        if source.cargo_rel_path == target.cargo_rel_path {
            let _ = self_loops.insert(source.cargo_rel_path.clone());
        }
    }

    let sccs = strongly_connected_components(&adjacency);
    let mut found_cycle = false;
    for component in sccs {
        let has_cycle = component.len() > 1
            || component
                .first()
                .is_some_and(|member| self_loops.contains(member));
        if !has_cycle {
            continue;
        }
        found_cycle = true;
        let Some(first) = component
            .first()
            .and_then(|path| crates_by_path.get(path).copied())
        else {
            continue;
        };
        let layer = first.layer.expect("component should have a layer");
        let members = component
            .iter()
            .filter_map(|path| crates_by_path.get(path).copied())
            .map(|krate| crate::run::display_crate(krate).to_owned())
            .collect::<Vec<_>>();
        results.push(G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Error,
            format!("same-layer {} dependency cycle", crate::run::layer_label(layer)),
            format!(
                "Found same-layer dependency cycle among {} crate(s): {}. Break the cycle by extracting shared code into one owning crate or removing one dependency edge.",
                crate::run::layer_label(layer),
                members.join(" -> ")
            ),
            None,
            None,
        ));
    }

    if !adjacency.is_empty() && !found_cycle {
        results.push(
            G3CheckResult::new(
                ID.to_owned(),
                G3Severity::Info,
                "no same-layer dependency cycles detected".to_owned(),
                format!(
                    "Apparch checked {} same-layer dependency node(s) and found no non-dev cycles.",
                    adjacency.len()
                ),
                None,
                None,
            )
            .into_inventory(),
        );
    }
}

fn strongly_connected_components(adjacency: &BTreeMap<String, Vec<String>>) -> Vec<Vec<String>> {
    let mut index = 0_usize;
    let mut indices = BTreeMap::<String, usize>::new();
    let mut lowlinks = BTreeMap::<String, usize>::new();
    let mut stack = Vec::<String>::new();
    let mut on_stack = BTreeSet::<String>::new();
    let mut components = Vec::new();

    for node in adjacency.keys() {
        if !indices.contains_key(node) {
            strong_connect(
                node,
                adjacency,
                &mut index,
                &mut indices,
                &mut lowlinks,
                &mut stack,
                &mut on_stack,
                &mut components,
            );
        }
    }

    components
}

fn strong_connect(
    node: &str,
    adjacency: &BTreeMap<String, Vec<String>>,
    index: &mut usize,
    indices: &mut BTreeMap<String, usize>,
    lowlinks: &mut BTreeMap<String, usize>,
    stack: &mut Vec<String>,
    on_stack: &mut BTreeSet<String>,
    components: &mut Vec<Vec<String>>,
) {
    let current_index = *index;
    *index += 1;
    let _ = indices.insert(node.to_owned(), current_index);
    let _ = lowlinks.insert(node.to_owned(), current_index);
    stack.push(node.to_owned());
    let _ = on_stack.insert(node.to_owned());

    if let Some(neighbors) = adjacency.get(node) {
        for neighbor in neighbors {
            if !indices.contains_key(neighbor) {
                strong_connect(
                    neighbor, adjacency, index, indices, lowlinks, stack, on_stack, components,
                );
                let lowlink = lowlinks[node];
                let neighbor_lowlink = lowlinks[neighbor];
                let _ = lowlinks.insert(node.to_owned(), lowlink.min(neighbor_lowlink));
            } else if on_stack.contains(neighbor) {
                let lowlink = lowlinks[node];
                let neighbor_index = indices[neighbor];
                let _ = lowlinks.insert(node.to_owned(), lowlink.min(neighbor_index));
            }
        }
    }

    if lowlinks[node] == indices[node] {
        let mut component = Vec::new();
        while let Some(member) = stack.pop() {
            let _ = on_stack.remove(&member);
            component.push(member.clone());
            if member == node {
                break;
            }
        }
        component.sort();
        components.push(component);
    }
}

#[cfg(test)]
#[path = "rs_apparch_config_06_same_layer_cycles_tests/mod.rs"]
mod rs_apparch_config_06_same_layer_cycles_tests;
