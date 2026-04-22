use g3rs_apparch_config_checks_assertions::rs_apparch_config_06_same_layer_cycles as assertions;
use g3rs_apparch_types::{
    G3RsApparchLayer, G3RsApparchSameLayerCyclesChecksInput,
    G3RsApparchSameLayerDependencyEdge,
};

use super::helpers::{krate, run_rule, same_layer_edge};

#[test]
fn same_layer_cycle_fires() {
    let types_a = krate(G3RsApparchLayer::Types, "types/a/Cargo.toml");
    let types_b = krate(G3RsApparchLayer::Types, "types/b/Cargo.toml");
    let types_c = krate(G3RsApparchLayer::Types, "types/c/Cargo.toml");
    let edges = vec![
        same_layer_edge(&types_a, &types_b),
        same_layer_edge(&types_b, &types_c),
        same_layer_edge(&types_c, &types_a),
    ];
    let results = run_rule(&edges);

    assertions::assert_cycle_members(
        &results,
        "same-layer types dependency cycle",
        "types crate(s)",
        &[&types_a.crate_name, &types_b.crate_name, &types_c.crate_name],
    );
}

#[test]
fn acyclic_same_layer_graph_reports_inventory() {
    let types_a = krate(G3RsApparchLayer::Types, "types/a/Cargo.toml");
    let types_b = krate(G3RsApparchLayer::Types, "types/b/Cargo.toml");
    let types_c = krate(G3RsApparchLayer::Types, "types/c/Cargo.toml");
    let edges = vec![
        same_layer_edge(&types_a, &types_b),
        same_layer_edge(&types_b, &types_c),
    ];
    let results = run_rule(&edges);

    assertions::assert_inventory_checked_nodes(&results, 3);
}

#[test]
fn same_layer_self_loop_fires() {
    let logic_service = krate(G3RsApparchLayer::Logic, "logic/service/Cargo.toml");
    let edges = vec![same_layer_edge(&logic_service, &logic_service)];
    let results = run_rule(&edges);

    assertions::assert_cycle_members(
        &results,
        "same-layer logic dependency cycle",
        "logic crate(s)",
        &[&logic_service.crate_name],
    );
}

#[test]
fn cycle_is_not_dropped_when_first_sorted_node_is_missing_from_crate_bag() {
    let types_a = krate(G3RsApparchLayer::Types, "types/a/Cargo.toml");
    let types_b = krate(G3RsApparchLayer::Types, "types/b/Cargo.toml");
    let input = G3RsApparchSameLayerCyclesChecksInput {
        edges: vec![
            G3RsApparchSameLayerDependencyEdge {
                from: types_a.clone(),
                to: types_b.clone(),
            },
            G3RsApparchSameLayerDependencyEdge {
                from: types_b.clone(),
                to: types_a.clone(),
            },
        ],
    };
    let mut results = Vec::new();
    crate::rs_apparch_config_06_same_layer_cycles::check(&input, &mut results);

    assertions::assert_cycle_members(
        &results,
        "same-layer types dependency cycle",
        "types crate(s)",
        &[&types_a.crate_name, &types_b.crate_name],
    );
}
