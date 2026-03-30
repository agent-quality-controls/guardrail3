use std::collections::BTreeMap;

use guardrail3_app_core::discover::resolve_app_paths_from_member_dirs;
use guardrail3_domain_config::types::{CrateConfig, GuardrailConfig, RustChecksConfig};
use guardrail3_domain_project_tree::ProjectTree;

use super::{CargoRootFacts, GardeInputFailureFacts, PolicySettings};

pub(super) fn read_policy_map(
    tree: &ProjectTree,
    cargo_roots: &BTreeMap<String, CargoRootFacts>,
    guardrail_rel_path: Option<&str>,
    input_failures: &mut Vec<GardeInputFailureFacts>,
) -> BTreeMap<String, PolicySettings> {
    let parsed = match guardrail_rel_path.and_then(|rel_path| tree.file_content(rel_path)) {
        Some(content) => match toml::from_str::<GuardrailConfig>(content) {
            Ok(parsed) => Some(parsed),
            Err(parse_error) => {
                input_failures.push(GardeInputFailureFacts {
                    rel_path: guardrail_rel_path.unwrap_or("guardrail3.toml").to_owned(),
                    message: format!(
                        "Failed to parse guardrail3.toml for garde policy resolution: {parse_error}"
                    ),
                });
                None
            }
        },
        None => None,
    };

    let default_garde = parsed
        .as_ref()
        .and_then(GuardrailConfig::rust)
        .and_then(|rust| rust.checks())
        .and_then(RustChecksConfig::garde)
        .unwrap_or(true);

    let app_paths = resolve_app_paths_from_member_dirs(
        cargo_roots
            .values()
            .flat_map(|root| root.workspace_members.iter().cloned()),
    );
    let app_paths_include_root = app_paths.values().any(|rel_dir| rel_dir.is_empty());

    let mut map = BTreeMap::from([(
        String::new(),
        PolicySettings {
            garde_enabled: default_garde,
        },
    )]);

    for (app_name, app_dir) in &app_paths {
        let app_cfg = parsed
            .as_ref()
            .and_then(GuardrailConfig::rust)
            .and_then(|rust| rust.apps())
            .and_then(|apps| apps.get(app_name));
        let garde_enabled = app_cfg
            .and_then(crate_checks)
            .and_then(RustChecksConfig::garde)
            .unwrap_or(default_garde);
        let _ = map.insert(app_dir.clone(), PolicySettings { garde_enabled });
    }

    if let Some(packages_cfg) = parsed
        .as_ref()
        .and_then(GuardrailConfig::rust)
        .and_then(|rust| rust.packages())
    {
        let garde_enabled = crate_checks(packages_cfg)
            .and_then(RustChecksConfig::garde)
            .unwrap_or(default_garde);
        if !app_paths_include_root {
            let _ = map.insert(String::new(), PolicySettings { garde_enabled });
        }
        for package_dir in cargo_roots
            .values()
            .filter(|facts| facts.has_workspace)
            .map(|facts| facts.rel_dir.as_str())
            .filter(|rel_dir| !rel_dir.is_empty())
            .filter(|rel_dir| !app_paths.values().any(|app_rel| app_rel == *rel_dir))
        {
            let _ = map.insert(package_dir.to_owned(), PolicySettings { garde_enabled });
        }
    }

    map
}

fn crate_checks(config: &CrateConfig) -> Option<&RustChecksConfig> {
    config.checks()
}

pub(super) fn policy_settings_for(
    rel_dir: &str,
    policy_map: &BTreeMap<String, PolicySettings>,
) -> PolicySettings {
    let mut best = policy_map.get("").cloned().unwrap_or(PolicySettings {
        garde_enabled: true,
    });
    let mut best_len = 0usize;

    for (candidate_dir, settings) in policy_map {
        if candidate_dir.is_empty() {
            continue;
        }
        if rel_dir == candidate_dir
            || rel_dir
                .strip_prefix(candidate_dir)
                .is_some_and(|rest| rest.starts_with('/'))
        {
            let len = candidate_dir.len();
            if len > best_len {
                best = settings.clone();
                best_len = len;
            }
        }
    }

    best
}
