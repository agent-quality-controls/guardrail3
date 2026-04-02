use std::collections::{BTreeMap, BTreeSet};

use guardrail3_app_rs_family_view::FamilyView as ProjectTree;

use super::{CargoRootFacts, GuardrailPolicyFacts, PolicySettings, ResolvedPolicyMap};

pub(super) fn read_policy_map(
    tree: &ProjectTree,
    cargo_roots: &BTreeMap<String, CargoRootFacts>,
) -> ResolvedPolicyMap {
    let mut map = BTreeMap::new();
    let guardrail = read_guardrail_policy(tree);
    let default_profile = guardrail.default_profile.clone();
    let default_garde = guardrail.default_garde;
    let resolved_app_paths = resolve_app_paths(cargo_roots);
    let app_root_paths: BTreeSet<_> = resolved_app_paths.values().cloned().collect();
    let _ = map.insert(
        String::new(),
        PolicySettings {
            profile_name: default_profile.clone(),
            garde_enabled: default_garde,
        },
    );

    let Some(parsed) = guardrail.parsed.as_ref() else {
        return ResolvedPolicyMap {
            map,
            parse_error: guardrail.parse_error,
        };
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
            let garde_enabled = app_cfg
                .get("checks")
                .and_then(|value| value.get("garde"))
                .and_then(toml::Value::as_bool)
                .unwrap_or(default_garde);
            if let Some(rel_dir) = resolved_app_paths.get(app_name) {
                let _ = map.insert(
                    rel_dir.clone(),
                    PolicySettings {
                        profile_name: profile_name.clone(),
                        garde_enabled,
                    },
                );
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
        let garde_enabled = packages
            .get("checks")
            .and_then(|value| value.get("garde"))
            .and_then(toml::Value::as_bool)
            .unwrap_or(default_garde);
        if !resolved_app_paths
            .values()
            .any(|rel_dir| rel_dir.is_empty())
        {
            let _ = map.insert(
                String::new(),
                PolicySettings {
                    profile_name: profile_name.clone(),
                    garde_enabled,
                },
            );
        }
        for rel_dir in cargo_roots
            .values()
            .filter(|facts| facts.has_workspace)
            .map(|facts| facts.rel_dir.as_str())
            .filter(|rel_dir| !rel_dir.is_empty())
            .filter(|rel_dir| !app_root_paths.contains(*rel_dir))
        {
            let _ = map.insert(
                rel_dir.to_owned(),
                PolicySettings {
                    profile_name: profile_name.clone(),
                    garde_enabled,
                },
            );
        }
    }

    ResolvedPolicyMap {
        map,
        parse_error: guardrail.parse_error,
    }
}

pub(super) fn policy_settings_for(
    rel_dir: &str,
    policy_map: &BTreeMap<String, PolicySettings>,
) -> PolicySettings {
    if rel_dir.is_empty() {
        return policy_map.get("").cloned().unwrap_or(PolicySettings {
            profile_name: None,
            garde_enabled: true,
        });
    }

    if let Some(settings) = policy_map.get(rel_dir) {
        return settings.clone();
    }

    policy_map.get("").cloned().unwrap_or(PolicySettings {
        profile_name: None,
        garde_enabled: true,
    })
}

pub(super) fn published_library_policy(
    tree: &ProjectTree,
    cargo_roots: &BTreeMap<String, CargoRootFacts>,
    rel_dir: &str,
    profile_name: Option<&str>,
) -> bool {
    if profile_name != Some("library") {
        return false;
    }

    package_publishable(tree, rel_dir)
        || cargo_roots
            .get(rel_dir)
            .filter(|facts| facts.has_workspace)
            .map(|facts| {
                facts
                    .workspace_members
                    .iter()
                    .any(|member_rel| package_publishable(tree, member_rel))
            })
            .unwrap_or(false)
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

fn read_guardrail_policy(tree: &ProjectTree) -> GuardrailPolicyFacts {
    if !tree.file_exists("guardrail3.toml") {
        return GuardrailPolicyFacts {
            parsed: None,
            parse_error: None,
            default_profile: None,
            default_garde: true,
        };
    }

    let Some(content) = tree.file_content("guardrail3.toml") else {
        return GuardrailPolicyFacts {
            parsed: None,
            parse_error: Some("guardrail3.toml content missing from ProjectTree".to_owned()),
            default_profile: None,
            default_garde: true,
        };
    };

    match toml::from_str::<toml::Value>(content) {
        Ok(parsed) => match validate_guardrail_policy_shape(&parsed) {
            Ok(()) => GuardrailPolicyFacts {
                default_profile: parsed
                    .get("profile")
                    .and_then(|value| value.get("name"))
                    .and_then(toml::Value::as_str)
                    .map(str::to_owned),
                default_garde: parsed
                    .get("rust")
                    .and_then(|value| value.get("checks"))
                    .and_then(|value| value.get("garde"))
                    .and_then(toml::Value::as_bool)
                    .unwrap_or(true),
                parsed: Some(parsed),
                parse_error: None,
            },
            Err(err) => GuardrailPolicyFacts {
                parsed: None,
                parse_error: Some(err),
                default_profile: None,
                default_garde: true,
            },
        },
        Err(err) => GuardrailPolicyFacts {
            parsed: None,
            parse_error: Some(err.to_string()),
            default_profile: None,
            default_garde: true,
        },
    }
}

fn validate_guardrail_policy_shape(parsed: &toml::Value) -> Result<(), String> {
    if let Some(profile) = parsed.get("profile") {
        let table = profile
            .as_table()
            .ok_or_else(|| "`profile` must be a table in active `guardrail3.toml`.".to_owned())?;
        if let Some(name) = table.get("name") {
            if !name.is_str() {
                return Err(
                    "`profile.name` must be a string in active `guardrail3.toml`.".to_owned(),
                );
            }
        }
    }

    let Some(rust) = parsed.get("rust") else {
        return Ok(());
    };
    let rust_table = rust
        .as_table()
        .ok_or_else(|| "`rust` must be a table in active `guardrail3.toml`.".to_owned())?;

    if let Some(checks) = rust_table.get("checks") {
        validate_garde_field(checks, "`rust.checks`")?;
    }

    if let Some(apps) = rust_table.get("apps") {
        let apps_table = apps
            .as_table()
            .ok_or_else(|| "`rust.apps` must be a table in active `guardrail3.toml`.".to_owned())?;
        for (app_name, app_cfg) in apps_table {
            let ctx = format!("`rust.apps.{app_name}`");
            validate_profile_block(app_cfg, &ctx)?;
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
    if let Some(profile_name) = table.get("type").or_else(|| table.get("profile")) {
        if !profile_name.is_str() {
            return Err(format!(
                "{context}.type/profile must be a string in active `guardrail3.toml`."
            ));
        }
    }
    if let Some(checks) = table.get("checks") {
        validate_garde_field(checks, &format!("{context}.checks"))?;
    }
    Ok(())
}

fn validate_garde_field(value: &toml::Value, context: &str) -> Result<(), String> {
    let table = value
        .as_table()
        .ok_or_else(|| format!("{context} must be a table in active `guardrail3.toml`."))?;
    if let Some(garde) = table.get("garde") {
        if !garde.is_bool() {
            return Err(format!(
                "{context}.garde must be a bool in active `guardrail3.toml`."
            ));
        }
    }
    Ok(())
}

fn package_publishable(tree: &ProjectTree, rel_dir: &str) -> bool {
    let cargo_rel = if rel_dir.is_empty() {
        "Cargo.toml".to_owned()
    } else {
        ProjectTree::join_rel(rel_dir, "Cargo.toml")
    };
    let Some(content) = tree.file_content(&cargo_rel) else {
        return false;
    };
    let Ok(parsed) = toml::from_str::<toml::Value>(content) else {
        return false;
    };
    let Some(package) = parsed.get("package") else {
        return false;
    };
    match package.get("publish") {
        None => true,
        Some(toml::Value::Boolean(value)) => *value,
        Some(toml::Value::Array(array)) => {
            !array.is_empty() && array.iter().all(toml::Value::is_str)
        }
        _ => false,
    }
}
