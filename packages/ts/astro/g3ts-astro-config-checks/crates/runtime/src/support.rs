use g3ts_astro_types::{
    G3TsAstroConfigSurfaceSnapshot, G3TsAstroConfigSurfaceState,
    G3TsAstroEslintPluginContractInput, G3TsAstroEslintSurfaceSnapshot,
    G3TsAstroEslintSurfaceState, G3TsAstroIntegrationContractInput, G3TsAstroOutputMode,
    G3TsAstroPackageSurfaceSnapshot, G3TsAstroPackageSurfaceState, G3TsAstroStaticObjectProperty,
    G3TsAstroStaticValue,
};
use guardrail3_check_types::{G3CheckResult, G3Severity};
use std::collections::BTreeMap;

const NUASITE_CHECKS_ALLOWED_OPTION_KEYS: [&str; 11] = [
    "mode",
    "failOnError",
    "failOnWarning",
    "reportJson",
    "ai",
    "customChecks",
    "overrides",
    "seo",
    "geo",
    "performance",
    "accessibility",
];

#[must_use]
pub(crate) fn parsed_package(
    contract: &G3TsAstroIntegrationContractInput,
) -> Option<&G3TsAstroPackageSurfaceSnapshot> {
    match &contract.package {
        G3TsAstroPackageSurfaceState::Parsed { snapshot } => Some(snapshot),
        G3TsAstroPackageSurfaceState::Missing { .. }
        | G3TsAstroPackageSurfaceState::Unreadable { .. }
        | G3TsAstroPackageSurfaceState::ParseError { .. } => None,
    }
}

#[must_use]
pub(crate) fn package_rel_path(contract: &G3TsAstroIntegrationContractInput) -> Option<&str> {
    match &contract.package {
        G3TsAstroPackageSurfaceState::Missing { rel_path }
        | G3TsAstroPackageSurfaceState::Unreadable { rel_path, .. }
        | G3TsAstroPackageSurfaceState::ParseError { rel_path, .. } => Some(rel_path),
        G3TsAstroPackageSurfaceState::Parsed { snapshot } => Some(&snapshot.rel_path),
    }
}

#[must_use]
pub(crate) fn package_has_dependency(
    contract: &G3TsAstroIntegrationContractInput,
    dependency_name: &str,
) -> bool {
    parsed_package(contract).is_some_and(|snapshot| {
        snapshot
            .dependencies
            .iter()
            .chain(snapshot.dev_dependencies.iter())
            .any(|dependency| dependency == dependency_name)
    })
}

#[must_use]
pub(crate) fn package_safely_runs_astro_check(
    contract: &G3TsAstroIntegrationContractInput,
) -> bool {
    parsed_package(contract).is_some_and(|snapshot| snapshot.safely_runs_astro_check)
}

#[must_use]
pub(crate) fn package_safely_runs_astro_build(
    contract: &G3TsAstroIntegrationContractInput,
) -> bool {
    parsed_package(contract).is_some_and(|snapshot| snapshot.safely_runs_astro_build)
}

#[must_use]
pub(crate) fn expected_syncpack_source_entry(
    syncpack_rel_path: &str,
    package_rel_path: &str,
) -> Option<String> {
    let _syncpack_rel_path = syncpack_rel_path;
    let _package_rel_path = package_rel_path;
    Some("package.json".to_owned())
}

#[must_use]
pub(crate) fn required_syncpack_pins_message(
    contract: &G3TsAstroIntegrationContractInput,
) -> String {
    contract
        .required_syncpack_pins
        .iter()
        .map(|pin| format!("`{}` -> `{}`", pin.dependency, pin.version))
        .collect::<Vec<_>>()
        .join(", ")
}

#[must_use]
#[cfg(test)]
pub(crate) fn forbidden_syncpack_deps_message(
    contract: &G3TsAstroIntegrationContractInput,
) -> String {
    contract
        .forbidden_syncpack_deps
        .iter()
        .map(|dependency| format!("`{dependency}`"))
        .collect::<Vec<_>>()
        .join(", ")
}

#[must_use]
pub(crate) fn parsed_eslint_surface(
    contract: &G3TsAstroEslintPluginContractInput,
) -> Option<&G3TsAstroEslintSurfaceSnapshot> {
    match &contract.config {
        G3TsAstroEslintSurfaceState::Parsed { snapshot } => Some(snapshot),
        G3TsAstroEslintSurfaceState::Missing { .. }
        | G3TsAstroEslintSurfaceState::Unreadable { .. }
        | G3TsAstroEslintSurfaceState::ParseError { .. } => None,
    }
}

#[must_use]
pub(crate) fn parsed_astro_config(
    contract: &G3TsAstroIntegrationContractInput,
) -> Option<&G3TsAstroConfigSurfaceSnapshot> {
    match &contract.astro_config {
        G3TsAstroConfigSurfaceState::Parsed { snapshot } => Some(snapshot),
        G3TsAstroConfigSurfaceState::Missing { .. }
        | G3TsAstroConfigSurfaceState::Unreadable { .. }
        | G3TsAstroConfigSurfaceState::ParseError { .. } => None,
    }
}

#[must_use]
pub(crate) fn astro_config_rel_path(contract: &G3TsAstroIntegrationContractInput) -> Option<&str> {
    match &contract.astro_config {
        G3TsAstroConfigSurfaceState::Missing { rel_path }
        | G3TsAstroConfigSurfaceState::Unreadable { rel_path, .. }
        | G3TsAstroConfigSurfaceState::ParseError { rel_path, .. } => Some(rel_path),
        G3TsAstroConfigSurfaceState::Parsed { snapshot } => Some(&snapshot.rel_path),
    }
}

#[must_use]
pub(crate) fn astro_config_is_static(contract: &G3TsAstroIntegrationContractInput) -> bool {
    parsed_astro_config(contract)
        .is_some_and(|snapshot| snapshot.output == Some(G3TsAstroOutputMode::Static))
}

#[must_use]
pub(crate) fn astro_config_site_is_https(snapshot: &G3TsAstroConfigSurfaceSnapshot) -> bool {
    snapshot.site.as_deref().is_some_and(|site| {
        url::Url::parse(site).is_ok_and(|url| url.scheme() == "https" && url.host_str().is_some())
    })
}

#[must_use]
pub(crate) fn astro_config_has_zero_arg_integration(
    snapshot: &G3TsAstroConfigSurfaceSnapshot,
    module: &str,
    accepted_imported_names: &[Option<&str>],
) -> bool {
    snapshot.integrations.iter().any(|integration| {
        integration.source_module.as_deref() == Some(module)
            && integration.call.is_some()
            && integration
                .call
                .as_ref()
                .is_some_and(|call| call.first_arg.is_none())
            && accepted_imported_names
                .iter()
                .any(|expected| integration.imported_name.as_deref() == *expected)
    })
}

#[must_use]
pub(crate) fn astro_config_has_object_arg_integration(
    snapshot: &G3TsAstroConfigSurfaceSnapshot,
    module: &str,
    accepted_imported_names: &[Option<&str>],
) -> bool {
    snapshot.integrations.iter().any(|integration| {
        integration.source_module.as_deref() == Some(module)
            && integration
                .call
                .as_ref()
                .is_some_and(|call| matches!(call.first_arg, Some(G3TsAstroStaticValue::Object(_))))
            && accepted_imported_names
                .iter()
                .any(|expected| integration.imported_name.as_deref() == *expected)
    })
}

#[must_use]
pub(crate) fn astro_config_has_nuasite_checks_with_required_options(
    snapshot: &G3TsAstroConfigSurfaceSnapshot,
) -> bool {
    snapshot.integrations.iter().any(|integration| {
        integration.source_module.as_deref() == Some("@nuasite/checks")
            && integration.call.is_some()
            && matches!(integration.imported_name.as_deref(), None | Some("checks"))
            && integration
                .call
                .as_ref()
                .and_then(|call| call.first_arg.as_ref())
                .is_some_and(checks_options_are_fail_closed)
    })
}

#[must_use]
pub(crate) fn checks_options_include_structured_data_check(
    snapshot: &G3TsAstroConfigSurfaceSnapshot,
) -> bool {
    snapshot.integrations.iter().any(|integration| {
        integration.source_module.as_deref() == Some("@nuasite/checks")
            && integration.call.is_some()
            && matches!(integration.imported_name.as_deref(), None | Some("checks"))
            && integration
                .call
                .as_ref()
                .and_then(|call| call.first_arg.as_ref())
                .is_some_and(checks_options_have_structured_data_custom_check)
    })
}

fn checks_options_are_fail_closed(value: &G3TsAstroStaticValue) -> bool {
    let Some(properties) = object_properties(value) else {
        return false;
    };
    if object_has_duplicate_keys(properties) {
        return false;
    }

    object_has_only_allowed_keys(properties, &NUASITE_CHECKS_ALLOWED_OPTION_KEYS)
        && property_string(properties, "mode") == Some("full")
        && property_bool(properties, "failOnError") == Some(true)
        && property_bool(properties, "failOnWarning") == Some(true)
        && property_bool(properties, "reportJson") == Some(true)
        && property_bool(properties, "ai") == Some(false)
        && overrides_absent_or_empty(properties)
        && validator_lane_not_disabled(properties, "seo")
        && validator_lane_not_disabled(properties, "geo")
        && validator_lane_not_disabled(properties, "performance")
        && validator_lane_not_disabled(properties, "accessibility")
        && checks_options_have_structured_data_custom_check(value)
}

fn checks_options_have_structured_data_custom_check(value: &G3TsAstroStaticValue) -> bool {
    let Some(properties) = object_properties(value) else {
        return false;
    };
    if object_has_duplicate_keys(properties) {
        return false;
    }
    let Some(G3TsAstroStaticValue::Array(values)) = property_value(properties, "customChecks")
    else {
        return false;
    };

    values.iter().any(|value| {
        matches!(
            value,
            G3TsAstroStaticValue::ImportedIdentifier {
                local_name,
                source_module: Some(source_module),
                imported_name: Some(imported_name),
            } if local_name == "structuredDataPresentCheck"
                && source_module == "g3ts-astro-nuasite-checks"
                && imported_name == "structuredDataPresentCheck"
        )
    })
}

fn object_properties(value: &G3TsAstroStaticValue) -> Option<&[G3TsAstroStaticObjectProperty]> {
    match value {
        G3TsAstroStaticValue::Object(properties) => Some(properties),
        G3TsAstroStaticValue::Bool(_)
        | G3TsAstroStaticValue::Number(_)
        | G3TsAstroStaticValue::String(_)
        | G3TsAstroStaticValue::Null
        | G3TsAstroStaticValue::Array(_)
        | G3TsAstroStaticValue::ImportedIdentifier { .. } => None,
    }
}

fn property_value<'a>(
    properties: &'a [G3TsAstroStaticObjectProperty],
    key: &str,
) -> Option<&'a G3TsAstroStaticValue> {
    properties
        .iter()
        .find(|property| property.key == key)
        .map(|property| &property.value)
}

fn object_has_duplicate_keys(properties: &[G3TsAstroStaticObjectProperty]) -> bool {
    let mut seen = std::collections::BTreeSet::new();
    properties
        .iter()
        .any(|property| !seen.insert(property.key.as_str()))
}

fn object_has_only_allowed_keys(
    properties: &[G3TsAstroStaticObjectProperty],
    allowed_keys: &[&str],
) -> bool {
    properties
        .iter()
        .all(|property| allowed_keys.contains(&property.key.as_str()))
}

fn property_string<'a>(
    properties: &'a [G3TsAstroStaticObjectProperty],
    key: &str,
) -> Option<&'a str> {
    match property_value(properties, key) {
        Some(G3TsAstroStaticValue::String(value)) => Some(value),
        _ => None,
    }
}

fn property_bool(properties: &[G3TsAstroStaticObjectProperty], key: &str) -> Option<bool> {
    match property_value(properties, key) {
        Some(G3TsAstroStaticValue::Bool(value)) => Some(*value),
        _ => None,
    }
}

fn overrides_absent_or_empty(properties: &[G3TsAstroStaticObjectProperty]) -> bool {
    match property_value(properties, "overrides") {
        None => true,
        Some(G3TsAstroStaticValue::Object(properties)) => properties.is_empty(),
        Some(_) => false,
    }
}

fn validator_lane_not_disabled(properties: &[G3TsAstroStaticObjectProperty], key: &str) -> bool {
    match property_value(properties, key) {
        None => true,
        Some(G3TsAstroStaticValue::Bool(false)) => false,
        Some(G3TsAstroStaticValue::Bool(true) | G3TsAstroStaticValue::Object(_)) => true,
        Some(_) => false,
    }
}

#[must_use]
pub(crate) fn eslint_rel_path(contract: &G3TsAstroEslintPluginContractInput) -> Option<&str> {
    match &contract.config {
        G3TsAstroEslintSurfaceState::Missing { rel_path }
        | G3TsAstroEslintSurfaceState::Unreadable { rel_path, .. }
        | G3TsAstroEslintSurfaceState::ParseError { rel_path, .. } => Some(rel_path),
        G3TsAstroEslintSurfaceState::Parsed { snapshot } => Some(&snapshot.rel_path),
    }
}

#[must_use]
pub(crate) fn eslint_required_lanes_have_effective_pipeline_rules(
    contract: &G3TsAstroEslintPluginContractInput,
    plugin_name: &str,
    plugin_package_name: &str,
    required_rules: &[&str],
    route_scoped_rules: &[&str],
    content_data_rules: &[&str],
    content_source_rules: &[&str],
) -> bool {
    parsed_eslint_surface(contract).is_some_and(|snapshot| {
        lane_has_plugin_and_rules(
            snapshot.astro_source_probe_present,
            snapshot.astro_source_probe_ignored,
            &snapshot.astro_source_plugins,
            &snapshot.astro_source_plugin_meta_names,
            &snapshot.astro_source_plugin_package_names,
            &snapshot.astro_source_error_rules,
            Some(&snapshot.astro_source_effective_route_scoped_pipeline_rules),
            Some(&snapshot.astro_source_effective_content_data_pipeline_rules),
            Some(&snapshot.astro_source_effective_content_source_pipeline_rules),
            plugin_name,
            Some(plugin_package_name),
            required_rules,
            route_scoped_rules,
            content_data_rules,
            content_source_rules,
        ) && lane_has_plugin_and_rules(
            snapshot.ts_source_probe_present,
            snapshot.ts_source_probe_ignored,
            &snapshot.ts_source_plugins,
            &snapshot.ts_source_plugin_meta_names,
            &snapshot.ts_source_plugin_package_names,
            &snapshot.ts_source_error_rules,
            Some(&snapshot.ts_source_effective_route_scoped_pipeline_rules),
            Some(&snapshot.ts_source_effective_content_data_pipeline_rules),
            Some(&snapshot.ts_source_effective_content_source_pipeline_rules),
            plugin_name,
            Some(plugin_package_name),
            required_rules,
            route_scoped_rules,
            content_data_rules,
            content_source_rules,
        ) && lane_has_plugin_and_rules(
            snapshot.tsx_source_probe_present,
            snapshot.tsx_source_probe_ignored,
            &snapshot.tsx_source_plugins,
            &snapshot.tsx_source_plugin_meta_names,
            &snapshot.tsx_source_plugin_package_names,
            &snapshot.tsx_source_error_rules,
            Some(&snapshot.tsx_source_effective_route_scoped_pipeline_rules),
            Some(&snapshot.tsx_source_effective_content_data_pipeline_rules),
            Some(&snapshot.tsx_source_effective_content_source_pipeline_rules),
            plugin_name,
            Some(plugin_package_name),
            required_rules,
            route_scoped_rules,
            content_data_rules,
            content_source_rules,
        )
    })
}

#[must_use]
pub(crate) fn eslint_required_lanes_have_inline_public_content_rule(
    contract: &G3TsAstroEslintPluginContractInput,
    plugin_name: &str,
    rule_name: &str,
) -> bool {
    parsed_eslint_surface(contract).is_some_and(|snapshot| {
        lane_has_inline_public_content_rule(
            snapshot.astro_source_probe_present,
            snapshot.astro_source_probe_ignored,
            &snapshot.astro_source_plugins,
            &snapshot.astro_source_error_rules,
            &snapshot.astro_source_effective_inline_public_content_rules,
            plugin_name,
            rule_name,
        ) && lane_has_inline_public_content_rule(
            snapshot.ts_source_probe_present,
            snapshot.ts_source_probe_ignored,
            &snapshot.ts_source_plugins,
            &snapshot.ts_source_error_rules,
            &snapshot.ts_source_effective_inline_public_content_rules,
            plugin_name,
            rule_name,
        ) && lane_has_inline_public_content_rule(
            snapshot.tsx_source_probe_present,
            snapshot.tsx_source_probe_ignored,
            &snapshot.tsx_source_plugins,
            &snapshot.tsx_source_error_rules,
            &snapshot.tsx_source_effective_inline_public_content_rules,
            plugin_name,
            rule_name,
        )
    })
}

#[must_use]
pub(crate) fn eslint_astro_source_has_plugin(
    contract: &G3TsAstroEslintPluginContractInput,
    plugin_name: &str,
) -> bool {
    parsed_eslint_surface(contract).is_some_and(|snapshot| {
        lane_has_plugin_and_rules(
            snapshot.astro_source_probe_present,
            snapshot.astro_source_probe_ignored,
            &snapshot.astro_source_plugins,
            &BTreeMap::new(),
            &snapshot.astro_source_plugin_package_names,
            &snapshot.astro_source_error_rules,
            None,
            None,
            None,
            plugin_name,
            None,
            &[],
            &[],
            &[],
            &[],
        )
    })
}

#[must_use]
pub(crate) fn eslint_mdx_lane_has_remark_rule(
    contract: &G3TsAstroEslintPluginContractInput,
    plugin_name: &str,
    rule_name: &str,
) -> bool {
    parsed_eslint_surface(contract).is_some_and(|snapshot| {
        lane_has_plugin_and_rules(
            snapshot.mdx_content_probe_present,
            snapshot.mdx_content_probe_ignored,
            &snapshot.mdx_content_plugins,
            &BTreeMap::new(),
            &snapshot.mdx_content_plugin_package_names,
            &snapshot.mdx_content_error_rules,
            None,
            None,
            None,
            plugin_name,
            None,
            &[rule_name],
            &[],
            &[],
            &[],
        )
    })
}

fn lane_has_inline_public_content_rule(
    lane_present: bool,
    lane_ignored: bool,
    plugins: &[String],
    error_rules: &[String],
    effective_inline_public_content_rules: &[String],
    plugin_name: &str,
    rule_name: &str,
) -> bool {
    if !lane_present || lane_ignored {
        return false;
    }

    plugins.iter().any(|plugin| plugin == plugin_name)
        && error_rules.iter().any(|rule| rule == rule_name)
        && effective_inline_public_content_rules
            .iter()
            .any(|rule| rule == rule_name)
}

#[must_use]
pub(crate) fn info(id: &str, title: &str, message: String, file: &str) -> G3CheckResult {
    G3CheckResult::new(
        id.to_owned(),
        G3Severity::Info,
        title.to_owned(),
        message,
        Some(file.to_owned()),
        None,
    )
    .into_inventory()
}

#[must_use]
pub(crate) fn error(id: &str, title: &str, message: String, file: Option<&str>) -> G3CheckResult {
    G3CheckResult::new(
        id.to_owned(),
        G3Severity::Error,
        title.to_owned(),
        message,
        file.map(str::to_owned),
        None,
    )
}

fn lane_has_plugin_and_rules(
    lane_present: bool,
    lane_ignored: bool,
    plugins: &[String],
    _plugin_meta_names: &BTreeMap<String, String>,
    plugin_package_names: &BTreeMap<String, Vec<String>>,
    error_rules: &[String],
    effective_route_scoped_rules: Option<&[String]>,
    effective_content_data_rules: Option<&[String]>,
    effective_content_source_rules: Option<&[String]>,
    plugin_name: &str,
    plugin_package_name: Option<&str>,
    required_rules: &[&str],
    route_scoped_rules: &[&str],
    content_data_rules: &[&str],
    content_source_rules: &[&str],
) -> bool {
    if !lane_present || lane_ignored {
        return false;
    }

    if !plugins.iter().any(|plugin| plugin == plugin_name) {
        return false;
    }

    if let Some(plugin_package_name) = plugin_package_name {
        if !plugin_package_names
            .get(plugin_name)
            .is_some_and(|package_names| {
                package_names.iter().any(|name| name == plugin_package_name)
            })
        {
            return false;
        }
    }

    let enabled_rules = error_rules
        .iter()
        .map(String::as_str)
        .collect::<std::collections::BTreeSet<_>>();

    let effective_route_scope = effective_route_scoped_rules.map(|rules| {
        rules
            .iter()
            .map(String::as_str)
            .collect::<std::collections::BTreeSet<_>>()
    });
    let effective_content_data_scope = effective_content_data_rules.map(|rules| {
        rules
            .iter()
            .map(String::as_str)
            .collect::<std::collections::BTreeSet<_>>()
    });
    let effective_content_source_scope = effective_content_source_rules.map(|rules| {
        rules
            .iter()
            .map(String::as_str)
            .collect::<std::collections::BTreeSet<_>>()
    });

    required_rules
        .iter()
        .all(|required_rule| enabled_rules.contains(*required_rule))
        && route_scoped_rules.iter().all(|required_rule| {
            effective_route_scope
                .as_ref()
                .is_none_or(|effective_rules| effective_rules.contains(*required_rule))
        })
        && content_data_rules.iter().all(|required_rule| {
            effective_content_data_scope
                .as_ref()
                .is_none_or(|effective_rules| effective_rules.contains(*required_rule))
        })
        && content_source_rules.iter().all(|required_rule| {
            effective_content_source_scope
                .as_ref()
                .is_none_or(|effective_rules| effective_rules.contains(*required_rule))
        })
}
