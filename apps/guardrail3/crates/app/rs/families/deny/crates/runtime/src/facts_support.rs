use std::collections::{BTreeMap, BTreeSet};

use guardrail3_domain_project_tree::ProjectTree;

use crate::facts::{
    CargoRootFacts, CoveredRustUnitFacts, DenyConfigFacts, PolicyRootKind, SameRootConflictFacts,
    UncoveredRustUnitFacts,
};

pub(crate) fn collect_same_root_conflicts(
    allowed_configs: &[DenyConfigFacts],
) -> Vec<SameRootConflictFacts> {
    let mut grouped = BTreeMap::<String, Vec<String>>::new();
    for config in allowed_configs {
        grouped
            .entry(config.policy_root_rel.clone())
            .or_default()
            .push(config.rel_path.clone());
    }

    grouped
        .into_iter()
        .filter_map(|(policy_root_rel, mut rel_paths)| {
            if rel_paths.len() <= 1 {
                None
            } else {
                rel_paths.sort();
                Some(SameRootConflictFacts {
                    policy_root_rel,
                    rel_paths,
                })
            }
        })
        .collect()
}

pub(crate) fn nearest_allowed_ancestor(
    rel_dir: &str,
    allowed_roots: &BTreeSet<String>,
) -> Option<String> {
    allowed_roots
        .iter()
        .filter(|ancestor| is_ancestor_dir(ancestor, rel_dir))
        .max_by_key(|ancestor| ancestor.len())
        .cloned()
}

fn is_ancestor_dir(ancestor: &str, rel_dir: &str) -> bool {
    ancestor.is_empty() || ancestor == rel_dir || rel_dir.starts_with(&format!("{ancestor}/"))
}

pub(crate) fn parent_dir(rel_dir: &str) -> String {
    rel_dir
        .rsplit_once('/')
        .map_or_else(String::new, |(parent, _)| parent.to_owned())
}

fn config_precedence(file_kind: &str) -> usize {
    match file_kind {
        "deny.toml" => 3,
        ".deny.toml" => 2,
        ".cargo/deny.toml" => 1,
        _ => 0,
    }
}

pub(crate) fn read_profile_map(
    tree: &ProjectTree,
    cargo_roots: &BTreeMap<String, CargoRootFacts>,
    standalone_package_roots: &BTreeSet<String>,
) -> BTreeMap<String, Option<String>> {
    let mut map = BTreeMap::new();
    let default_profile = read_default_profile(tree);
    let _ = map.insert(String::new(), default_profile.clone());
    let resolved_app_paths = resolve_app_paths(cargo_roots);

    let Some(content) = tree.file_content("guardrail3.toml") else {
        return map;
    };
    let Ok(parsed) = toml::from_str::<toml::Value>(content) else {
        return map;
    };
    let rust = parsed.get("rust");

    if let Some(apps) = rust
        .and_then(|value| value.get("apps"))
        .and_then(toml::Value::as_table)
    {
        for (app_name, app_cfg) in apps {
            let profile_name = app_cfg
                .get("type")
                .or_else(|| app_cfg.get("profile"))
                .and_then(toml::Value::as_str)
                .map(str::to_owned)
                .or_else(|| default_profile.clone());
            if let Some(rel_dir) = resolved_app_paths.get(app_name) {
                let _ = map.insert(rel_dir.clone(), profile_name);
            }
        }
    }

    if let Some(packages) = rust.and_then(|value| value.get("packages")) {
        let profile_name = packages
            .get("type")
            .or_else(|| packages.get("profile"))
            .and_then(toml::Value::as_str)
            .map(str::to_owned)
            .or_else(|| Some("library".to_owned()))
            .or_else(|| default_profile.clone());
        if !resolved_app_paths
            .values()
            .any(|rel_dir| rel_dir.is_empty())
        {
            let _ = map.insert(String::new(), profile_name.clone());
        }
        for rel_dir in standalone_package_roots {
            let _ = map.insert(rel_dir.clone(), profile_name.clone());
        }
    }

    map
}

fn resolve_app_paths(cargo_roots: &BTreeMap<String, CargoRootFacts>) -> BTreeMap<String, String> {
    guardrail3_app_core::discover::resolve_app_paths_from_member_dirs(
        cargo_roots
            .values()
            .filter(|facts| facts.has_workspace)
            .flat_map(|workspace| workspace.workspace_members.iter().cloned())
            .collect::<Vec<_>>(),
    )
}

fn read_default_profile(tree: &ProjectTree) -> Option<String> {
    let content = tree.file_content("guardrail3.toml")?;
    let parsed = toml::from_str::<toml::Value>(content).ok()?;
    parsed
        .get("profile")
        .and_then(|value| value.get("name"))
        .and_then(toml::Value::as_str)
        .map(str::to_owned)
}

pub(crate) fn profile_for(
    rel_dir: &str,
    profile_map: &BTreeMap<String, Option<String>>,
) -> Option<String> {
    if let Some(profile) = profile_map.get(rel_dir) {
        return profile.clone();
    }
    profile_map.get("").cloned().flatten()
}

pub(crate) fn push_coverage_facts(
    tree: &ProjectTree,
    rel_dir: &str,
    kind: PolicyRootKind,
    allowed_configs: &[DenyConfigFacts],
    covered_units: &mut Vec<CoveredRustUnitFacts>,
    uncovered_units: &mut Vec<UncoveredRustUnitFacts>,
) {
    if let Some(covering_config_rel) = nearest_covering_config(rel_dir, allowed_configs) {
        covered_units.push(CoveredRustUnitFacts {
            rel_dir: rel_dir.to_owned(),
            kind,
            covering_config_rel,
            quiet_if_self_hosted: rel_dir.is_empty() && is_self_hosted_family_root(tree),
        });
    } else {
        uncovered_units.push(UncoveredRustUnitFacts {
            rel_dir: rel_dir.to_owned(),
            kind,
        });
    }
}

fn is_self_hosted_family_root(tree: &ProjectTree) -> bool {
    let Some(root) = tree.structure.get("") else {
        return false;
    };
    if !root.has_file("Cargo.toml")
        || !root.has_file("README.md")
        || !root.has_file("rustfmt.toml")
        || !root.has_file("rust-toolchain.toml")
        || !root.has_dir("crates")
        || !root.has_dir("test_support")
    {
        return false;
    }
    tree.file_content("crates/runtime/Cargo.toml").is_some()
        && tree.file_content("crates/assertions/Cargo.toml").is_some()
        && tree.file_content("test_support/Cargo.toml").is_some()
}

fn nearest_covering_config(rel_dir: &str, allowed_configs: &[DenyConfigFacts]) -> Option<String> {
    allowed_configs
        .iter()
        .filter(|config| is_ancestor_dir(&config.policy_root_rel, rel_dir))
        .max_by_key(|config| {
            (
                config.policy_root_rel.len(),
                config_precedence(&config.file_kind),
            )
        })
        .map(|config| config.rel_path.clone())
}
