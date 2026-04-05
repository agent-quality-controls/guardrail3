use std::collections::{BTreeMap, BTreeSet};

use cargo_toml_parser::CargoToml;
use guardrail3_app_rs_family_mapper::RsToolchainRoute;
use guardrail3_app_rs_ownership::RustFamilyFileKind;
use guardrail3_app_rs_family_view::FamilyView as ProjectTree;

use super::facts::{ToolchainFamilyFacts, ToolchainPolicyRootFacts};

#[derive(Debug, Clone)]
struct CargoSnapshot {
    rel_dir: String,
    cargo_rel_path: String,
    cargo_parsed: Option<CargoToml>,
    parse_error: Option<String>,
    has_workspace: bool,
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
    if tree.file_content("Cargo.toml").is_some() {
        let _ = rel_dirs.insert(String::new());
    }
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
            cargo_parsed: None,
            parse_error: Some("Cargo.toml content missing from ProjectTree".to_owned()),
            has_workspace: false,
        };
    };

    let has_workspace = contains_workspace_table(content);
    match cargo_toml_parser::parse(content) {
        Ok(parsed) => CargoSnapshot {
            rel_dir: rel_dir.to_owned(),
            cargo_rel_path: cargo_rel_path.to_owned(),
            cargo_parsed: Some(parsed),
            parse_error: None,
            has_workspace,
        },
        Err(parse_error) => CargoSnapshot {
            rel_dir: rel_dir.to_owned(),
            cargo_rel_path: cargo_rel_path.to_owned(),
            cargo_parsed: None,
            parse_error: Some(parse_error.to_string()),
            has_workspace,
        },
    }
}

fn contains_workspace_table(content: &str) -> bool {
    content.lines().map(str::trim).any(|line| {
        matches!(line, "[workspace]") || line.starts_with("[workspace.")
    })
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
    }).or_else(|| direct_toolchain_rel(tree, &snapshot.rel_dir, "rust-toolchain.toml"));
    let legacy_toolchain_rel = route.family_files().iter().find_map(|file| {
        (file.logical_owner_rel() == snapshot.rel_dir
            && file.exact_rust_root_owner()
            && file.kind() == RustFamilyFileKind::RustToolchainLegacy)
            .then(|| file.rel_path().to_owned())
    }).or_else(|| direct_toolchain_rel(tree, &snapshot.rel_dir, "rust-toolchain"));

    let modern_toolchain = match modern_toolchain_rel.as_deref() {
        Some(toolchain_toml_rel) => match tree.file_content(toolchain_toml_rel) {
            Some(content) => match rust_toolchain_toml_parser::parse(content) {
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
        cargo_parsed: snapshot.cargo_parsed.clone(),
        cargo_parse_error: snapshot.parse_error.clone(),
    }
}

fn direct_toolchain_rel(tree: &ProjectTree, rel_dir: &str, file_name: &str) -> Option<String> {
    let rel = if rel_dir.is_empty() {
        file_name.to_owned()
    } else {
        ProjectTree::join_rel(rel_dir, file_name)
    };
    tree.file_content(&rel).map(|_| rel)
}
