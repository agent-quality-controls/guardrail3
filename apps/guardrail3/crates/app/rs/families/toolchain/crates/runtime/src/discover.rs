use std::collections::{BTreeMap, BTreeSet};

use guardrail3_app_rs_family_mapper::RsToolchainRoute;
use guardrail3_app_rs_ownership::RustFamilyFileKind;
use guardrail3_app_rs_family_mapper::RsProjectSurface as ProjectTree;

use super::facts::{ToolchainFamilyFacts, ToolchainPolicyRootFacts};

#[derive(Debug, Clone)]
struct CargoSnapshot {
    rel_dir: String,
    cargo_rel_path: String,
    parse_error: Option<String>,
    has_workspace: bool,
    rust_version: Option<String>,
    rust_version_invalid: bool,
}

pub fn collect(tree: &ProjectTree, route: &RsToolchainRoute) -> ToolchainFamilyFacts {
    let snapshots = collect_cargo_snapshots(tree, route);
    let workspace_roots: Vec<_> = snapshots
        .values()
        .filter(|snapshot| snapshot.has_workspace)
        .map(|snapshot| snapshot.rel_dir.clone())
        .collect();
    let mut policy_roots = Vec::new();

    for workspace_root_rel in &workspace_roots {
        let Some(snapshot) = snapshots.get(workspace_root_rel) else {
            continue;
        };
        policy_roots.push(build_policy_root(tree, route, snapshot));
    }

    policy_roots.sort_by(|a, b| a.cargo_rel_path.cmp(&b.cargo_rel_path));

    ToolchainFamilyFacts { policy_roots }
}

fn collect_cargo_snapshots(
    tree: &ProjectTree,
    route: &RsToolchainRoute,
) -> BTreeMap<String, CargoSnapshot> {
    let mut rel_dirs = route
        .roots()
        .iter()
        .map(|root| root.rel_dir().to_owned())
        .collect::<BTreeSet<_>>();
    rel_dirs.extend(route.family_files().iter().filter_map(|file| {
        (file.kind() == RustFamilyFileKind::CargoToml
            && (file.nearest_rust_root_rel().is_some() || file.ancestor_rust_root_rels().is_some()))
        .then(|| file.logical_owner_rel().to_owned())
    }));

    rel_dirs
        .into_iter()
        .map(|rel_dir| {
            let cargo_rel_path = route
                .family_files()
                .iter()
                .find(|file| {
                    file.kind() == RustFamilyFileKind::CargoToml
                        && file.logical_owner_rel() == rel_dir
                })
                .map(|file| file.rel_path().to_owned())
                .unwrap_or_else(|| {
                    if rel_dir.is_empty() {
                        "Cargo.toml".to_owned()
                    } else {
                        ProjectTree::join_rel(&rel_dir, "Cargo.toml")
                    }
                });
            let snapshot = cargo_snapshot(tree, &rel_dir, &cargo_rel_path);
            (snapshot.rel_dir.clone(), snapshot)
        })
        .collect()
}

fn cargo_snapshot(tree: &ProjectTree, rel_dir: &str, cargo_rel_path: &str) -> CargoSnapshot {
    let Some(content) = tree.file_content(cargo_rel_path) else {
        return CargoSnapshot {
            rel_dir: rel_dir.to_owned(),
            cargo_rel_path: cargo_rel_path.to_owned(),
            parse_error: Some("Cargo.toml content missing from ProjectTree".to_owned()),
            has_workspace: false,
            rust_version: None,
            rust_version_invalid: false,
        };
    };

    match toml::from_str::<toml::Value>(content) {
        Ok(parsed) => {
            let has_workspace = parsed.get("workspace").is_some();
            let rust_version = extract_rust_version(&parsed, has_workspace);
            CargoSnapshot {
                rel_dir: rel_dir.to_owned(),
                cargo_rel_path: cargo_rel_path.to_owned(),
                parse_error: None,
                has_workspace,
                rust_version: rust_version.value,
                rust_version_invalid: rust_version.invalid,
            }
        }
        Err(parse_error) => CargoSnapshot {
            rel_dir: rel_dir.to_owned(),
            cargo_rel_path: cargo_rel_path.to_owned(),
            parse_error: Some(parse_error.to_string()),
            has_workspace: false,
            rust_version: None,
            rust_version_invalid: false,
        },
    }
}

fn build_policy_root(
    tree: &ProjectTree,
    route: &RsToolchainRoute,
    snapshot: &CargoSnapshot,
) -> ToolchainPolicyRootFacts {
    let modern_toolchain_rel = route.family_files().iter().find_map(|file| {
        (file.logical_owner_rel() == snapshot.rel_dir
            && file.exact_rust_root_owner()
            && file.kind() == RustFamilyFileKind::RustToolchainToml)
            .then(|| file.rel_path().to_owned())
    });
    let legacy_toolchain_rel = route.family_files().iter().find_map(|file| {
        (file.logical_owner_rel() == snapshot.rel_dir
            && file.exact_rust_root_owner()
            && file.kind() == RustFamilyFileKind::RustToolchainLegacy)
            .then(|| file.rel_path().to_owned())
    });

    let modern_toolchain = match modern_toolchain_rel.as_deref() {
        Some(toolchain_toml_rel) => match tree.file_content(toolchain_toml_rel) {
            Some(content) => match toml::from_str::<toml::Value>(content) {
                Ok(parsed) => (Some(toolchain_toml_rel.to_owned()), Some(parsed), None),
                Err(parse_error) => (
                    Some(toolchain_toml_rel.to_owned()),
                    None,
                    Some(parse_error.to_string()),
                ),
            },
            None => (
                Some(toolchain_toml_rel.to_owned()),
                None,
                Some("rust-toolchain.toml content missing from ProjectTree".to_owned()),
            ),
        },
        None => (None, None, None),
    };

    ToolchainPolicyRootFacts {
        rel_dir: snapshot.rel_dir.clone(),
        cargo_rel_path: snapshot.cargo_rel_path.clone(),
        toolchain_toml_rel: modern_toolchain.0,
        legacy_toolchain_rel,
        parsed: modern_toolchain.1,
        parse_error: modern_toolchain.2,
        cargo_rust_version: snapshot.rust_version.clone(),
        cargo_rust_version_invalid: snapshot.rust_version_invalid,
        cargo_parse_error: snapshot.parse_error.clone(),
    }
}

struct RustVersionField {
    value: Option<String>,
    invalid: bool,
}

fn extract_rust_version(parsed: &toml::Value, is_workspace_root: bool) -> RustVersionField {
    if is_workspace_root {
        if let Some(value) = parsed
            .get("workspace")
            .and_then(|item| item.get("package"))
            .and_then(|item| item.get("rust-version"))
        {
            return RustVersionField {
                value: value.as_str().map(str::to_owned),
                invalid: !value.is_str(),
            };
        }
    }

    if let Some(value) = parsed
        .get("package")
        .and_then(|item| item.get("rust-version"))
    {
        return RustVersionField {
            value: value.as_str().map(str::to_owned),
            invalid: !value.is_str(),
        };
    }

    RustVersionField {
        value: None,
        invalid: false,
    }
}
