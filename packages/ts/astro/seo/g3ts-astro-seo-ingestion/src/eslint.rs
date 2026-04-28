use g3_workspace_crawl::G3RsWorkspaceCrawl as G3WorkspaceCrawl;
use g3ts_astro_check_support::surfaces::{
    G3TsAstroRawEslintConfigState, read_eslint_config_surface,
};
use g3ts_astro_seo_types::{
    G3TsAstroSeoEslintSurfaceSnapshot, G3TsAstroSeoEslintSurfaceState,
    G3TsAstroSeoPolicySurfaceState,
};
use std::collections::BTreeSet;

const METADATA_HELPER_PIPELINE_RULE: &str =
    "astro-pipeline/require-approved-metadata-helper-in-routes";
const JSON_LD_HELPER_PIPELINE_RULE: &str =
    "astro-pipeline/require-approved-json-ld-helper-in-routes";
const SOURCE_MODULE_EXTENSIONS: [&str; 9] = [
    ".ts", ".tsx", ".js", ".jsx", ".mts", ".cts", ".mjs", ".cjs", ".astro",
];

#[must_use]
pub(crate) fn ingest_seo_eslint_surface(
    crawl: &G3WorkspaceCrawl,
    app_root_rel_path: &str,
    astro_policy: &G3TsAstroSeoPolicySurfaceState,
) -> G3TsAstroSeoEslintSurfaceState {
    let raw = read_eslint_config_surface(
        crawl,
        app_root_rel_path,
        &probe_targets(crawl, app_root_rel_path),
    );
    let G3TsAstroRawEslintConfigState::Parsed { rel_path, snapshot } = raw else {
        return map_raw_state(raw);
    };

    let route_page_paths = route_page_paths(crawl, app_root_rel_path);
    let endpoint_paths = endpoint_paths(crawl, app_root_rel_path);
    let metadata_helpers = seo_metadata_helper_policy_paths(astro_policy);
    let json_ld_helpers = seo_json_ld_helper_policy_paths(astro_policy);
    let astro = active_probe(
        &snapshot,
        eslint_config_parser::types::EslintProbeKind::AstroSource,
    );
    let ts = active_probe(
        &snapshot,
        eslint_config_parser::types::EslintProbeKind::TsSource,
    );
    let tsx = active_probe(
        &snapshot,
        eslint_config_parser::types::EslintProbeKind::TsxSource,
    );

    G3TsAstroSeoEslintSurfaceState::Parsed {
        snapshot: G3TsAstroSeoEslintSurfaceSnapshot {
            rel_path,
            astro_source_probe_present: astro.is_some(),
            ts_source_probe_present: ts.is_some(),
            tsx_source_probe_present: tsx.is_some(),
            astro_source_effective_metadata_helper_rules: effective_metadata_helper_rules(
                astro,
                &route_page_paths,
                &endpoint_paths,
                &metadata_helpers,
            ),
            ts_source_effective_metadata_helper_rules: effective_metadata_helper_rules(
                ts,
                &route_page_paths,
                &endpoint_paths,
                &metadata_helpers,
            ),
            tsx_source_effective_metadata_helper_rules: effective_metadata_helper_rules(
                tsx,
                &route_page_paths,
                &endpoint_paths,
                &metadata_helpers,
            ),
            astro_source_effective_json_ld_helper_rules: effective_json_ld_helper_rules(
                astro,
                &route_page_paths,
                &endpoint_paths,
                &json_ld_helpers,
            ),
            ts_source_effective_json_ld_helper_rules: effective_json_ld_helper_rules(
                ts,
                &route_page_paths,
                &endpoint_paths,
                &json_ld_helpers,
            ),
            tsx_source_effective_json_ld_helper_rules: effective_json_ld_helper_rules(
                tsx,
                &route_page_paths,
                &endpoint_paths,
                &json_ld_helpers,
            ),
            astro_source_warn_or_error_rules: crate::eslint_suppression::active_warn_or_error_rules(
                astro,
            ),
            ts_source_warn_or_error_rules: crate::eslint_suppression::active_warn_or_error_rules(
                ts,
            ),
            tsx_source_warn_or_error_rules: crate::eslint_suppression::active_warn_or_error_rules(
                tsx,
            ),
            astro_source_restricted_disable_patterns:
                crate::eslint_suppression::restricted_disable_patterns(astro),
            ts_source_restricted_disable_patterns:
                crate::eslint_suppression::restricted_disable_patterns(ts),
            tsx_source_restricted_disable_patterns:
                crate::eslint_suppression::restricted_disable_patterns(tsx),
        },
    }
}

fn probe_targets(
    crawl: &G3WorkspaceCrawl,
    app_root_rel_path: &str,
) -> Vec<eslint_config_parser::types::EslintProbeTarget> {
    vec![
        source_probe(
            crawl,
            app_root_rel_path,
            eslint_config_parser::types::EslintProbeKind::AstroSource,
            |rel_path| rel_path.starts_with("src/") && rel_path.ends_with(".astro"),
            "src/__g3ts_probe__.astro",
        ),
        source_probe(
            crawl,
            app_root_rel_path,
            eslint_config_parser::types::EslintProbeKind::TsSource,
            |rel_path| rel_path.starts_with("src/") && rel_path.ends_with(".ts"),
            "src/index.ts",
        ),
        source_probe(
            crawl,
            app_root_rel_path,
            eslint_config_parser::types::EslintProbeKind::TsxSource,
            |rel_path| rel_path.starts_with("src/") && rel_path.ends_with(".tsx"),
            "src/__g3ts_probe__.tsx",
        ),
    ]
}

fn source_probe(
    crawl: &G3WorkspaceCrawl,
    app_root_rel_path: &str,
    probe: eslint_config_parser::types::EslintProbeKind,
    predicate: impl Fn(&str) -> bool,
    fallback_rel_path: &str,
) -> eslint_config_parser::types::EslintProbeTarget {
    eslint_config_parser::types::EslintProbeTarget {
        probe,
        rel_path: first_matching_app_rel_path(crawl, app_root_rel_path, predicate).unwrap_or_else(
            || {
                g3ts_astro_check_support::surfaces::scoped_rel_path(
                    app_root_rel_path,
                    fallback_rel_path,
                )
            },
        ),
    }
}

fn first_matching_app_rel_path(
    crawl: &G3WorkspaceCrawl,
    app_root_rel_path: &str,
    predicate: impl Fn(&str) -> bool,
) -> Option<String> {
    crawl
        .entries
        .iter()
        .find(|entry| {
            entry.kind == g3_workspace_crawl::G3RsWorkspaceEntryKind::File
                && entry.ignore_state == g3_workspace_crawl::G3RsWorkspaceIgnoreState::Included
                && g3ts_astro_check_support::surfaces::is_under_app_root(
                    &entry.path.rel_path,
                    app_root_rel_path,
                )
                && predicate(&g3ts_astro_check_support::surfaces::app_relative_path(
                    &entry.path.rel_path,
                    app_root_rel_path,
                ))
        })
        .map(|entry| entry.path.rel_path.clone())
}

fn map_raw_state(raw: G3TsAstroRawEslintConfigState) -> G3TsAstroSeoEslintSurfaceState {
    match raw {
        G3TsAstroRawEslintConfigState::Missing { rel_path } => {
            G3TsAstroSeoEslintSurfaceState::Missing { rel_path }
        }
        G3TsAstroRawEslintConfigState::Unreadable { rel_path, reason } => {
            G3TsAstroSeoEslintSurfaceState::Unreadable { rel_path, reason }
        }
        G3TsAstroRawEslintConfigState::ParseError { rel_path, reason } => {
            G3TsAstroSeoEslintSurfaceState::ParseError { rel_path, reason }
        }
        G3TsAstroRawEslintConfigState::Parsed { rel_path, .. } => {
            G3TsAstroSeoEslintSurfaceState::ParseError {
                rel_path,
                reason: "parsed raw eslint state reached seo error mapper".to_owned(),
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

fn effective_metadata_helper_rules(
    probe: Option<&eslint_config_parser::types::EslintEffectiveConfigProbe>,
    route_page_paths: &[String],
    endpoint_paths: &[String],
    approved_metadata_helpers: &[String],
) -> Vec<String> {
    let required_modules = [("approvedMetadataHelperModules", approved_metadata_helpers)];
    effective_route_helper_rules(
        probe,
        route_page_paths,
        endpoint_paths,
        METADATA_HELPER_PIPELINE_RULE,
        &required_modules,
    )
}

fn effective_json_ld_helper_rules(
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

fn effective_route_helper_rules(
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

fn seo_metadata_helper_policy_paths(astro_policy: &G3TsAstroSeoPolicySurfaceState) -> Vec<String> {
    let G3TsAstroSeoPolicySurfaceState::Parsed { snapshot } = astro_policy else {
        return Vec::new();
    };

    snapshot.metadata_helpers.clone()
}

fn seo_json_ld_helper_policy_paths(astro_policy: &G3TsAstroSeoPolicySurfaceState) -> Vec<String> {
    let G3TsAstroSeoPolicySurfaceState::Parsed { snapshot } = astro_policy else {
        return Vec::new();
    };

    snapshot.json_ld_helpers.clone()
}

fn route_page_paths(crawl: &G3WorkspaceCrawl, app_root_rel_path: &str) -> Vec<String> {
    let pages_prefix =
        g3ts_astro_check_support::surfaces::scoped_rel_path(app_root_rel_path, "src/pages/");

    crawl
        .entries
        .iter()
        .filter(|entry| {
            is_included_file(entry)
                && entry.path.rel_path.starts_with(&pages_prefix)
                && is_route_page_file(&entry.path.rel_path)
        })
        .map(|entry| {
            g3ts_astro_check_support::surfaces::app_relative_path(
                &entry.path.rel_path,
                app_root_rel_path,
            )
        })
        .collect()
}

fn endpoint_paths(crawl: &G3WorkspaceCrawl, app_root_rel_path: &str) -> Vec<String> {
    let pages_prefix =
        g3ts_astro_check_support::surfaces::scoped_rel_path(app_root_rel_path, "src/pages/");

    crawl
        .entries
        .iter()
        .filter(|entry| {
            is_included_file(entry)
                && entry.path.rel_path.starts_with(&pages_prefix)
                && is_endpoint_file(&entry.path.rel_path)
        })
        .map(|entry| {
            g3ts_astro_check_support::surfaces::app_relative_path(
                &entry.path.rel_path,
                app_root_rel_path,
            )
        })
        .collect()
}

fn is_included_file(entry: &g3_workspace_crawl::G3RsWorkspaceEntry) -> bool {
    entry.kind == g3_workspace_crawl::G3RsWorkspaceEntryKind::File
        && entry.ignore_state == g3_workspace_crawl::G3RsWorkspaceIgnoreState::Included
}

fn is_route_page_file(rel_path: &str) -> bool {
    rel_path.ends_with(".astro")
        || rel_path.ends_with(".md")
        || rel_path.ends_with(".mdx")
        || rel_path.ends_with(".html")
}

fn is_endpoint_file(rel_path: &str) -> bool {
    (rel_path.ends_with(".js") || rel_path.ends_with(".ts")) && !rel_path.ends_with(".d.ts")
}

fn rule_setting_has_route_and_endpoint_coverage(
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
