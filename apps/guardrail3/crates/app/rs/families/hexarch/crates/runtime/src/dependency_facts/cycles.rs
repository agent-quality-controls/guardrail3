use std::collections::{BTreeMap, BTreeSet};

use super::{CycleFacts, DependencyEdgeFacts, MemberDependencyFacts};

pub(super) fn collect_same_layer_cycles(
    edges: &[DependencyEdgeFacts],
    member_by_dir: &BTreeMap<String, &MemberDependencyFacts>,
) -> Vec<CycleFacts> {
    let mut graph: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for edge in edges
        .iter()
        .filter(|edge| !edge.kind.is_dev() && !edge.kind.is_target())
    {
        let Some(target_rel) = &edge.resolved_target_rel_dir else {
            continue;
        };
        if !member_by_dir.contains_key(target_rel) {
            continue;
        }
        graph
            .entry(edge.source_rel_dir.clone())
            .or_default()
            .push(target_rel.clone());
    }

    let mut cycles = Vec::new();
    let mut seen = BTreeSet::new();
    for start in graph.keys() {
        let mut stack = Vec::<String>::new();
        dfs_cycle(
            start,
            start,
            &graph,
            &mut stack,
            &mut seen,
            &mut cycles,
            member_by_dir,
        );
    }
    cycles
}

fn dfs_cycle(
    start: &str,
    node: &str,
    graph: &BTreeMap<String, Vec<String>>,
    stack: &mut Vec<String>,
    seen: &mut BTreeSet<String>,
    cycles: &mut Vec<CycleFacts>,
    member_by_dir: &BTreeMap<String, &MemberDependencyFacts>,
) {
    stack.push(node.to_owned());
    if let Some(neighbors) = graph.get(node) {
        for neighbor in neighbors {
            if neighbor == start && stack.len() > 1 {
                let cycle = canonical_cycle(stack);
                let cycle_key = cycle.join(" -> ");
                if seen.insert(cycle_key) {
                    let member_layers = cycle
                        .iter()
                        .map(|member| member_by_dir.get(member).and_then(|facts| facts.layer))
                        .collect::<Vec<_>>();
                    if let Some(layer) = member_layers.first().copied().flatten().filter(|layer| {
                        member_layers
                            .iter()
                            .all(|candidate| *candidate == Some(*layer))
                    }) {
                        cycles.push(CycleFacts {
                            layer,
                            members: cycle,
                        });
                    }
                }
            } else if !stack.contains(neighbor) {
                dfs_cycle(start, neighbor, graph, stack, seen, cycles, member_by_dir);
            }
        }
    }
    let _ = stack.pop();
}

fn canonical_cycle(stack: &[String]) -> Vec<String> {
    let mut cycle = stack.to_vec();
    if cycle.is_empty() {
        return cycle;
    }
    let (min_index, _) = cycle
        .iter()
        .enumerate()
        .min_by(|(_, left), (_, right)| left.cmp(right))
        .expect("non-empty cycle");
    cycle.rotate_left(min_index);
    cycle
}
