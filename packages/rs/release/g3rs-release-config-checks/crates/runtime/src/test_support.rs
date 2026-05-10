#![cfg(test)]

use std::collections::BTreeSet;

use cargo_toml_parser::types::{CargoToml, InheritableValue, WorkspacePackageSection};
use g3rs_release_types::{
    G3RsReleaseConfigChecksInput, G3RsReleaseConfigCrate, G3RsReleaseConfigRepo,
};

pub(crate) fn config_input_for_crate(
    cargo_toml: &str,
    workspace_cargo_toml: Option<&str>,
) -> G3RsReleaseConfigChecksInput {
    let cargo = cargo_toml_parser::parse(cargo_toml).expect("cargo fixture should parse");
    let workspace_package = workspace_package(workspace_cargo_toml);

    G3RsReleaseConfigChecksInput {
        repos: Vec::new(),
        crates: vec![build_crate("Cargo.toml", &cargo, workspace_package)],
        edges: Vec::new(),
        input_failures: Vec::new(),
    }
}

pub(crate) fn config_input_for_publishable_crate(
    cargo_toml: &str,
    workspace_cargo_toml: Option<&str>,
) -> G3RsReleaseConfigChecksInput {
    let mut cargo = cargo_toml_parser::parse(cargo_toml).expect("cargo fixture should parse");
    if let Some(package) = cargo.package.as_mut()
        && package.publish.is_none()
    {
        package.publish = Some(InheritableValue::Value(
            cargo_toml_parser::types::VecStringOrBool::Bool(true),
        ));
    }

    G3RsReleaseConfigChecksInput {
        repos: Vec::new(),
        crates: vec![build_crate(
            "Cargo.toml",
            &cargo,
            workspace_package(workspace_cargo_toml),
        )],
        edges: Vec::new(),
        input_failures: Vec::new(),
    }
}

pub(crate) fn config_input_for_repo(
    release_plz_toml: Option<&str>,
    cliff_toml: Option<&str>,
) -> G3RsReleaseConfigChecksInput {
    let release_plz = release_plz_toml.map(|value| {
        release_plz_toml_parser::parse(value).expect("release-plz fixture should parse")
    });
    let cliff = cliff_toml
        .map(|value| cliff_toml_parser::parse(value).expect("cliff fixture should parse"));
    let publishable_name = release_plz
        .as_ref()
        .and_then(|release_plz| release_plz.package.first())
        .and_then(|package| package.name.as_deref())
        .unwrap_or("demo")
        .to_owned();

    G3RsReleaseConfigChecksInput {
        repos: vec![G3RsReleaseConfigRepo {
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
            release_plz_exists: release_plz.is_some(),
            release_plz,
            cliff_rel_path: "cliff.toml".to_owned(),
            cliff_exists: cliff.is_some(),
            cliff,
            workflows: Vec::new(),
            workflow_flags: g3rs_release_types::G3RsReleaseConfigRepoWorkflowFlags::default(),
            release_plz_workflow_rel_path: None,
            publish_dry_run_workflow_rel_path: None,
            registry_token_workflow_rel_path: None,
            semver_checks_installed: false,
        }],
        crates: vec![publishable_crate(&publishable_name)],
        edges: Vec::new(),
        input_failures: Vec::new(),
    }
}

fn workspace_package(workspace_cargo_toml: Option<&str>) -> Option<WorkspacePackageSection> {
    workspace_cargo_toml
        .map(|workspace| {
            cargo_toml_parser::parse(workspace).expect("workspace cargo fixture should parse")
        })
        .and_then(|workspace| workspace.workspace.and_then(|section| section.package))
}

fn publishable_crate(name: &str) -> G3RsReleaseConfigCrate {
    let cargo = cargo_toml_parser::parse(&format!(
        r#"
[package]
name = "{name}"
version = "0.1.0"
publish = true
"#
    ))
    .expect("publishable cargo fixture should parse");
    build_crate(&format!("crates/{name}/Cargo.toml"), &cargo, None)
}

fn build_crate(
    cargo_rel_path: &str,
    cargo: &CargoToml,
    workspace_package: Option<WorkspacePackageSection>,
) -> G3RsReleaseConfigCrate {
    let name = cargo
        .package
        .as_ref()
        .and_then(|package| package.name.clone())
        .unwrap_or_else(|| cargo_rel_path.to_owned());

    G3RsReleaseConfigCrate {
        name,
        cargo_rel_path: cargo_rel_path.to_owned(),
        cargo: cargo.clone(),
        workspace_package,
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
