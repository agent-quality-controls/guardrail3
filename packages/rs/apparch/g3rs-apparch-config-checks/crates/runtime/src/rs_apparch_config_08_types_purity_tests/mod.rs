use g3rs_apparch_types::{
    G3RsApparchConfigChecksInput, G3RsApparchCrate, G3RsApparchExternalDependency,
    G3RsApparchDependencyKind, G3RsApparchLayer, G3RsApparchRustPolicyState,
};
use guardrail3_check_types::G3Severity;

fn types_crate() -> G3RsApparchCrate {
    G3RsApparchCrate {
        crate_name: "types-core".to_owned(),
        cargo_rel_path: "types/core/Cargo.toml".to_owned(),
        rel_dir: "types/core".to_owned(),
        layer: Some(G3RsApparchLayer::Types),
    }
}

fn input(
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

#[test]
fn impure_external_dependency_fires() {
    let results = crate::check(&input(
        "sqlx",
        G3RsApparchDependencyKind::BuildDependency,
        G3RsApparchRustPolicyState::Missing,
    ));
    let result = results
        .iter()
        .find(|result| result.id() == "RS-APPARCH-CONFIG-08")
        .expect("types purity error");

    assert_eq!(result.severity(), G3Severity::Error);
}

#[test]
fn built_in_allowed_dependency_emits_inventory() {
    let results = crate::check(&input(
        "serde",
        G3RsApparchDependencyKind::Dependency,
        G3RsApparchRustPolicyState::Missing,
    ));
    let result = results
        .iter()
        .find(|result| result.id() == "RS-APPARCH-CONFIG-08")
        .expect("types purity inventory");

    assert_eq!(result.severity(), G3Severity::Info);
    assert!(result.inventory());
}

#[test]
fn policy_allowlist_allows_extra_dependency() {
    let results = crate::check(&input(
        "reqwest",
        G3RsApparchDependencyKind::Dependency,
        G3RsApparchRustPolicyState::Parsed {
            rel_path: "guardrail3-rs.toml".to_owned(),
            profile: None,
            allowed_deps: vec!["reqwest".to_owned()],
            waivers: Vec::new(),
        },
    ));

    let result = results
        .iter()
        .find(|result| result.id() == "RS-APPARCH-CONFIG-08")
        .expect("types purity allowlist inventory");
    assert_eq!(result.severity(), G3Severity::Info);
}

#[test]
fn invalid_policy_fires_instead_of_dropping_to_empty_allowlist() {
    let results = crate::check(&input(
        "reqwest",
        G3RsApparchDependencyKind::Dependency,
        G3RsApparchRustPolicyState::ParseError {
            rel_path: "guardrail3-rs.toml".to_owned(),
            reason: "bad toml".to_owned(),
        },
    ));
    let result = results
        .iter()
        .find(|result| result.id() == "RS-APPARCH-CONFIG-08")
        .expect("types purity parse error");

    assert_eq!(result.severity(), G3Severity::Error);
    assert!(result.message().contains("bad toml"));
}
