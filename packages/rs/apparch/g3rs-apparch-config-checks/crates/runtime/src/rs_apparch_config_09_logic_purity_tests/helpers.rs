use g3rs_apparch_types::{
    G3RsApparchConfigChecksInput, G3RsApparchCrate, G3RsApparchCratePurityChecksInput,
    G3RsApparchDependencyKind, G3RsApparchExternalDependency, G3RsApparchLayer,
    G3RsApparchPatchBypassChecksInput, G3RsApparchRustPolicyState,
    G3RsApparchSameLayerCyclesChecksInput,
};
use guardrail3_check_types::G3CheckResult;

fn logic_crate() -> G3RsApparchCrate {
    G3RsApparchCrate {
        crate_name: "logic-service".to_owned(),
        cargo_rel_path: "logic/service/Cargo.toml".to_owned(),
        rel_dir: "logic/service".to_owned(),
        layer: Some(G3RsApparchLayer::Logic),
    }
}

pub(super) fn input(
    dep_name: &str,
    kind: G3RsApparchDependencyKind,
    rust_policy: G3RsApparchRustPolicyState,
) -> G3RsApparchConfigChecksInput {
    G3RsApparchConfigChecksInput {
        crate_dependency_checks: Vec::new(),
        crate_purity_checks: vec![G3RsApparchCratePurityChecksInput {
            krate: logic_crate(),
            external_dependencies: vec![G3RsApparchExternalDependency {
                cargo_rel_path: "logic/service/Cargo.toml".to_owned(),
                dep_name: dep_name.to_owned(),
                kind,
            }],
            rust_policy,
        }],
        patch_bypass_checks: Vec::<G3RsApparchPatchBypassChecksInput>::new(),
        same_layer_cycles_check: G3RsApparchSameLayerCyclesChecksInput { edges: Vec::new() },
    }
}

pub(super) fn run_rule(input: &G3RsApparchConfigChecksInput) -> Vec<G3CheckResult> {
    let mut results = Vec::new();
    let purity_check = input
        .crate_purity_checks
        .first()
        .expect("logic purity test input should contain a source crate");

    crate::rs_apparch_config_09_logic_purity::check(purity_check, &mut results);

    results
}
