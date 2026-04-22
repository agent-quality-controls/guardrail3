use std::collections::BTreeSet;

use cargo_toml_parser::types::CargoToml;
use g3rs_release_config_checks_assertions::run as assertions;
use g3rs_release_types::{
    G3RsReleaseConfigChecksInput, G3RsReleaseConfigCrate, G3RsReleaseConfigRepo,
};

#[test]
fn skips_repo_level_release_setup_when_nothing_publishes() {
    let mut input = config_input_for_repo(
        Some(
            r#"
[[package]]
name = "some-crate"
"#,
        ),
        Some("# empty cliff.toml\n"),
    );
    input.crate_checks = config_input_for_crate(
        r#"
[package]
name = "demo"
version = "0.1.0"
publish = false
"#,
    )
    .crate_checks;

    let results = super::super::check(&input);

    assertions::assert_no_findings(&results);
}

fn config_input_for_repo(
    release_plz_toml: Option<&str>,
    cliff_toml: Option<&str>,
) -> G3RsReleaseConfigChecksInput {
    G3RsReleaseConfigChecksInput {
        repo_checks: vec![G3RsReleaseConfigRepo {
            cargo_rel_path: "Cargo.toml".to_owned(),
            cargo: cargo_toml_parser::parse(
                r#"
[workspace]
members = ["crates/demo"]
resolver = "2"
"#,
            )
            .expect("repo cargo fixture should parse"),
            release_plz_rel_path: "release-plz.toml".to_owned(),
            release_plz_exists: release_plz_toml.is_some(),
            release_plz: release_plz_toml.map(|value| {
                release_plz_toml_parser::parse(value).expect("release-plz fixture should parse")
            }),
            cliff_rel_path: "cliff.toml".to_owned(),
            cliff_exists: cliff_toml.is_some(),
            cliff: cliff_toml
                .map(|value| cliff_toml_parser::parse(value).expect("cliff fixture should parse")),
            workflows: Vec::new(),
            semver_checks_installed: false,
        }],
        crate_checks: vec![publishable_crate("demo")],
        edge_checks: Vec::new(),
        input_failure_checks: Vec::new(),
    }
}

fn config_input_for_crate(cargo_toml: &str) -> G3RsReleaseConfigChecksInput {
    G3RsReleaseConfigChecksInput {
        repo_checks: Vec::new(),
        crate_checks: vec![build_crate(
            "Cargo.toml",
            cargo_toml_parser::parse(cargo_toml).expect("cargo fixture should parse"),
        )],
        edge_checks: Vec::new(),
        input_failure_checks: Vec::new(),
    }
}

fn publishable_crate(name: &str) -> G3RsReleaseConfigCrate {
    build_crate(
        &format!("crates/{name}/Cargo.toml"),
        cargo_toml_parser::parse(&format!(
            r#"
[package]
name = "{name}"
version = "0.1.0"
publish = true
"#
        ))
        .expect("publishable cargo fixture should parse"),
    )
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
