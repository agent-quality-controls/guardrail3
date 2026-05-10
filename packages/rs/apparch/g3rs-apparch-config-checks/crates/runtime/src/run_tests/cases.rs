use g3rs_apparch_config_checks_assertions::run as assertions;
use g3rs_apparch_types::{
    G3RsApparchBoundDependency, G3RsApparchConfigChecksInput, G3RsApparchCrate,
    G3RsApparchCrateDependencyChecksInput, G3RsApparchCratePurityChecksInput,
    G3RsApparchDependencyKind, G3RsApparchExternalDependency, G3RsApparchLayer,
    G3RsApparchPatchBypassChecksInput, G3RsApparchRustPolicyState,
    G3RsApparchSameLayerCyclesChecksInput,
};

use super::helpers::input;

#[test]
fn io_inbound_has_no_dependency_direction_rule() {
    let results = crate::run::check(&input());

    assertions::assert_no_finding_for_file(&results, "io/inbound/http/Cargo.toml");
}

#[test]
fn run_dispatches_prebound_dependency_and_purity_inputs() {
    let types = fixture_crate("types/core", G3RsApparchLayer::Types);
    let logic = fixture_crate("logic/service", G3RsApparchLayer::Logic);
    let input = G3RsApparchConfigChecksInput {
        crate_dependency_checks: vec![G3RsApparchCrateDependencyChecksInput {
            krate: types.clone(),
            internal_dependencies: vec![G3RsApparchBoundDependency {
                dep_name: logic.crate_name.clone(),
                kind: G3RsApparchDependencyKind::Dependency,
                target: logic,
            }],
        }],
        crate_purity_checks: vec![G3RsApparchCratePurityChecksInput {
            krate: types.clone(),
            external_dependencies: vec![G3RsApparchExternalDependency {
                cargo_rel_path: types.cargo_rel_path,
                dep_name: "sqlx".to_owned(),
                kind: G3RsApparchDependencyKind::Dependency,
            }],
            rust_policy: G3RsApparchRustPolicyState::Missing,
        }],
        patch_bypass_checks: Vec::<G3RsApparchPatchBypassChecksInput>::new(),
        same_layer_cycles_check: G3RsApparchSameLayerCyclesChecksInput { edges: Vec::new() },
    };

    let results = crate::run::check(&input);

    assertions::assert_has_finding_id(&results, "g3rs-apparch/types-dependency-direction");
    assertions::assert_has_finding_id(&results, "g3rs-apparch/types-purity");
}

#[test]
fn run_dispatches_prebound_same_layer_cycle_nodes() {
    let types_a = fixture_crate("types/a", G3RsApparchLayer::Types);
    let types_b = fixture_crate("types/b", G3RsApparchLayer::Types);
    let input = G3RsApparchConfigChecksInput {
        crate_dependency_checks: Vec::new(),
        crate_purity_checks: Vec::new(),
        patch_bypass_checks: Vec::new(),
        same_layer_cycles_check: G3RsApparchSameLayerCyclesChecksInput {
            edges: vec![
                g3rs_apparch_types::G3RsApparchSameLayerDependencyEdge {
                    from: types_a.clone(),
                    to: types_b.clone(),
                },
                g3rs_apparch_types::G3RsApparchSameLayerDependencyEdge {
                    from: types_b,
                    to: types_a,
                },
            ],
        },
    };

    let results = crate::run::check(&input);

    assertions::assert_has_finding_id(&results, "g3rs-apparch/same-layer-cycles");
}

fn fixture_crate(rel_dir: &str, layer: G3RsApparchLayer) -> G3RsApparchCrate {
    G3RsApparchCrate {
        crate_name: rel_dir
            .rsplit('/')
            .next()
            .expect("fixture crate path should end with a crate name")
            .to_owned(),
        cargo_rel_path: format!("{rel_dir}/Cargo.toml"),
        rel_dir: rel_dir.to_owned(),
        layer: Some(layer),
    }
}
