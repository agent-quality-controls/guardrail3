use std::collections::{BTreeMap, BTreeSet};

use g3rs_apparch_types::G3RsApparchSameLayerCyclesChecksInput;
use guardrail3_check_types::{G3CheckResult, G3Severity};

const ID: &str = "g3rs-apparch/same-layer-cycles";

pub(crate) fn check(
    input: &G3RsApparchSameLayerCyclesChecksInput,
    results: &mut Vec<G3CheckResult>,
) {
    let mut adjacency = BTreeMap::<String, Vec<String>>::new();
    let mut crates_by_path = BTreeMap::new();
    let mut self_loops = BTreeSet::new();

    for edge in &input.edges {
        let _ = crates_by_path.insert(edge.from.cargo_rel_path.clone(), &edge.from);
        let _ = crates_by_path.insert(edge.to.cargo_rel_path.clone(), &edge.to);
        adjacency
            .entry(edge.from.cargo_rel_path.clone())
            .or_default()
            .push(edge.to.cargo_rel_path.clone());
        let _ = adjacency.entry(edge.to.cargo_rel_path.clone()).or_default();
        if edge.from.cargo_rel_path == edge.to.cargo_rel_path {
            let _ = self_loops.insert(edge.from.cargo_rel_path.clone());
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
        let Some(layer) = first.layer else {
            continue;
        };
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

#[allow(
    clippy::too_many_arguments,
    reason = "Tarjan's SCC requires threading 8 mutable working sets (index, indices, lowlinks, stack, on_stack, components) through the recursion; packing into a struct would obscure the canonical algorithm"
)]
#[allow(
    clippy::arithmetic_side_effects,
    reason = "Tarjan's SCC index is bounded by adjacency.len() (the node count); usize overflow is impossible in any reachable workspace"
)]
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
