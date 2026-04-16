use g3rs_apparch_types::{
    G3RsApparchConfigChecksInput, G3RsApparchCrate, G3RsApparchDependencyKind,
    G3RsApparchExternalDependency, G3RsApparchLayer, G3RsApparchRustPolicyState,
};
use guardrail3_check_types::G3CheckResult;

fn types_crate() -> G3RsApparchCrate {
    G3RsApparchCrate {
        crate_name: "types-core".to_owned(),
        cargo_rel_path: "types/core/Cargo.toml".to_owned(),
        rel_dir: "types/core".to_owned(),
        layer: Some(G3RsApparchLayer::Types),
    }
}

pub(super) fn input(
    dep_name: &str,
    kind: G3RsApparchDependencyKind,
    rust_policy: G3RsApparchRustPolicyState,
) -> G3RsApparchConfigChecksInput {
    G3RsApparchConfigChecksInput {
        crates: vec![types_crate()],
        dependency_edges: Vec::new(),
        external_dependencies: vec![G3RsApparchExternalDependency {
            cargo_rel_path: "types/core/Cargo.toml".to_owned(),
            dep_name: dep_name.to_owned(),
            kind,
        }],
        patch_bypasses: Vec::new(),
        rust_policy,
    }
}

pub(super) fn run_rule(input: &G3RsApparchConfigChecksInput) -> Vec<G3CheckResult> {
    let mut results = Vec::new();
    let krate = input
        .crates
        .first()
        .expect("types purity test input should contain a source crate");

    crate::rs_apparch_config_08_types_purity::check(
        krate,
        &input.external_dependencies,
        &input.rust_policy,
        &mut results,
    );

    results
}
