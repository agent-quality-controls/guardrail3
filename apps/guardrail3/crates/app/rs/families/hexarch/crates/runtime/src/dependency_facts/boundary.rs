use std::collections::{BTreeMap, BTreeSet};

use guardrail3_domain_config::types::GuardrailConfig;
use guardrail3_app_rs_family_view::FamilyView as ProjectTree;

use super::{BoundaryConfigFacts, GuardrailConfigSnapshot, ParsedGuardrailConfig};

pub(super) fn collect_boundary_configs(
    owned_app_roots: &BTreeSet<String>,
    guardrail: &GuardrailConfigSnapshot,
) -> Vec<BoundaryConfigFacts> {
    let mut boundaries = BTreeMap::<String, BoundaryConfigFacts>::new();
    if let Some(parse_error) = &guardrail.parse_error {
        let _ = boundaries.insert(
            "guardrail3.toml".to_owned(),
            BoundaryConfigFacts {
                rel_dir: "guardrail3.toml".to_owned(),
                has_config_entry: false,
                is_app_boundary: false,
                parse_error: Some(parse_error.clone()),
            },
        );
        if !guardrail.raw_parse_succeeded {
            return boundaries.into_values().collect();
        }
    }

    for app_root in owned_app_roots {
        let app_name = app_root.rsplit('/').next().unwrap_or(app_root);
        let _ = boundaries.insert(
            app_root.clone(),
            BoundaryConfigFacts {
                rel_dir: app_root.clone(),
                has_config_entry: guardrail.app_config_names.contains(app_name),
                is_app_boundary: true,
                parse_error: None,
            },
        );
    }
    boundaries.into_values().collect()
}

pub(super) fn parse_guardrail_config(
    tree: &ProjectTree,
    rel_path: Option<&str>,
) -> GuardrailConfigSnapshot {
    let Some(rel_path) = rel_path else {
        return GuardrailConfigSnapshot::default();
    };
    let Some(content) = tree.file_content(rel_path) else {
        return GuardrailConfigSnapshot::default();
    };
    let raw_value = toml::from_str::<toml::Value>(content).ok();
    let raw_app_config_names = raw_value
        .as_ref()
        .and_then(|value| {
            value
                .get("rust")
                .and_then(|rust| rust.get("apps"))
                .and_then(toml::Value::as_table)
                .map(|apps| apps.keys().cloned().collect::<BTreeSet<_>>())
        })
        .unwrap_or_default();
    match toml::from_str::<GuardrailConfig>(content) {
        Ok(config) => GuardrailConfigSnapshot {
            parsed: Some(ParsedGuardrailConfig {
                root_profile_name: config.profile().map(|profile| profile.name().to_owned()),
                app_configs: config
                    .rust()
                    .and_then(|rust| rust.apps().cloned())
                    .unwrap_or_default(),
                packages_config: config.rust().and_then(|rust| rust.packages().cloned()),
                escape_hatches: config.escape_hatches().to_vec(),
            }),
            parse_error: None,
            app_config_names: raw_app_config_names,
            raw_parse_succeeded: true,
        },
        Err(parse_error) => GuardrailConfigSnapshot {
            parsed: None,
            parse_error: Some(parse_error.to_string()),
            app_config_names: raw_app_config_names,
            raw_parse_succeeded: raw_value.is_some(),
        },
    }
}
