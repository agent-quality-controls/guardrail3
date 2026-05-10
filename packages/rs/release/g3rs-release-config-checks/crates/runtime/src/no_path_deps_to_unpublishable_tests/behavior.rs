use std::collections::BTreeSet;

use cargo_toml_parser::types::CargoToml;
use g3rs_release_config_checks_assertions::no_path_deps_to_unpublishable as assertions;
use g3rs_release_types::{
    G3RsReleaseConfigCrate, G3RsReleaseConfigEdge, G3RsReleasePathTargetKind,
};

fn publishable_crate() -> G3RsReleaseConfigCrate {
    let cargo = cargo_toml_parser::parse(
        r#"
[package]
name = "crate-a"
version = "0.1.0"
publish = true
"#,
    )
    .expect("publishable crate should parse");
    build_crate("Cargo.toml", &cargo)
}

fn non_publishable_crate() -> G3RsReleaseConfigCrate {
    let cargo = cargo_toml_parser::parse(
        r#"
[package]
name = "dep-a"
version = "0.1.0"
publish = false
"#,
    )
    .expect("non-publishable crate should parse");
    build_crate("Cargo.toml", &cargo)
}

fn edge() -> G3RsReleaseConfigEdge {
    G3RsReleaseConfigEdge {
        source: publishable_crate(),
        dep_name: "dep-a".to_owned(),
        dep_package_name: "dep-a".to_owned(),
        section_label: "dependencies".to_owned(),
        target_label: None,
        has_path: true,
        path_target_kind: Some(G3RsReleasePathTargetKind::InWorkspace),
        version_req: None,
        target: Some(non_publishable_crate()),
    }
}

#[test]
fn errors_for_local_path_dep_to_non_publishable_crate() {
    let mut results = Vec::new();

    super::super::check(&edge(), &mut results);

    assertions::assert_findings(
        &results,
        &[assertions::error(
            "crate-a: path dep to non-publishable crate",
            "Dependency `dep-a` in `[dependencies]` points at a crate inside this workspace that is not publishable. Make the target crate publishable or stop depending on it from a publishable crate.",
            "Cargo.toml",
            false,
        )],
    );
}

#[test]
fn stands_down_for_external_path_dep_with_version_requirement() {
    let mut edge = edge();
    edge.path_target_kind = None;
    edge.version_req = Some("^0.1.0".to_owned());
    edge.target = None;
    let mut results = Vec::new();

    super::super::check(&edge, &mut results);

    assertions::assert_no_findings(&results);
}

#[test]
fn errors_for_local_unpublishable_path_dep_even_with_version_requirement() {
    let mut edge = edge();
    edge.version_req = Some("^0.1.0".to_owned());
    edge.target = None;
    let mut results = Vec::new();

    super::super::check(&edge, &mut results);

    assertions::assert_findings(
        &results,
        &[assertions::error(
            "crate-a: path dep to non-publishable crate",
            "Dependency `dep-a` in `[dependencies]` points at a crate inside this workspace that is not publishable. Make the target crate publishable or stop depending on it from a publishable crate.",
            "Cargo.toml",
            false,
        )],
    );
}

#[test]
fn warns_for_outside_workspace_path_dep_with_version_requirement() {
    let mut edge = edge();
    edge.path_target_kind = Some(G3RsReleasePathTargetKind::OutsideWorkspace);
    edge.version_req = Some("^0.1.0".to_owned());
    edge.target = None;
    let mut results = Vec::new();

    super::super::check(&edge, &mut results);

    assertions::assert_findings(
        &results,
        &[assertions::warn(
            "crate-a: path dep escapes workspace",
            "Dependency `dep-a` in `[dependencies]` points outside this workspace by path. Replace it with a normal versioned dependency if this crate is meant to publish.",
            "Cargo.toml",
            false,
        )],
    );
}

#[test]
fn skips_non_publishable_source_crate() {
    let mut edge = edge();
    edge.source = non_publishable_crate();
    let mut results = Vec::new();

    super::super::check(&edge, &mut results);

    assertions::assert_no_findings(&results);
}

fn build_crate(cargo_rel_path: &str, cargo: &CargoToml) -> G3RsReleaseConfigCrate {
    let name = cargo
        .package
        .as_ref()
        .and_then(|package| package.name.clone())
        .unwrap_or_else(|| cargo_rel_path.to_owned());

    G3RsReleaseConfigCrate {
        name,
        cargo_rel_path: cargo_rel_path.to_owned(),
        cargo: cargo.clone(),
        workspace_package: None,
        is_binary: !cargo.bin.is_empty(),
        is_library: cargo.lib.is_some(),
        binary_target_names: cargo
            .bin
            .iter()
            .filter_map(|target| target.name.clone())
            .collect::<BTreeSet<_>>(),
        dry_run: None,
    }
}
