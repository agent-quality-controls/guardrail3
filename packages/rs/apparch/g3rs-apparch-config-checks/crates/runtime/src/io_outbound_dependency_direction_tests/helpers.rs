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

pub(super) fn input(edges: &[(&str, &str)]) -> Vec<G3RsApparchCrateDependencyChecksInput> {
    let crates = [
        crate_input(
            "db",
            G3RsApparchLayer::IoOutbound,
            "io/outbound/db/Cargo.toml",
        ),
        crate_input(
            "cache",
            G3RsApparchLayer::IoOutbound,
            "io/outbound/cache/Cargo.toml",
        ),
        crate_input(
            "report-runtime",
            G3RsApparchLayer::IoOutbound,
            "io/outbound/report/crates/runtime/Cargo.toml",
        ),
        crate_input(
            "report-assertions",
            G3RsApparchLayer::IoOutbound,
            "io/outbound/report/crates/assertions/Cargo.toml",
        ),
        crate_input(
            "service",
            G3RsApparchLayer::Logic,
            "logic/service/Cargo.toml",
        ),
        crate_input(
            "http",
            G3RsApparchLayer::IoInbound,
            "io/inbound/http/Cargo.toml",
        ),
        crate_input("core", G3RsApparchLayer::Types, "types/core/Cargo.toml"),
    ];

    crates
        .iter()
        .map(|krate| G3RsApparchCrateDependencyChecksInput {
            krate: krate.clone(),
            internal_dependencies: edges
                .iter()
                .filter(|(from, _)| *from == krate.cargo_rel_path)
                .filter_map(|(_, to)| {
                    crates
                        .iter()
                        .find(|target| target.cargo_rel_path == *to)
                        .cloned()
                        .map(|target| G3RsApparchBoundDependency {
                            dep_name: target.crate_name.clone(),
                            kind: G3RsApparchDependencyKind::Dependency,
                            target,
                        })
                })
                .collect(),
        })
        .collect()
}

pub(super) fn run_rule(
    input: &[G3RsApparchCrateDependencyChecksInput],
    source_cargo_rel_path: &str,
) -> Vec<G3CheckResult> {
    let mut results = Vec::new();
    let crate_check = input
        .iter()
        .find(|check| check.krate.cargo_rel_path == source_cargo_rel_path)
        .expect("io/outbound test input should contain the requested source crate");

    crate::io_outbound_dependency_direction::check(crate_check, &mut results);

    results
}
