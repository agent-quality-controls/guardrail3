use g3rs_apparch_types::{
    G3RsApparchCrate, G3RsApparchDependencyEdge, G3RsApparchDependencyKind, G3RsApparchLayer,
};
use guardrail3_check_types::G3Severity;

fn krate(layer: G3RsApparchLayer, cargo_rel_path: &str) -> G3RsApparchCrate {
    G3RsApparchCrate {
        crate_name: cargo_rel_path.replace('/', "-"),
        cargo_rel_path: cargo_rel_path.to_owned(),
        rel_dir: cargo_rel_path.trim_end_matches("/Cargo.toml").to_owned(),
        layer: Some(layer),
    }
}

fn edge(from: &str, to: &str, kind: G3RsApparchDependencyKind) -> G3RsApparchDependencyEdge {
    G3RsApparchDependencyEdge {
        from_cargo_rel_path: from.to_owned(),
        to_cargo_rel_path: to.to_owned(),
        dep_name: to.to_owned(),
        kind,
    }
}

#[test]
fn same_layer_cycle_fires() {
    let crates = vec![
        krate(G3RsApparchLayer::Types, "types/a/Cargo.toml"),
        krate(G3RsApparchLayer::Types, "types/b/Cargo.toml"),
        krate(G3RsApparchLayer::Types, "types/c/Cargo.toml"),
    ];
    let edges = vec![
        edge("types/a/Cargo.toml", "types/b/Cargo.toml", G3RsApparchDependencyKind::Dependency),
        edge("types/b/Cargo.toml", "types/c/Cargo.toml", G3RsApparchDependencyKind::Dependency),
        edge("types/c/Cargo.toml", "types/a/Cargo.toml", G3RsApparchDependencyKind::Dependency),
    ];
    let mut results = Vec::new();

    crate::rs_apparch_config_06_same_layer_cycles::check(&crates, &edges, &mut results);

    let result = results.first().expect("cycle result");
    assert_eq!(result.id(), "RS-APPARCH-CONFIG-06");
    assert_eq!(result.severity(), G3Severity::Error);
}

#[test]
fn dev_only_cycle_is_ignored() {
    let crates = vec![
        krate(G3RsApparchLayer::Types, "types/a/Cargo.toml"),
        krate(G3RsApparchLayer::Types, "types/b/Cargo.toml"),
    ];
    let edges = vec![
        edge("types/a/Cargo.toml", "types/b/Cargo.toml", G3RsApparchDependencyKind::DevDependency),
        edge("types/b/Cargo.toml", "types/a/Cargo.toml", G3RsApparchDependencyKind::TargetDevDependency),
    ];
    let mut results = Vec::new();

    crate::rs_apparch_config_06_same_layer_cycles::check(&crates, &edges, &mut results);

    assert!(results.is_empty());
}

#[test]
fn same_layer_self_loop_fires() {
    let crates = vec![krate(G3RsApparchLayer::Logic, "logic/service/Cargo.toml")];
    let edges = vec![edge(
        "logic/service/Cargo.toml",
        "logic/service/Cargo.toml",
        G3RsApparchDependencyKind::BuildDependency,
    )];
    let mut results = Vec::new();

    crate::rs_apparch_config_06_same_layer_cycles::check(&crates, &edges, &mut results);

    let result = results.first().expect("self loop result");
    assert_eq!(result.severity(), G3Severity::Error);
    assert!(result.message().contains("logic"));
}
