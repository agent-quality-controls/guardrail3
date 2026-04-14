use g3rs_apparch_types::{
    G3RsApparchConfigChecksInput, G3RsApparchCrate, G3RsApparchExternalDependency,
    G3RsApparchDependencyKind, G3RsApparchLayer, G3RsApparchRustPolicyState,
};
use guardrail3_check_types::G3Severity;

fn logic_crate() -> G3RsApparchCrate {
    G3RsApparchCrate {
        crate_name: "logic-service".to_owned(),
        cargo_rel_path: "logic/service/Cargo.toml".to_owned(),
        rel_dir: "logic/service".to_owned(),
        layer: Some(G3RsApparchLayer::Logic),
    }
}

fn input(
    dep_name: &str,
    kind: G3RsApparchDependencyKind,
    rust_policy: G3RsApparchRustPolicyState,
) -> G3RsApparchConfigChecksInput {
    G3RsApparchConfigChecksInput {
        crates: vec![logic_crate()],
        dependency_edges: Vec::new(),
        external_dependencies: vec![G3RsApparchExternalDependency {
            cargo_rel_path: "logic/service/Cargo.toml".to_owned(),
            dep_name: dep_name.to_owned(),
            kind,
        }],
        patch_bypasses: Vec::new(),
        rust_policy,
    }
}

#[test]
fn impure_external_dependency_fires() {
    let results = crate::check(&input(
        "reqwest",
        G3RsApparchDependencyKind::Dependency,
        G3RsApparchRustPolicyState::Missing,
    ));
    let result = results
        .iter()
        .find(|result| result.id() == "RS-APPARCH-CONFIG-09")
        .expect("logic purity error");

    assert_eq!(result.severity(), G3Severity::Error);
}

#[test]
fn built_in_allowed_dependency_emits_inventory() {
    let results = crate::check(&input(
        "serde_json",
        G3RsApparchDependencyKind::BuildDependency,
        G3RsApparchRustPolicyState::Missing,
    ));
    let result = results
        .iter()
        .find(|result| result.id() == "RS-APPARCH-CONFIG-09")
        .expect("logic purity inventory");

    assert_eq!(result.severity(), G3Severity::Info);
    assert!(result.inventory());
}

#[test]
fn invalid_policy_fires_instead_of_dropping_to_empty_allowlist() {
    let results = crate::check(&input(
        "reqwest",
        G3RsApparchDependencyKind::Dependency,
        G3RsApparchRustPolicyState::Unreadable {
            rel_path: "guardrail3-rs.toml".to_owned(),
            reason: "permission denied".to_owned(),
        },
    ));
    let result = results
        .iter()
        .find(|result| result.id() == "RS-APPARCH-CONFIG-09")
        .expect("logic purity unreadable policy");

    assert_eq!(result.severity(), G3Severity::Error);
    assert!(result.message().contains("permission denied"));
}
