#![cfg(test)]

use std::collections::BTreeSet;

use cargo_toml_parser::types::{CargoToml, WorkspacePackageSection};
use g3rs_release_types::{G3RsReleaseConfigChecksInput, G3RsReleaseConfigCrate};

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

fn workspace_package(workspace_cargo_toml: Option<&str>) -> Option<WorkspacePackageSection> {
    workspace_cargo_toml
        .map(|workspace| {
            cargo_toml_parser::parse(workspace).expect("workspace cargo fixture should parse")
        })
        .and_then(|workspace| workspace.workspace.and_then(|section| section.package))
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
