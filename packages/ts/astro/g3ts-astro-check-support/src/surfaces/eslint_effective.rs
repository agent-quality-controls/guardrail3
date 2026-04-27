use super::eslint_options::*;
use super::prelude::*;
use super::constants::*;

pub(super) fn active_probe<'a>(
    typed: &'a eslint_config_parser::types::EslintConfigSnapshot,
    kind: eslint_config_parser::types::EslintProbeKind,
) -> Option<&'a eslint_config_parser::types::EslintEffectiveConfigProbe> {
    probe_by_kind(typed, kind).filter(|probe| !probe.ignored)
}

pub(super) fn probe_by_kind(
    typed: &eslint_config_parser::types::EslintConfigSnapshot,
    kind: eslint_config_parser::types::EslintProbeKind,
) -> Option<&eslint_config_parser::types::EslintEffectiveConfigProbe> {
    typed.probes.iter().find(|probe| probe.probe == kind)
}

pub(super) fn probe_ignored(
    typed: &eslint_config_parser::types::EslintConfigSnapshot,
    kind: eslint_config_parser::types::EslintProbeKind,
) -> bool {
    probe_by_kind(typed, kind).is_none_or(|probe| probe.ignored)
}

pub(super) fn active_error_rules(
    probe: Option<&eslint_config_parser::types::EslintEffectiveConfigProbe>,
) -> Vec<String> {
    probe
        .map(|probe| {
            probe
                .rules
                .iter()
                .filter_map(|(rule_name, setting)| {
                    (setting.severity == eslint_config_parser::types::EslintRuleSeverity::Error)
                        .then_some(rule_name.clone())
                })
                .collect()
        })
        .unwrap_or_default()
}

pub(super) fn route_scoped_pipeline_rule_scopes(
    probe: Option<&eslint_config_parser::types::EslintEffectiveConfigProbe>,
) -> Vec<G3TsAstroPipelineRuleScopeSnapshot> {
    let Some(probe) = probe else {
        return Vec::new();
    };

    ROUTE_SCOPED_PIPELINE_RULES
        .iter()
        .filter_map(|rule_name| {
            let setting = probe.rules.get(*rule_name)?;
            if !rule_setting_is_error(setting) {
                return None;
            }
            Some(G3TsAstroPipelineRuleScopeSnapshot {
                rule_name: (*rule_name).to_owned(),
                route_globs: string_array_option(setting, "routeGlobs"),
                endpoint_globs: string_array_option(setting, "endpointGlobs"),
            })
        })
        .collect()
}

pub(super) fn effective_route_scoped_pipeline_rules(
    probe: Option<&eslint_config_parser::types::EslintEffectiveConfigProbe>,
    route_page_paths: &[String],
    endpoint_paths: &[String],
) -> Vec<String> {
    let Some(probe) = probe else {
        return Vec::new();
    };

    ROUTE_SCOPED_PIPELINE_RULES
        .iter()
        .filter(|rule_name| {
            probe.rules.get(**rule_name).is_some_and(|setting| {
                rule_setting_is_error(setting)
                    && rule_setting_has_route_and_endpoint_coverage(
                        setting,
                        route_page_paths,
                        endpoint_paths,
                    )
                    && (**rule_name != CONTENT_ADAPTER_PIPELINE_RULE
                        || !string_array_option(setting, "approvedContentAdapterModules")
                            .is_empty())
            })
        })
        .map(|rule_name| (*rule_name).to_owned())
        .collect()
}

pub(super) fn effective_content_adapter_modules(
    probe: Option<&eslint_config_parser::types::EslintEffectiveConfigProbe>,
) -> Vec<String> {
    let Some(probe) = probe else {
        return Vec::new();
    };

    probe
        .rules
        .get(CONTENT_ADAPTER_PIPELINE_RULE)
        .map_or_else(Vec::new, |setting| {
            if rule_setting_is_error(setting)
                && probe_has_pipeline_plugin_package(probe)
                && !string_array_option(setting, "approvedContentAdapterModules").is_empty()
            {
                string_array_option(setting, "approvedContentAdapterModules")
            } else {
                Vec::new()
            }
        })
}

pub(super) fn effective_content_data_pipeline_rules(
    probe: Option<&eslint_config_parser::types::EslintEffectiveConfigProbe>,
    route_page_paths: &[String],
    endpoint_paths: &[String],
) -> Vec<String> {
    let Some(probe) = probe else {
        return Vec::new();
    };

    CONTENT_DATA_PIPELINE_RULES
        .iter()
        .filter(|rule_name| {
            probe.rules.get(**rule_name).is_some_and(|setting| {
                rule_setting_is_error(setting)
                    && rule_setting_has_route_and_endpoint_coverage(
                        setting,
                        route_page_paths,
                        endpoint_paths,
                    )
                    && rule_setting_has_content_data_scope(setting)
            })
        })
        .map(|rule_name| (*rule_name).to_owned())
        .collect()
}

pub(super) fn effective_content_source_pipeline_rules(
    probe: Option<&eslint_config_parser::types::EslintEffectiveConfigProbe>,
    route_page_paths: &[String],
    endpoint_paths: &[String],
) -> Vec<String> {
    let Some(probe) = probe else {
        return Vec::new();
    };

    CONTENT_SOURCE_PIPELINE_RULES
        .iter()
        .filter(|rule_name| {
            probe.rules.get(**rule_name).is_some_and(|setting| {
                rule_setting_is_error(setting)
                    && rule_setting_has_route_and_endpoint_coverage(
                        setting,
                        route_page_paths,
                        endpoint_paths,
                    )
                    && rule_setting_has_content_source_scope(setting)
            })
        })
        .map(|rule_name| (*rule_name).to_owned())
        .collect()
}

pub(super) fn effective_inline_public_content_rules(
    probe: Option<&eslint_config_parser::types::EslintEffectiveConfigProbe>,
) -> Vec<String> {
    let Some(probe) = probe else {
        return Vec::new();
    };

    probe
        .rules
        .get(INLINE_PUBLIC_CONTENT_RULE)
        .map_or_else(Vec::new, |setting| {
            if rule_setting_is_error(setting)
                && rule_setting_has_inline_public_content_policy(setting)
            {
                vec![INLINE_PUBLIC_CONTENT_RULE.to_owned()]
            } else {
                Vec::new()
            }
        })
}

pub(super) fn effective_mdx_component_map_rules(
    probe: Option<&eslint_config_parser::types::EslintEffectiveConfigProbe>,
    mdx_content_paths: &[String],
    approved_mdx_component_modules: &[String],
) -> Vec<String> {
    let Some(probe) = probe else {
        return Vec::new();
    };

    probe
        .rules
        .get(MDX_COMPONENT_MAP_PIPELINE_RULE)
        .map_or_else(Vec::new, |setting| {
            if rule_setting_is_error(setting)
                && probe_has_pipeline_plugin_package(probe)
                && rule_setting_has_option_globs_coverage(
                    setting,
                    "mdxContentGlobs",
                    mdx_content_paths,
                )
                && rule_setting_has_expected_module_globs(
                    setting,
                    "approvedMdxComponentModules",
                    approved_mdx_component_modules,
                )
            {
                vec![MDX_COMPONENT_MAP_PIPELINE_RULE.to_owned()]
            } else {
                Vec::new()
            }
        })
}

pub(super) fn effective_metadata_helper_rules(
    probe: Option<&eslint_config_parser::types::EslintEffectiveConfigProbe>,
    route_page_paths: &[String],
    endpoint_paths: &[String],
    approved_metadata_helpers: &[String],
    approved_content_adapters: &[String],
) -> Vec<String> {
    let required_modules = [
        ("approvedMetadataHelperModules", approved_metadata_helpers),
        ("approvedContentAdapterModules", approved_content_adapters),
    ];
    effective_route_helper_rules(
        probe,
        route_page_paths,
        endpoint_paths,
        METADATA_HELPER_PIPELINE_RULE,
        &required_modules,
    )
}

pub(super) fn effective_json_ld_helper_rules(
    probe: Option<&eslint_config_parser::types::EslintEffectiveConfigProbe>,
    route_page_paths: &[String],
    endpoint_paths: &[String],
    approved_json_ld_helpers: &[String],
) -> Vec<String> {
    let required_modules = [("approvedJsonLdHelperModules", approved_json_ld_helpers)];
    effective_route_helper_rules(
        probe,
        route_page_paths,
        endpoint_paths,
        JSON_LD_HELPER_PIPELINE_RULE,
        &required_modules,
    )
}

pub(super) fn effective_route_helper_rules(
    probe: Option<&eslint_config_parser::types::EslintEffectiveConfigProbe>,
    route_page_paths: &[String],
    endpoint_paths: &[String],
    rule_name: &str,
    required_module_options: &[(&str, &[String])],
) -> Vec<String> {
    let Some(probe) = probe else {
        return Vec::new();
    };

    probe.rules.get(rule_name).map_or_else(Vec::new, |setting| {
        if rule_setting_is_error(setting)
            && probe_has_pipeline_plugin_package(probe)
            && rule_setting_has_route_and_endpoint_coverage(
                setting,
                route_page_paths,
                endpoint_paths,
            )
            && required_module_options.iter().all(|(key, expected)| {
                rule_setting_has_expected_module_globs(setting, key, expected)
            })
        {
            vec![rule_name.to_owned()]
        } else {
            Vec::new()
        }
    })
}

pub(super) fn rule_setting_has_option_globs_coverage(
    setting: &eslint_config_parser::types::EslintRuleSetting,
    key: &str,
    candidate_paths: &[String],
) -> bool {
    if candidate_paths.is_empty() {
        return rule_setting_option_globs_are_valid(setting, key);
    }

    rule_setting_option_globs_match_any_path(setting, key, candidate_paths)
}

pub(super) fn rule_setting_has_route_and_endpoint_coverage(
    setting: &eslint_config_parser::types::EslintRuleSetting,
    route_page_paths: &[String],
    endpoint_paths: &[String],
) -> bool {
    let route_coverage = !route_page_paths.is_empty()
        && rule_setting_option_globs_match_any_path(setting, "routeGlobs", route_page_paths);
    let endpoint_coverage = if endpoint_paths.is_empty() {
        rule_setting_option_globs_are_valid(setting, "endpointGlobs")
    } else {
        rule_setting_option_globs_match_any_path(setting, "endpointGlobs", endpoint_paths)
    };

    route_coverage && endpoint_coverage
}

pub(super) fn rule_setting_has_content_data_scope(
    setting: &eslint_config_parser::types::EslintRuleSetting,
) -> bool {
    first_option_object(setting).is_some_and(|object| {
        has_non_empty_string_array_option(object.get("contentDataModuleGlobs"))
    })
}

pub(super) fn rule_setting_has_content_source_scope(
    setting: &eslint_config_parser::types::EslintRuleSetting,
) -> bool {
    first_option_object(setting).is_some_and(|object| {
        has_non_empty_string_array_option(object.get("authoredContentGlobs"))
            || has_non_empty_string_array_option(object.get("specContentGlobs"))
    })
}

pub(super) fn probe_has_pipeline_plugin_package(
    probe: &eslint_config_parser::types::EslintEffectiveConfigProbe,
) -> bool {
    probe
        .plugin_package_names
        .get("astro-pipeline")
        .is_some_and(|package_names| {
            package_names
                .iter()
                .any(|name| name == "g3ts-eslint-plugin-astro-pipeline")
        })
}
