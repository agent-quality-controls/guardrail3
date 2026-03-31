use std::collections::BTreeMap;

use guardrail3_app_rs_family_mapper::RsProjectSurface as ProjectTree;

use crate::facts::{
    CargoRootFacts, CoveredRustUnitFacts, DenyConfigFacts, PolicyRootKind, UncoveredRustUnitFacts,
};

pub(crate) struct ProfileMapFacts {
    pub(crate) map: BTreeMap<String, Option<String>>,
    pub(crate) parse_error: Option<String>,
}

fn is_ancestor_dir(ancestor: &str, rel_dir: &str) -> bool {
    ancestor.is_empty() || ancestor == rel_dir || rel_dir.starts_with(&format!("{ancestor}/"))
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
) -> ProfileMapFacts {
    let mut map = BTreeMap::new();
    let _ = map.insert(String::new(), None);
    let resolved_app_paths = resolve_app_paths(cargo_roots);

    if !tree.file_exists("guardrail3.toml") {
        return ProfileMapFacts {
            map,
            parse_error: None,
        };
    };

    let Some(content) = tree.file_content("guardrail3.toml") else {
        return ProfileMapFacts {
            map,
            parse_error: Some("guardrail3.toml content missing from ProjectTree".to_owned()),
        };
    };
    let parsed = match toml::from_str::<toml::Value>(content) {
        Ok(parsed) => parsed,
        Err(err) => {
            return ProfileMapFacts {
                map,
                parse_error: Some(format!(
                    "TOML parse error in active `guardrail3.toml`: {err}"
                )),
            };
        }
    };
    if let Err(err) = validate_guardrail_policy_shape(&parsed) {
        return ProfileMapFacts {
            map,
            parse_error: Some(err),
        };
    }
    let default_profile = parsed
        .get("profile")
        .and_then(|value| value.get("name"))
        .and_then(toml::Value::as_str)
        .map(str::to_owned);
    let _ = map.insert(String::new(), default_profile.clone());
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
        for rel_dir in cargo_roots
            .values()
            .filter(|facts| facts.has_workspace)
            .map(|facts| facts.rel_dir.as_str())
            .filter(|rel_dir| !rel_dir.is_empty())
            .filter(|rel_dir| {
                !resolved_app_paths
                    .values()
                    .any(|app_rel| app_rel == *rel_dir)
            })
        {
            let _ = map
                .entry(rel_dir.to_owned())
                .or_insert(profile_name.clone());
        }
    }

    ProfileMapFacts {
        map,
        parse_error: None,
    }
}

fn resolve_app_paths(cargo_roots: &BTreeMap<String, CargoRootFacts>) -> BTreeMap<String, String> {
    let mut resolved = guardrail3_app_core::discover::resolve_app_paths_from_member_dirs(
        cargo_roots
            .values()
            .filter(|facts| facts.has_workspace)
            .flat_map(|workspace| workspace.workspace_members.iter().cloned())
            .collect::<Vec<_>>(),
    );

    for rel_dir in cargo_roots.keys() {
        let mut parts = rel_dir.split('/');
        if let (Some("apps"), Some(app_name), None) = (parts.next(), parts.next(), parts.next()) {
            let _ = resolved
                .entry(app_name.to_owned())
                .or_insert_with(|| rel_dir.clone());
        }
    }

    resolved
}

fn validate_guardrail_policy_shape(parsed: &toml::Value) -> Result<(), String> {
    if let Some(profile) = parsed.get("profile") {
        let table = profile
            .as_table()
            .ok_or_else(|| "`profile` must be a table in active `guardrail3.toml`.".to_owned())?;
        if let Some(name) = table.get("name") {
            let Some(name) = name.as_str() else {
                return Err(
                    "`profile.name` must be a string in active `guardrail3.toml`.".to_owned(),
                );
            };
            validate_known_profile_name(name, "`profile.name`")?;
        }
    }

    let Some(rust) = parsed.get("rust") else {
        return Ok(());
    };
    let rust_table = rust
        .as_table()
        .ok_or_else(|| "`rust` must be a table in active `guardrail3.toml`.".to_owned())?;

    if let Some(apps) = rust_table.get("apps") {
        let apps_table = apps
            .as_table()
            .ok_or_else(|| "`rust.apps` must be a table in active `guardrail3.toml`.".to_owned())?;
        for (app_name, app_cfg) in apps_table {
            validate_profile_block(app_cfg, &format!("`rust.apps.{app_name}`"))?;
        }
    }

    if let Some(packages) = rust_table.get("packages") {
        validate_profile_block(packages, "`rust.packages`")?;
    }

    Ok(())
}

fn validate_profile_block(value: &toml::Value, context: &str) -> Result<(), String> {
    let table = value
        .as_table()
        .ok_or_else(|| format!("{context} must be a table in active `guardrail3.toml`."))?;
    let type_name = table.get("type").map_or(Ok(None), |value| {
        value.as_str().map(Some).ok_or_else(|| {
            format!("{context}.type/profile must be a string in active `guardrail3.toml`.")
        })
    })?;
    let profile_name = table.get("profile").map_or(Ok(None), |value| {
        value.as_str().map(Some).ok_or_else(|| {
            format!("{context}.type/profile must be a string in active `guardrail3.toml`.")
        })
    })?;

    if let Some(name) = type_name {
        validate_known_profile_name(name, &format!("{context}.type"))?;
    }
    if let Some(name) = profile_name {
        validate_known_profile_name(name, &format!("{context}.profile"))?;
    }
    if let (Some(type_name), Some(profile_name)) = (type_name, profile_name) {
        if type_name != profile_name {
            return Err(format!(
                "{context}.type and {context}.profile must match in active `guardrail3.toml`."
            ));
        }
    }
    Ok(())
}

fn validate_known_profile_name(profile_name: &str, context: &str) -> Result<(), String> {
    if matches!(profile_name, "service" | "library") {
        Ok(())
    } else {
        Err(format!(
            "{context} must be `service` or `library` in active `guardrail3.toml`."
        ))
    }
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
    let Some(root) = tree.structure().get("") else {
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
