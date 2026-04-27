use g3_workspace_crawl::G3RsWorkspaceCrawl as G3WorkspaceCrawl;
use g3ts_astro_check_support::surfaces::{
    G3TsAstroRawEslintConfigState, read_eslint_config_surface,
};
use g3ts_astro_mdx_types::{
    G3TsAstroMdxEslintSurfaceSnapshot, G3TsAstroMdxEslintSurfaceState,
    G3TsAstroMdxPolicySurfaceState,
};
use std::collections::BTreeSet;

const MDX_COMPONENT_MAP_PIPELINE_RULE: &str =
    "astro-pipeline/mdx-component-imports-from-approved-map";
const SOURCE_MODULE_EXTENSIONS: [&str; 9] = [
    ".ts", ".tsx", ".js", ".jsx", ".mts", ".cts", ".mjs", ".cjs", ".astro",
];

#[must_use]
pub(crate) fn ingest_mdx_eslint_surface(
    crawl: &G3WorkspaceCrawl,
    app_root_rel_path: &str,
    astro_policy: &G3TsAstroMdxPolicySurfaceState,
) -> G3TsAstroMdxEslintSurfaceState {
    let raw = read_eslint_config_surface(
        crawl,
        app_root_rel_path,
        &probe_targets(crawl, app_root_rel_path, astro_policy),
    );
    let G3TsAstroRawEslintConfigState::Parsed { rel_path, snapshot } = raw else {
        return map_raw_state(raw);
    };

    let mdx_probe = active_probe(
        &snapshot,
        eslint_config_parser::types::EslintProbeKind::MdxContent,
    );
    let mdx_paths = mdx_content_paths(crawl, app_root_rel_path, astro_policy);
    let component_maps = mdx_component_map_policy_paths(astro_policy);

    G3TsAstroMdxEslintSurfaceState::Parsed {
        snapshot: G3TsAstroMdxEslintSurfaceSnapshot {
            rel_path,
            mdx_content_probe_present: mdx_probe.is_some(),
            mdx_content_plugins: plugins(mdx_probe),
            mdx_content_plugin_package_names: plugin_package_names(mdx_probe),
            mdx_content_error_rules: active_error_rules(mdx_probe),
            mdx_content_effective_mdx_component_map_rules: effective_mdx_component_map_rules(
                mdx_probe,
                &mdx_paths,
                &component_maps,
            ),
            mdx_content_probe_ignored: probe_ignored(
                &snapshot,
                eslint_config_parser::types::EslintProbeKind::MdxContent,
            ),
        },
    }
}

fn probe_targets(
    crawl: &G3WorkspaceCrawl,
    app_root_rel_path: &str,
    astro_policy: &G3TsAstroMdxPolicySurfaceState,
) -> Vec<eslint_config_parser::types::EslintProbeTarget> {
    vec![eslint_config_parser::types::EslintProbeTarget {
        probe: eslint_config_parser::types::EslintProbeKind::MdxContent,
        rel_path: mdx_content_paths(crawl, app_root_rel_path, astro_policy)
            .into_iter()
            .next()
            .unwrap_or_else(|| {
                g3ts_astro_check_support::surfaces::scoped_rel_path(
                    app_root_rel_path,
                    "content/__g3ts_probe__.mdx",
                )
            }),
    }]
}

fn map_raw_state(raw: G3TsAstroRawEslintConfigState) -> G3TsAstroMdxEslintSurfaceState {
    match raw {
        G3TsAstroRawEslintConfigState::Missing { rel_path } => {
            G3TsAstroMdxEslintSurfaceState::Missing { rel_path }
        }
        G3TsAstroRawEslintConfigState::Unreadable { rel_path, reason } => {
            G3TsAstroMdxEslintSurfaceState::Unreadable { rel_path, reason }
        }
        G3TsAstroRawEslintConfigState::ParseError { rel_path, reason } => {
            G3TsAstroMdxEslintSurfaceState::ParseError { rel_path, reason }
        }
        G3TsAstroRawEslintConfigState::Parsed { rel_path, .. } => {
            G3TsAstroMdxEslintSurfaceState::ParseError {
                rel_path,
                reason: "parsed raw eslint state reached mdx error mapper".to_owned(),
            }
        }
    }
}

fn active_probe(
    typed: &eslint_config_parser::types::EslintConfigSnapshot,
    kind: eslint_config_parser::types::EslintProbeKind,
) -> Option<&eslint_config_parser::types::EslintEffectiveConfigProbe> {
    probe_by_kind(typed, kind).filter(|probe| !probe.ignored)
}

fn probe_by_kind(
    typed: &eslint_config_parser::types::EslintConfigSnapshot,
    kind: eslint_config_parser::types::EslintProbeKind,
) -> Option<&eslint_config_parser::types::EslintEffectiveConfigProbe> {
    typed.probes.iter().find(|probe| probe.probe == kind)
}

fn probe_ignored(
    typed: &eslint_config_parser::types::EslintConfigSnapshot,
    kind: eslint_config_parser::types::EslintProbeKind,
) -> bool {
    probe_by_kind(typed, kind).is_none_or(|probe| probe.ignored)
}

fn plugins(probe: Option<&eslint_config_parser::types::EslintEffectiveConfigProbe>) -> Vec<String> {
    probe.map_or_else(Vec::new, |probe| probe.plugins.clone())
}

fn plugin_package_names(
    probe: Option<&eslint_config_parser::types::EslintEffectiveConfigProbe>,
) -> std::collections::BTreeMap<String, Vec<String>> {
    probe.map_or_else(std::collections::BTreeMap::new, |probe| {
        probe.plugin_package_names.clone()
    })
}

fn active_error_rules(
    probe: Option<&eslint_config_parser::types::EslintEffectiveConfigProbe>,
) -> Vec<String> {
    probe.map_or_else(Vec::new, |probe| {
        probe
            .rules
            .iter()
            .filter_map(|(rule_name, setting)| {
                (setting.severity == eslint_config_parser::types::EslintRuleSeverity::Error)
                    .then_some(rule_name.clone())
            })
            .collect()
    })
}

fn effective_mdx_component_map_rules(
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

fn mdx_content_paths(
    crawl: &G3WorkspaceCrawl,
    app_root_rel_path: &str,
    astro_policy: &G3TsAstroMdxPolicySurfaceState,
) -> Vec<String> {
    let content_root = match astro_policy {
        G3TsAstroMdxPolicySurfaceState::Parsed { snapshot } => snapshot.content_root.as_deref(),
        G3TsAstroMdxPolicySurfaceState::Missing { .. }
        | G3TsAstroMdxPolicySurfaceState::Unreadable { .. }
        | G3TsAstroMdxPolicySurfaceState::ParseError { .. }
        | G3TsAstroMdxPolicySurfaceState::MissingAstroPolicy { .. } => None,
    }
    .unwrap_or("src/content")
    .trim_end_matches('/');
    let scoped_content_root =
        g3ts_astro_check_support::surfaces::scoped_rel_path(app_root_rel_path, content_root);
    let scoped_prefix = format!("{scoped_content_root}/");

    crawl
        .entries
        .iter()
        .filter(|entry| {
            entry.kind == g3_workspace_crawl::G3RsWorkspaceEntryKind::File
                && entry.ignore_state == g3_workspace_crawl::G3RsWorkspaceIgnoreState::Included
                && entry.path.rel_path.starts_with(&scoped_prefix)
                && entry.path.rel_path.ends_with(".mdx")
        })
        .map(|entry| {
            g3ts_astro_check_support::surfaces::app_relative_path(
                &entry.path.rel_path,
                app_root_rel_path,
            )
        })
        .collect()
}

fn mdx_component_map_policy_paths(astro_policy: &G3TsAstroMdxPolicySurfaceState) -> Vec<String> {
    let G3TsAstroMdxPolicySurfaceState::Parsed { snapshot } = astro_policy else {
        return Vec::new();
    };

    snapshot.mdx_component_maps.clone()
}

fn rule_setting_has_expected_module_globs(
    setting: &eslint_config_parser::types::EslintRuleSetting,
    key: &str,
    expected_sources: &[String],
) -> bool {
    let expected = expected_module_globs(expected_sources);
    !expected.is_empty()
        && string_arrays_match_as_sets(&string_array_option(setting, key), &expected)
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

fn string_arrays_match_as_sets(left: &[String], right: &[String]) -> bool {
    BTreeSet::from_iter(left.iter().map(|value| normalize_glob(value)))
        == BTreeSet::from_iter(right.iter().map(|value| normalize_glob(value)))
}

fn rule_setting_has_option_globs_coverage(
    setting: &eslint_config_parser::types::EslintRuleSetting,
    key: &str,
    candidate_paths: &[String],
) -> bool {
    if candidate_paths.is_empty() {
        return rule_setting_option_globs_are_valid(setting, key);
    }

    rule_setting_option_globs_match_any_path(setting, key, candidate_paths)
}

fn rule_setting_option_globs_match_any_path(
    setting: &eslint_config_parser::types::EslintRuleSetting,
    option_name: &str,
    candidate_paths: &[String],
) -> bool {
    first_option_object(setting).is_some_and(|object| {
        non_empty_string_array_option(object.get(option_name))
            .is_some_and(|patterns| all_paths_match_globs(&patterns, candidate_paths))
    })
}

fn rule_setting_option_globs_are_valid(
    setting: &eslint_config_parser::types::EslintRuleSetting,
    option_name: &str,
) -> bool {
    first_option_object(setting).is_some_and(|object| {
        non_empty_string_array_option(object.get(option_name))
            .is_some_and(|patterns| globs_are_valid(&patterns))
    })
}

fn first_option_object(
    setting: &eslint_config_parser::types::EslintRuleSetting,
) -> Option<&serde_json::Map<String, serde_json::Value>> {
    setting
        .options
        .first()
        .and_then(serde_json::Value::as_object)
}

fn rule_setting_is_error(setting: &eslint_config_parser::types::EslintRuleSetting) -> bool {
    setting.severity == eslint_config_parser::types::EslintRuleSeverity::Error
}

fn probe_has_pipeline_plugin_package(
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

fn non_empty_string_array_option(option: Option<&serde_json::Value>) -> Option<Vec<&str>> {
    let values = option.and_then(serde_json::Value::as_array)?;

    if values.is_empty() {
        return None;
    }

    let mut strings = Vec::with_capacity(values.len());

    for value in values {
        let text = value.as_str()?.trim();
        if text.is_empty() {
            return None;
        }
        strings.push(text);
    }

    Some(strings)
}

fn string_array_option(
    setting: &eslint_config_parser::types::EslintRuleSetting,
    option_name: &str,
) -> Vec<String> {
    first_option_object(setting)
        .and_then(|object| object.get(option_name))
        .and_then(serde_json::Value::as_array)
        .map_or_else(Vec::new, |values| {
            values
                .iter()
                .filter_map(serde_json::Value::as_str)
                .map(str::trim)
                .filter(|value| !value.is_empty())
                .map(str::to_owned)
                .collect()
        })
}

fn all_paths_match_globs(patterns: &[&str], candidate_paths: &[String]) -> bool {
    let mut builder = globset::GlobSetBuilder::new();

    for pattern in patterns {
        let Ok(glob) = globset::Glob::new(&normalize_glob(pattern)) else {
            return false;
        };
        let _ = builder.add(glob);
    }

    let Ok(glob_set) = builder.build() else {
        return false;
    };

    candidate_paths
        .iter()
        .all(|candidate_path| glob_set.is_match(normalize_glob(candidate_path)))
}

fn globs_are_valid(patterns: &[&str]) -> bool {
    let mut builder = globset::GlobSetBuilder::new();

    for pattern in patterns {
        let Ok(glob) = globset::Glob::new(&normalize_glob(pattern)) else {
            return false;
        };
        let _ = builder.add(glob);
    }

    builder.build().is_ok()
}

fn normalize_glob(value: &str) -> String {
    let mut normalized = value.replace('\\', "/");
    while normalized.contains("//") {
        normalized = normalized.replace("//", "/");
    }
    normalized.trim_start_matches("./").to_owned()
}

fn is_source_module_file(rel_path: &str) -> bool {
    SOURCE_MODULE_EXTENSIONS
        .iter()
        .any(|extension| rel_path.ends_with(extension))
}
