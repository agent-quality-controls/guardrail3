#![cfg(test)]

use std::collections::BTreeSet;

use cargo_toml_parser::{
    types::CargoToml,
    types::InheritableValue,
    types::PackageSection,
    types::VecStringOrBool,
    types::WorkspacePackageSection,
};
use g3rs_release_types::{
    G3RsReleaseConfigChecksInput, G3RsReleaseConfigCrate, G3RsReleaseConfigRepo,
};

pub(crate) fn config_input_for_crate(
    cargo_toml: &str,
    workspace_cargo_toml: Option<&str>,
) -> G3RsReleaseConfigChecksInput {
    let cargo = cargo_toml_parser::parse(cargo_toml).expect("cargo fixture should parse");
    let workspace_package = workspace_cargo_toml
        .map(|workspace| {
            cargo_toml_parser::parse(workspace).expect("workspace cargo fixture should parse")
        })
        .and_then(|workspace| workspace.workspace.and_then(|section| section.package));
    let krate = build_crate("Cargo.toml", cargo, workspace_package);

    G3RsReleaseConfigChecksInput {
        repo: None,
        crates: vec![krate],
        edges: Vec::new(),
        input_failures: Vec::new(),
    }
}

pub(crate) fn config_input_for_publishable_crate(
    cargo_toml: &str,
    workspace_cargo_toml: Option<&str>,
) -> G3RsReleaseConfigChecksInput {
    let mut cargo = cargo_toml_parser::parse(cargo_toml).expect("cargo fixture should parse");
    if let Some(package) = cargo.package.as_mut() {
        if package.publish.is_none() {
            package.publish = Some(InheritableValue::Value(VecStringOrBool::Bool(true)));
        }
    }
    let workspace_package = workspace_cargo_toml
        .map(|workspace| {
            cargo_toml_parser::parse(workspace).expect("workspace cargo fixture should parse")
        })
        .and_then(|workspace| workspace.workspace.and_then(|section| section.package));
    let krate = build_crate("Cargo.toml", cargo, workspace_package);

    G3RsReleaseConfigChecksInput {
        repo: None,
        crates: vec![krate],
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

    G3RsReleaseConfigChecksInput {
        repo: Some(G3RsReleaseConfigRepo {
            cargo_rel_path: "Cargo.toml".to_owned(),
            release_plz_rel_path: "release-plz.toml".to_owned(),
            release_plz_exists: release_plz.is_some(),
            release_plz,
            release_plz_package_names: BTreeSet::new(),
            cliff_rel_path: "cliff.toml".to_owned(),
            cliff_exists: cliff.is_some(),
            cliff,
            has_release_plz_workflow: false,
            release_plz_workflow_rel_path: None,
            has_publish_dry_run_workflow: false,
            publish_dry_run_workflow_rel_path: None,
            has_registry_token_workflow: false,
            registry_token_workflow_rel_path: None,
            publishable_crate_names: BTreeSet::new(),
            publishable_binary_crate_names: BTreeSet::new(),
            publishable_count: 1,
            non_publishable_count: 0,
            semver_checks_installed: false,
            publish_setting: None,
            release_profile_settings: Vec::new(),
        }),
        crates: Vec::new(),
        edges: Vec::new(),
        input_failures: Vec::new(),
    }
}

fn build_crate(
    cargo_rel_path: &str,
    cargo: CargoToml,
    workspace_package: Option<WorkspacePackageSection>,
) -> G3RsReleaseConfigCrate {
    let package = cargo.package.clone();
    let package_ref = package.as_ref();
    let name = package_ref
        .and_then(|pkg| pkg.name.clone())
        .unwrap_or_else(|| cargo_rel_path.to_owned());
    let publish_declared = publish_declared(package_ref);
    let publishable = publishable(package_ref, workspace_package.as_ref());
    let version_string = version_string(package_ref, workspace_package.as_ref());

    G3RsReleaseConfigCrate {
        name,
        cargo_rel_path: cargo_rel_path.to_owned(),
        publish_declared,
        is_binary: !cargo.bin.is_empty(),
        is_library: cargo.lib.is_some(),
        binary_target_names: BTreeSet::new(),
        description_present: inherited_string_present(
            package_ref.and_then(|pkg| pkg.description.as_ref()),
            workspace_package
                .as_ref()
                .and_then(|ws| ws.description.as_deref()),
        ),
        license_present: inherited_string_present(
            package_ref.and_then(|pkg| pkg.license.as_ref()),
            workspace_package
                .as_ref()
                .and_then(|ws| ws.license.as_deref()),
        ) || inherited_string_present(
            package_ref.and_then(|pkg| pkg.license_file.as_ref()),
            workspace_package
                .as_ref()
                .and_then(|ws| ws.license_file.as_deref()),
        ),
        repository_present: inherited_string_present(
            package_ref.and_then(|pkg| pkg.repository.as_ref()),
            workspace_package
                .as_ref()
                .and_then(|ws| ws.repository.as_deref()),
        ),
        keywords_count: inherited_vec_count(
            package_ref.and_then(|pkg| pkg.keywords.as_ref()),
            workspace_package.as_ref().map(|ws| ws.keywords.as_slice()),
        ),
        categories_count: inherited_vec_count(
            package_ref.and_then(|pkg| pkg.categories.as_ref()),
            workspace_package
                .as_ref()
                .map(|ws| ws.categories.as_slice()),
        ),
        version_valid: version_string
            .as_deref()
            .is_some_and(|version| semver::Version::parse(version).is_ok()),
        docs_rs_present: package_ref
            .and_then(|pkg| pkg.metadata.as_ref())
            .and_then(|metadata| {
                metadata
                    .get("docs.rs")
                    .or_else(|| metadata.get("docs").and_then(|docs| docs.get("rs")))
            })
            .and_then(|value| value.as_table())
            .is_some_and(|table| {
                [
                    "all-features",
                    "features",
                    "no-default-features",
                    "default-target",
                    "targets",
                    "rustdoc-args",
                    "cargo-args",
                ]
                .iter()
                .any(|key| table.contains_key(*key))
            }),
        include_exclude_present: package_ref.is_some_and(|pkg| {
            pkg.include.as_ref().is_some_and(
                |value| matches!(value, InheritableValue::Value(values) if !values.is_empty()),
            ) || pkg.exclude.as_ref().is_some_and(
                |value| matches!(value, InheritableValue::Value(values) if !values.is_empty()),
            )
        }),
        has_binstall_metadata: package_ref
            .and_then(|pkg| pkg.metadata.as_ref())
            .and_then(|metadata| metadata.get("binstall"))
            .and_then(|value| value.as_table())
            .is_some(),
        publishable,
        workspace_package,
        cargo,
        workspace_version: matches!(
            package_ref.and_then(|pkg| pkg.version.as_ref()),
            Some(InheritableValue::Inherit(_))
        ),
        version_string,
        binary_release_workflow_present: false,
        linux_release_target_present: false,
        dry_run: None,
    }
}

fn publish_declared(package: Option<&PackageSection>) -> bool {
    package
        .and_then(|package| package.publish.as_ref())
        .is_some()
}

fn publishable(
    package: Option<&PackageSection>,
    workspace_package: Option<&WorkspacePackageSection>,
) -> bool {
    let Some(package) = package else {
        return false;
    };
    match package.publish.as_ref() {
        None => false,
        Some(InheritableValue::Value(VecStringOrBool::Bool(false))) => false,
        Some(InheritableValue::Value(VecStringOrBool::VecString(values))) => !values.is_empty(),
        Some(InheritableValue::Value(VecStringOrBool::Bool(true))) => true,
        Some(InheritableValue::Inherit(_)) => {
            match workspace_package.and_then(|ws| ws.publish.as_ref()) {
                None => false,
                Some(VecStringOrBool::Bool(false)) => false,
                Some(VecStringOrBool::VecString(values)) => !values.is_empty(),
                Some(VecStringOrBool::Bool(true)) => true,
            }
        }
    }
}

fn inherited_string_present(
    value: Option<&InheritableValue<String>>,
    workspace_value: Option<&str>,
) -> bool {
    match value {
        Some(InheritableValue::Value(value)) => !value.trim().is_empty(),
        Some(InheritableValue::Inherit(_)) => {
            workspace_value.is_some_and(|value| !value.trim().is_empty())
        }
        None => false,
    }
}

fn inherited_vec_count(
    value: Option<&InheritableValue<Vec<String>>>,
    workspace_values: Option<&[String]>,
) -> Option<usize> {
    match value {
        Some(InheritableValue::Value(values)) => Some(values.len()),
        Some(InheritableValue::Inherit(_)) => workspace_values.map(|values| values.len()),
        None => None,
    }
}

fn version_string(
    package: Option<&PackageSection>,
    workspace_package: Option<&WorkspacePackageSection>,
) -> Option<String> {
    match package.and_then(|pkg| pkg.version.as_ref()) {
        Some(InheritableValue::Value(value)) => Some(value.clone()),
        Some(InheritableValue::Inherit(_)) => workspace_package.and_then(|ws| ws.version.clone()),
        None => None,
    }
}
