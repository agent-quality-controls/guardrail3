use g3rs_apparch_config_checks_assertions::rs_apparch_config_06_same_layer_cycles as assertions;
use g3rs_apparch_types::{
    G3RsApparchDependencyKind, G3RsApparchLayer, G3RsApparchSameLayerCyclesChecksInput,
    G3RsApparchSameLayerDependencyEdge,
};

use super::helpers::{edge, krate, run_rule};

#[test]
fn same_layer_cycle_fires() {
    let crates = vec![
        krate(G3RsApparchLayer::Types, "types/a/Cargo.toml"),
        krate(G3RsApparchLayer::Types, "types/b/Cargo.toml"),
        krate(G3RsApparchLayer::Types, "types/c/Cargo.toml"),
    ];
    let edges = vec![
        edge(
            "types/a/Cargo.toml",
            "types/b/Cargo.toml",
            G3RsApparchDependencyKind::Dependency,
        ),
        edge(
            "types/b/Cargo.toml",
            "types/c/Cargo.toml",
            G3RsApparchDependencyKind::Dependency,
        ),
        edge(
            "types/c/Cargo.toml",
            "types/a/Cargo.toml",
            G3RsApparchDependencyKind::Dependency,
        ),
    ];
    let results = run_rule(&crates, &edges);

    assertions::assert_cycle(
        &results,
        "same-layer types dependency cycle",
        "types crate(s)",
    );
}

#[test]
fn dev_only_cycle_is_ignored() {
    let crates = vec![
        krate(G3RsApparchLayer::Types, "types/a/Cargo.toml"),
        krate(G3RsApparchLayer::Types, "types/b/Cargo.toml"),
    ];
    let edges = vec![
        edge(
            "types/a/Cargo.toml",
            "types/b/Cargo.toml",
            G3RsApparchDependencyKind::DevDependency,
        ),
        edge(
            "types/b/Cargo.toml",
            "types/a/Cargo.toml",
            G3RsApparchDependencyKind::TargetDevDependency,
        ),
    ];
    let results = run_rule(&crates, &edges);

    assertions::assert_no_findings(&results);
}

#[test]
fn same_layer_self_loop_fires() {
    let crates = vec![krate(G3RsApparchLayer::Logic, "logic/service/Cargo.toml")];
    let edges = vec![edge(
        "logic/service/Cargo.toml",
        "logic/service/Cargo.toml",
        G3RsApparchDependencyKind::BuildDependency,
    )];
    let results = run_rule(&crates, &edges);

    assertions::assert_cycle(
        &results,
        "same-layer logic dependency cycle",
        "logic crate(s)",
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
                from: types_b,
                to: types_a,
            },
        ],
    };
    let mut results = Vec::new();
    crate::rs_apparch_config_06_same_layer_cycles::check(&input, &mut results);

    assertions::assert_cycle(
        &results,
        "same-layer types dependency cycle",
        "types crate(s)",
    );
}
