use std::collections::{BTreeMap, BTreeSet};

use guardrail3_app_rs_family_mapper::RsToolchainRoute;
use guardrail3_app_rs_placement::is_excluded_live_root_dir;
use guardrail3_domain_project_tree::ProjectTree;

use super::facts::{
    AncestorToolchainFacts, DescendantToolchainFacts, ToolchainFamilyFacts, ToolchainPolicyRootFacts,
    UnownedToolchainFacts,
};

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
        policy_roots.push(build_policy_root(tree, snapshot));
    }

    policy_roots.sort_by(|a, b| a.cargo_rel_path.cmp(&b.cargo_rel_path));

    let unowned_toolchains = unowned_toolchains(&policy_roots, tree);

    ToolchainFamilyFacts {
        policy_roots,
        unowned_toolchains,
    }
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
    snapshot: &CargoSnapshot,
) -> ToolchainPolicyRootFacts {
    let toolchain_toml_rel = rel_path(&snapshot.rel_dir, "rust-toolchain.toml");
    let legacy_toolchain_rel = rel_path(&snapshot.rel_dir, "rust-toolchain");
    let has_legacy_toolchain = tree.file_exists(&legacy_toolchain_rel);

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
        rel_dir: snapshot.rel_dir.clone(),
        cargo_rel_path: snapshot.cargo_rel_path.clone(),
        toolchain_toml_rel: modern_toolchain.0,
        legacy_toolchain_rel: has_legacy_toolchain.then_some(legacy_toolchain_rel),
        parsed: modern_toolchain.1,
        parse_error: modern_toolchain.2,
        cargo_rust_version: snapshot.rust_version.clone(),
        cargo_rust_version_invalid: snapshot.rust_version_invalid,
        cargo_parse_error: snapshot.parse_error.clone(),
        ancestor_toolchain: nearest_ancestor_toolchain(tree, &snapshot.rel_dir),
        descendant_toolchains: descendant_toolchains(tree, &snapshot.rel_dir),
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

fn rel_path(rel_dir: &str, file_name: &str) -> String {
    if rel_dir.is_empty() {
        file_name.to_owned()
    } else {
        ProjectTree::join_rel(rel_dir, file_name)
    }
}

fn is_nested_beneath(rel_dir: &str, parent_rel: &str) -> bool {
    if parent_rel.is_empty() {
        return !rel_dir.is_empty();
    }

    rel_dir
        .strip_prefix(parent_rel)
        .is_some_and(|suffix| suffix.starts_with('/'))
}

fn nearest_ancestor_toolchain(tree: &ProjectTree, rel_dir: &str) -> Option<AncestorToolchainFacts> {
    for ancestor_rel in ancestor_rel_dirs(rel_dir) {
        let legacy_rel = rel_path(&ancestor_rel, "rust-toolchain");
        if tree.file_exists(&legacy_rel) {
            return Some(AncestorToolchainFacts {
                rel_path: legacy_rel,
                is_legacy: true,
                parsed: None,
                parse_error: None,
            });
        }

        let modern_rel = rel_path(&ancestor_rel, "rust-toolchain.toml");
        if tree.file_exists(&modern_rel) {
            return Some(match tree.file_content(&modern_rel) {
                Some(content) => match toml::from_str::<toml::Value>(content) {
                    Ok(parsed) => AncestorToolchainFacts {
                        rel_path: modern_rel,
                        is_legacy: false,
                        parsed: Some(parsed),
                        parse_error: None,
                    },
                    Err(parse_error) => AncestorToolchainFacts {
                        rel_path: modern_rel,
                        is_legacy: false,
                        parsed: None,
                        parse_error: Some(parse_error.to_string()),
                    },
                },
                None => AncestorToolchainFacts {
                    rel_path: modern_rel,
                    is_legacy: false,
                    parsed: None,
                    parse_error: Some(
                        "ancestor rust-toolchain.toml content missing from ProjectTree".to_owned(),
                    ),
                },
            });
        }
    }

    None
}

fn ancestor_rel_dirs(rel_dir: &str) -> Vec<String> {
    let mut segments = rel_dir
        .split('/')
        .filter(|segment| !segment.is_empty())
        .collect::<Vec<_>>();
    let mut ancestors = Vec::new();

    while !segments.is_empty() {
        let _ = segments.pop();
        ancestors.push(segments.join("/"));
    }

    ancestors
}

fn descendant_toolchains(tree: &ProjectTree, rel_dir: &str) -> Vec<DescendantToolchainFacts> {
    let mut descendants = tree
        .dirs_with_file("rust-toolchain.toml")
        .into_iter()
        .filter(|dir| is_nested_beneath(dir, rel_dir))
        .map(|dir| DescendantToolchainFacts {
            rel_path: rel_path(&dir, "rust-toolchain.toml"),
            is_legacy: false,
        })
        .collect::<Vec<_>>();

    descendants.extend(
        tree.dirs_with_file("rust-toolchain")
            .into_iter()
            .filter(|dir| is_nested_beneath(dir, rel_dir))
            .map(|dir| DescendantToolchainFacts {
                rel_path: rel_path(&dir, "rust-toolchain"),
                is_legacy: true,
            }),
    );

    descendants.sort_by(|left, right| left.rel_path.cmp(&right.rel_path));
    descendants
}

fn unowned_toolchains(
    policy_roots: &[ToolchainPolicyRootFacts],
    tree: &ProjectTree,
) -> Vec<UnownedToolchainFacts> {
    let allowed_roots = policy_roots
        .iter()
        .map(|root| root.rel_dir.clone())
        .collect::<BTreeSet<_>>();

    let mut toolchains = all_toolchain_files(tree)
        .into_iter()
        .filter(|toolchain| {
            let rel_dir = toolchain
                .rel_path
                .rsplit_once('/')
                .map(|(dir, _)| dir)
                .unwrap_or("");

            !allowed_roots.contains(rel_dir)
                && !allowed_roots
                    .iter()
                    .any(|workspace_root| is_nested_beneath(rel_dir, workspace_root))
        })
        .collect::<Vec<_>>();
    toolchains.sort_by(|left, right| left.rel_path.cmp(&right.rel_path));
    toolchains
}

fn all_toolchain_files(tree: &ProjectTree) -> Vec<UnownedToolchainFacts> {
    let mut toolchains = Vec::new();

    if tree.file_exists("rust-toolchain.toml") && !is_excluded_live_root_dir("") {
        toolchains.push(UnownedToolchainFacts {
            rel_path: "rust-toolchain.toml".to_owned(),
            is_legacy: false,
        });
    }
    if tree.file_exists("rust-toolchain") && !is_excluded_live_root_dir("") {
        toolchains.push(UnownedToolchainFacts {
            rel_path: "rust-toolchain".to_owned(),
            is_legacy: true,
        });
    }

    toolchains.extend(
        tree.dirs_with_file("rust-toolchain.toml")
            .into_iter()
            .filter(|dir| !is_excluded_live_root_dir(dir))
            .map(|dir| UnownedToolchainFacts {
                rel_path: rel_path(&dir, "rust-toolchain.toml"),
                is_legacy: false,
            }),
    );
    toolchains.extend(
        tree.dirs_with_file("rust-toolchain")
            .into_iter()
            .filter(|dir| !is_excluded_live_root_dir(dir))
            .map(|dir| UnownedToolchainFacts {
                rel_path: rel_path(&dir, "rust-toolchain"),
                is_legacy: true,
            }),
    );

    toolchains
}
