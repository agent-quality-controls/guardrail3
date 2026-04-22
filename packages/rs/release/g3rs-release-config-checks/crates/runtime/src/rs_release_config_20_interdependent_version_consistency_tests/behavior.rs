use std::collections::BTreeSet;

use cargo_toml_parser::types::CargoToml;
use g3rs_release_config_checks_assertions::rs_release_config_20_interdependent_version_consistency as assertions;
use g3rs_release_types::G3RsReleaseConfigCrate;
use g3rs_release_types::G3RsReleaseConfigEdge;

fn publishable_crate(name: &str, version: &str) -> G3RsReleaseConfigCrate {
    build_crate(
        "Cargo.toml",
        cargo_toml_parser::parse(&format!(
            r#"
[package]
name = "{name}"
version = "{version}"
publish = true
"#
        ))
        .expect("publishable crate should parse"),
    )
}

fn edge() -> G3RsReleaseConfigEdge {
    G3RsReleaseConfigEdge {
        source: publishable_crate("crate-a", "0.1.0"),
        dep_name: "dep-a".to_owned(),
        dep_package_name: "dep-a".to_owned(),
        section_label: "dependencies".to_owned(),
        target_label: None,
        has_path: true,
        path_target_kind: None,
        version_req: Some("^0.2.0".to_owned()),
        target: Some(publishable_crate("dep-a", "0.1.0")),
    }
}

#[test]
fn skips_non_publishable_source_crate() {
    let mut edge = edge();
    edge.source = build_crate(
        "Cargo.toml",
        cargo_toml_parser::parse(
            r#"
[package]
name = "crate-a"
version = "0.1.0"
publish = false
"#,
        )
        .expect("crate should parse"),
    );
    let mut results = Vec::new();

    super::super::check(&edge, &mut results);

    assertions::assert_no_findings(&results);
}

fn build_crate(cargo_rel_path: &str, cargo: CargoToml) -> G3RsReleaseConfigCrate {
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
