use g3rs_apparch_types::{
    G3RsApparchBoundDependency, G3RsApparchCrate, G3RsApparchCrateDependencyChecksInput,
    G3RsApparchDependencyKind, G3RsApparchLayer,
};
use guardrail3_check_types::G3CheckResult;

fn crate_input(name: &str, layer: G3RsApparchLayer, cargo_rel_path: &str) -> G3RsApparchCrate {
    G3RsApparchCrate {
        crate_name: name.to_owned(),
        cargo_rel_path: cargo_rel_path.to_owned(),
        rel_dir: cargo_rel_path.trim_end_matches("/Cargo.toml").to_owned(),
        layer: Some(layer),
    }
}

pub(super) fn input(edges: &[(&str, &str)]) -> G3RsApparchCrateDependencyChecksInput {
    let crates = [
        crate_input("core", G3RsApparchLayer::Types, "types/core/Cargo.toml"),
        crate_input("shared", G3RsApparchLayer::Types, "types/shared/Cargo.toml"),
        crate_input(
            "service",
            G3RsApparchLayer::Logic,
            "logic/service/Cargo.toml",
        ),
        crate_input(
            "db",
            G3RsApparchLayer::IoOutbound,
            "io/outbound/db/Cargo.toml",
        ),
    ];

    G3RsApparchCrateDependencyChecksInput {
        krate: crates
            .first()
            .expect("types test input should contain a source crate")
            .clone(),
        internal_dependencies: edges
            .iter()
            .filter(|(from, _)| *from == "types/core/Cargo.toml")
            .filter_map(|(_, to)| {
                crates
                    .iter()
                    .find(|krate| krate.cargo_rel_path == *to)
                    .cloned()
                    .map(|target| G3RsApparchBoundDependency {
                        dep_name: target.crate_name.clone(),
                        kind: G3RsApparchDependencyKind::Dependency,
                        target,
                    })
            })
            .collect(),
    }
}

pub(super) fn run_rule(input: &G3RsApparchCrateDependencyChecksInput) -> Vec<G3CheckResult> {
    let mut results = Vec::new();
    crate::types_dependency_direction::check(input, &mut results);
    results
}
