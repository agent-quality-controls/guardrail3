use crate::core::parsed_eslint_surface;
use g3ts_astro_types::{G3TsAstroEslintPluginContractInput, G3TsAstroEslintSurfaceState};
use std::collections::BTreeMap;

#[must_use]
pub fn eslint_rel_path(contract: &G3TsAstroEslintPluginContractInput) -> Option<&str> {
    match &contract.config {
        G3TsAstroEslintSurfaceState::Missing { rel_path }
        | G3TsAstroEslintSurfaceState::Unreadable { rel_path, .. }
        | G3TsAstroEslintSurfaceState::ParseError { rel_path, .. } => Some(rel_path),
        G3TsAstroEslintSurfaceState::Parsed { snapshot } => Some(&snapshot.rel_path),
    }
}

#[must_use]
pub fn eslint_required_lanes_have_effective_pipeline_rules(
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
pub fn eslint_required_lanes_have_content_adapter_modules(
    contract: &G3TsAstroEslintPluginContractInput,
    expected_policy_adapters: &[String],
) -> bool {
    let expected_modules = expected_module_globs(expected_policy_adapters);
    !expected_modules.is_empty()
        && parsed_eslint_surface(contract).is_some_and(|snapshot| {
            string_arrays_match_as_sets(
                &snapshot.astro_source_effective_content_adapter_modules,
                &expected_modules,
            ) && string_arrays_match_as_sets(
                &snapshot.ts_source_effective_content_adapter_modules,
                &expected_modules,
            ) && string_arrays_match_as_sets(
                &snapshot.tsx_source_effective_content_adapter_modules,
                &expected_modules,
            )
        })
}

#[must_use]
pub fn eslint_required_lanes_have_inline_public_content_rule(
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
pub fn eslint_astro_source_has_plugin(
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
pub fn eslint_mdx_lane_has_remark_rule(
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

fn expected_module_globs(source_paths: &[String]) -> Vec<String> {
    let mut globs = source_paths
        .iter()
        .map(|source_path| {
            let source_path = source_path.trim_end_matches('/');
            if is_source_module_file(source_path) {
                normalize_glob(source_path)
            } else {
                format!("{}/**/*", normalize_glob(source_path))
            }
        })
        .collect::<Vec<_>>();
    globs.sort();
    globs.dedup();
    globs
}

fn is_source_module_file(path: &str) -> bool {
    path.ends_with(".ts")
        || path.ends_with(".tsx")
        || path.ends_with(".mts")
        || path.ends_with(".cts")
        || path.ends_with(".js")
        || path.ends_with(".jsx")
        || path.ends_with(".mjs")
        || path.ends_with(".cjs")
        || path.ends_with(".astro")
}

fn normalize_glob(value: &str) -> String {
    value
        .trim_start_matches("./")
        .trim_end_matches('/')
        .to_owned()
}

fn string_arrays_match_as_sets(left: &[String], right: &[String]) -> bool {
    std::collections::BTreeSet::from_iter(left.iter().map(|value| normalize_glob(value)))
        == std::collections::BTreeSet::from_iter(right.iter().map(|value| normalize_glob(value)))
}
