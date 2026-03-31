use std::collections::{BTreeMap, BTreeSet};

use guardrail3_domain_config::types::GuardrailConfig;
use guardrail3_app_rs_family_mapper::RsProjectSurface as ProjectTree;

use super::{CargoRootFacts, CodeInputFailureFacts, PolicySettings, rel_is_same_or_descendant};

pub(super) fn read_policy_map(
    tree: &ProjectTree,
    cargo_roots: &BTreeMap<String, CargoRootFacts>,
    input_failures: &mut Vec<CodeInputFailureFacts>,
) -> BTreeMap<String, PolicySettings> {
    let mut map = BTreeMap::new();
    let parsed = match tree.file_content("guardrail3.toml") {
        Some(content) => match toml::from_str::<GuardrailConfig>(content) {
            Ok(_) => match toml::from_str::<toml::Value>(content) {
                Ok(parsed) => Some(parsed),
                Err(parse_error) => {
                    input_failures.push(CodeInputFailureFacts {
                        rel_path: "guardrail3.toml".to_owned(),
                        message: format!(
                            "Failed to parse guardrail3.toml for code-family policy resolution: {parse_error}"
                        ),
                    });
                    None
                }
            },
            Err(parse_error) => {
                input_failures.push(CodeInputFailureFacts {
                    rel_path: "guardrail3.toml".to_owned(),
                    message: format!(
                        "Failed to parse guardrail3.toml for code-family policy resolution: {parse_error}"
                    ),
                });
                None
            }
        },
        None if tree.file_exists("guardrail3.toml") => {
            input_failures.push(CodeInputFailureFacts {
                rel_path: "guardrail3.toml".to_owned(),
                message: "Failed to read guardrail3.toml for code-family policy resolution."
                    .to_owned(),
            });
            None
        }
        None => None,
    };
    let default_profile = parsed
        .as_ref()
        .and_then(|parsed| parsed.get("profile"))
        .and_then(|value| value.get("name"))
        .and_then(toml::Value::as_str)
        .map(str::to_owned);
    let _ = map.insert(
        String::new(),
        PolicySettings {
            profile_name: default_profile.clone(),
        },
    );

    let Some(parsed) = parsed.as_ref() else {
        return map;
    };
    let rust = parsed.get("rust");

    let resolved_app_paths = resolve_app_paths(cargo_roots);
    let mut configured_app_roots = BTreeSet::new();
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
                let _ = configured_app_roots.insert(rel_dir.clone());
                let _ = map.insert(
                    rel_dir.clone(),
                    PolicySettings {
                        profile_name: profile_name.clone(),
                    },
                );
            }
        }
    }

    let package_roots: BTreeSet<_> = cargo_roots
        .values()
        .filter(|facts| {
            facts.has_package
                && !configured_app_roots
                    .iter()
                    .any(|app_root| rel_is_same_or_descendant(&facts.rel_dir, app_root))
        })
        .map(|facts| facts.rel_dir.clone())
        .collect();

    if let Some(packages) = rust.and_then(|value| value.get("packages")) {
        let profile_name = packages
            .get("type")
            .or_else(|| packages.get("profile"))
            .and_then(toml::Value::as_str)
            .map(str::to_owned)
            .or_else(|| Some("library".to_owned()))
            .or_else(|| default_profile.clone());
        for rel_dir in &package_roots {
            let _ = map.insert(
                rel_dir.clone(),
                PolicySettings {
                    profile_name: profile_name.clone(),
                },
            );
        }
    }

    map
}

pub(super) fn policy_settings_for(
    rel_dir: &str,
    policy_map: &BTreeMap<String, PolicySettings>,
) -> PolicySettings {
    if rel_dir.is_empty() {
        return policy_map
            .get("")
            .cloned()
            .unwrap_or(PolicySettings { profile_name: None });
    }

    let mut current = rel_dir;
    loop {
        if let Some(settings) = policy_map.get(current) {
            return settings.clone();
        }
        let Some((parent, _)) = current.rsplit_once('/') else {
            break;
        };
        current = parent;
    }

    policy_map
        .get("")
        .cloned()
        .unwrap_or(PolicySettings { profile_name: None })
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
