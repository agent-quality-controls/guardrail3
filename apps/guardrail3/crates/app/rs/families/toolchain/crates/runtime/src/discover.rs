use std::collections::{BTreeMap, BTreeSet};

use guardrail3_app_rs_family_mapper::RsToolchainRoute;
use guardrail3_domain_project_tree::ProjectTree;

use super::facts::{PolicyRootKind, ToolchainFamilyFacts, ToolchainPolicyRootFacts};

#[derive(Debug, Clone)]
struct CargoSnapshot {
    rel_dir: String,
    cargo_rel_path: String,
    parse_error: Option<String>,
    has_workspace: bool,
    has_package: bool,
    declared_members: Vec<String>,
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
    let workspace_members: BTreeSet<_> = snapshots
        .values()
        .filter(|snapshot| snapshot.has_workspace)
        .flat_map(|snapshot| snapshot.declared_members.iter().cloned())
        .collect();

    let mut policy_roots = Vec::new();

    for workspace_root_rel in &workspace_roots {
        let Some(snapshot) = snapshots.get(workspace_root_rel) else {
            continue;
        };
        policy_roots.push(build_policy_root(
            tree,
            snapshot,
            PolicyRootKind::WorkspaceRoot,
        ));
    }

    for snapshot in snapshots.values() {
        if snapshot.has_workspace {
            continue;
        }
        if workspace_members.contains(&snapshot.rel_dir) {
            continue;
        }
        if snapshot.has_package || snapshot.parse_error.is_some() {
            policy_roots.push(build_policy_root(
                tree,
                snapshot,
                PolicyRootKind::StandalonePackageRoot,
            ));
        }
    }

    policy_roots.sort_by(|a, b| a.cargo_rel_path.cmp(&b.cargo_rel_path));

    ToolchainFamilyFacts { policy_roots }
}

fn collect_cargo_snapshots(
    tree: &ProjectTree,
    route: &RsToolchainRoute,
) -> BTreeMap<String, CargoSnapshot> {
    route
        .roots()
        .iter()
        .map(|root| {
            let snapshot = cargo_snapshot(tree, root.rel_dir(), root.cargo_rel_path());
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
            has_package: false,
            declared_members: Vec::new(),
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
                has_package: parsed.get("package").is_some(),
                declared_members: if has_workspace {
                    parse_workspace_members(tree, rel_dir, &parsed)
                } else {
                    Vec::new()
                },
                rust_version: rust_version.value,
                rust_version_invalid: rust_version.invalid,
            }
        }
        Err(parse_error) => CargoSnapshot {
            rel_dir: rel_dir.to_owned(),
            cargo_rel_path: cargo_rel_path.to_owned(),
            parse_error: Some(parse_error.to_string()),
            has_workspace: false,
            has_package: false,
            declared_members: Vec::new(),
            rust_version: None,
            rust_version_invalid: false,
        },
    }
}

fn build_policy_root(
    tree: &ProjectTree,
    snapshot: &CargoSnapshot,
    kind: PolicyRootKind,
) -> ToolchainPolicyRootFacts {
    let toolchain_toml_rel = rel_path(&snapshot.rel_dir, "rust-toolchain.toml");
    let legacy_toolchain_rel = rel_path(&snapshot.rel_dir, "rust-toolchain");

    let modern_toolchain = if tree.file_exists(&toolchain_toml_rel) {
        match tree.file_content(&toolchain_toml_rel) {
            Some(content) => match toml::from_str::<toml::Value>(content) {
                Ok(parsed) => (Some(toolchain_toml_rel.clone()), Some(parsed), None),
                Err(parse_error) => (
                    Some(toolchain_toml_rel.clone()),
                    None,
                    Some(parse_error.to_string()),
                ),
            },
            None => (
                Some(toolchain_toml_rel.clone()),
                None,
                Some("rust-toolchain.toml content missing from ProjectTree".to_owned()),
            ),
        }
    } else {
        (None, None, None)
    };

    ToolchainPolicyRootFacts {
        kind,
        rel_dir: snapshot.rel_dir.clone(),
        cargo_rel_path: snapshot.cargo_rel_path.clone(),
        toolchain_toml_rel: modern_toolchain.0,
        legacy_toolchain_rel: tree
            .file_exists(&legacy_toolchain_rel)
            .then_some(legacy_toolchain_rel),
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

fn parse_workspace_members(
    tree: &ProjectTree,
    workspace_rel: &str,
    parsed: &toml::Value,
) -> Vec<String> {
    let mut members = BTreeSet::new();
    let raw_patterns = parsed
        .get("workspace")
        .and_then(|item| item.get("members"))
        .and_then(toml::Value::as_array)
        .map(|members| {
            members
                .iter()
                .filter_map(toml::Value::as_str)
                .map(str::to_owned)
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();

    for pattern in raw_patterns {
        for member_rel in expand_member_pattern(tree, workspace_rel, &pattern) {
            let _ = members.insert(member_rel);
        }
    }

    members.into_iter().collect()
}

fn expand_member_pattern(tree: &ProjectTree, workspace_rel: &str, pattern: &str) -> Vec<String> {
    let normalized = normalize_member_rel(pattern);
    let rel_pattern = if workspace_rel.is_empty() {
        normalized.clone()
    } else {
        ProjectTree::join_rel(workspace_rel, &normalized)
    };

    if looks_like_glob(&normalized) {
        tree.matching_dir_rels(&rel_pattern)
            .into_iter()
            .map(|rel| normalize_member_rel(&rel))
            .collect()
    } else {
        vec![normalize_member_rel(&rel_pattern)]
    }
}

fn looks_like_glob(pattern: &str) -> bool {
    pattern.contains('*') || pattern.contains('?') || pattern.contains('[')
}

fn normalize_member_rel(pattern: &str) -> String {
    pattern
        .trim_matches('/')
        .strip_prefix("./")
        .unwrap_or(pattern.trim_matches('/'))
        .trim_matches('/')
        .to_owned()
}

fn rel_path(rel_dir: &str, file_name: &str) -> String {
    if rel_dir.is_empty() {
        file_name.to_owned()
    } else {
        ProjectTree::join_rel(rel_dir, file_name)
    }
}
