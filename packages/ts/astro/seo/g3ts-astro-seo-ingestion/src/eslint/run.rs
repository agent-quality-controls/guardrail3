use super::rule_helpers::{
    probe_has_pipeline_plugin_package, rule_setting_has_expected_module_globs,
    rule_setting_is_error, rule_setting_option_globs_are_valid,
    rule_setting_option_globs_match_any_path,
};
use g3_workspace_crawl::G3RsWorkspaceCrawl as G3WorkspaceCrawl;
use g3ts_astro_check_support::surfaces::{
    G3TsAstroRawEslintConfigState, read_eslint_config_surface,
};
use g3ts_astro_seo_types::{
    G3TsAstroSeoEslintSurfaceSnapshot, G3TsAstroSeoEslintSurfaceState,
    G3TsAstroSeoPolicySurfaceState,
};

/// `METADATA_HELPER_PIPELINE_RULE` constant.
const METADATA_HELPER_PIPELINE_RULE: &str =
    "astro-pipeline/require-approved-metadata-helper-in-routes";
/// `JSON_LD_HELPER_PIPELINE_RULE` constant.
const JSON_LD_HELPER_PIPELINE_RULE: &str =
    "astro-pipeline/require-approved-json-ld-helper-in-routes";
/// Required option-name/value pair for a route-helper rule.
type RequiredModuleOption<'a> = (&'a str, &'a [String]);

#[must_use]
/// `ingest_seo_eslint_surface`: ingest seo eslint surface.
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

    let parsed_snapshot = build_eslint_snapshot(
        rel_path,
        astro,
        ts,
        tsx,
        &route_page_paths,
        &endpoint_paths,
        &metadata_helpers,
        &json_ld_helpers,
    );
    G3TsAstroSeoEslintSurfaceState::Parsed {
        snapshot: parsed_snapshot,
    }
}

/// Builds a `G3TsAstroSeoEslintSurfaceSnapshot` from the three per-lane probes and the policy helper lists.
#[expect(
    clippy::too_many_arguments,
    reason = "snapshot builder mirrors per-lane probe inputs and policy collections; collapsing into a struct would couple unrelated lane configs"
)]
fn build_eslint_snapshot(
    rel_path: String,
    astro: Option<&eslint_config_parser::types::EslintEffectiveConfigProbe>,
    ts: Option<&eslint_config_parser::types::EslintEffectiveConfigProbe>,
    tsx: Option<&eslint_config_parser::types::EslintEffectiveConfigProbe>,
    route_page_paths: &[String],
    endpoint_paths: &[String],
    metadata_helpers: &[String],
    json_ld_helpers: &[String],
) -> G3TsAstroSeoEslintSurfaceSnapshot {
    G3TsAstroSeoEslintSurfaceSnapshot {
        rel_path,
        astro_source_probe_present: astro.is_some(),
        ts_source_probe_present: ts.is_some(),
        tsx_source_probe_present: tsx.is_some(),
        astro_source_effective_metadata_helper_rules: effective_metadata_helper_rules(
            astro,
            route_page_paths,
            endpoint_paths,
            metadata_helpers,
        ),
        ts_source_effective_metadata_helper_rules: effective_metadata_helper_rules(
            ts,
            route_page_paths,
            endpoint_paths,
            metadata_helpers,
        ),
        tsx_source_effective_metadata_helper_rules: effective_metadata_helper_rules(
            tsx,
            route_page_paths,
            endpoint_paths,
            metadata_helpers,
        ),
        astro_source_effective_json_ld_helper_rules: effective_json_ld_helper_rules(
            astro,
            route_page_paths,
            endpoint_paths,
            json_ld_helpers,
        ),
        ts_source_effective_json_ld_helper_rules: effective_json_ld_helper_rules(
            ts,
            route_page_paths,
            endpoint_paths,
            json_ld_helpers,
        ),
        tsx_source_effective_json_ld_helper_rules: effective_json_ld_helper_rules(
            tsx,
            route_page_paths,
            endpoint_paths,
            json_ld_helpers,
        ),
        astro_source_warn_or_error_rules: crate::eslint_suppression::active_warn_or_error_rules(
            astro,
        ),
        ts_source_warn_or_error_rules: crate::eslint_suppression::active_warn_or_error_rules(ts),
        tsx_source_warn_or_error_rules: crate::eslint_suppression::active_warn_or_error_rules(tsx),
        astro_source_restricted_disable_patterns:
            crate::eslint_suppression::restricted_disable_patterns(astro),
        ts_source_restricted_disable_patterns:
            crate::eslint_suppression::restricted_disable_patterns(ts),
        tsx_source_restricted_disable_patterns:
            crate::eslint_suppression::restricted_disable_patterns(tsx),
    }
}

/// `probe_targets`: probe targets.
fn probe_targets(
    crawl: &G3WorkspaceCrawl,
    app_root_rel_path: &str,
) -> Vec<eslint_config_parser::types::EslintProbeTarget> {
    vec![
        source_probe(
            crawl,
            app_root_rel_path,
            eslint_config_parser::types::EslintProbeKind::AstroSource,
            |rel_path| rel_path.starts_with("src/") && has_extension(rel_path, "astro"),
            "src/__g3ts_probe__.astro",
        ),
        source_probe(
            crawl,
            app_root_rel_path,
            eslint_config_parser::types::EslintProbeKind::TsSource,
            |rel_path| rel_path.starts_with("src/") && has_extension(rel_path, "ts"),
            "src/index.ts",
        ),
        source_probe(
            crawl,
            app_root_rel_path,
            eslint_config_parser::types::EslintProbeKind::TsxSource,
            |rel_path| rel_path.starts_with("src/") && has_extension(rel_path, "tsx"),
            "src/__g3ts_probe__.tsx",
        ),
    ]
}

/// `source_probe`: source probe.
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

/// `first_matching_app_rel_path`: first matching app rel path.
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

/// `map_raw_state`: map raw state.
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

/// `active_probe`: active probe.
fn active_probe(
    typed: &eslint_config_parser::types::EslintConfigSnapshot,
    kind: eslint_config_parser::types::EslintProbeKind,
) -> Option<&eslint_config_parser::types::EslintEffectiveConfigProbe> {
    probe_by_kind(typed, kind).filter(|probe| !probe.ignored)
}

/// `probe_by_kind`: probe by kind.
fn probe_by_kind(
    typed: &eslint_config_parser::types::EslintConfigSnapshot,
    kind: eslint_config_parser::types::EslintProbeKind,
) -> Option<&eslint_config_parser::types::EslintEffectiveConfigProbe> {
    typed.probes.iter().find(|probe| probe.probe == kind)
}

/// `effective_metadata_helper_rules`: effective metadata helper rules.
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

/// `effective_json_ld_helper_rules`: effective json ld helper rules.
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

/// `effective_route_helper_rules`: effective route helper rules.
fn effective_route_helper_rules(
    probe: Option<&eslint_config_parser::types::EslintEffectiveConfigProbe>,
    route_page_paths: &[String],
    endpoint_paths: &[String],
    rule_name: &str,
    required_module_options: &[RequiredModuleOption<'_>],
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

/// `seo_metadata_helper_policy_paths`: seo metadata helper policy paths.
fn seo_metadata_helper_policy_paths(astro_policy: &G3TsAstroSeoPolicySurfaceState) -> Vec<String> {
    let G3TsAstroSeoPolicySurfaceState::Parsed { snapshot } = astro_policy else {
        return Vec::new();
    };

    snapshot.metadata_helpers.clone()
}

/// `seo_json_ld_helper_policy_paths`: seo json ld helper policy paths.
fn seo_json_ld_helper_policy_paths(astro_policy: &G3TsAstroSeoPolicySurfaceState) -> Vec<String> {
    let G3TsAstroSeoPolicySurfaceState::Parsed { snapshot } = astro_policy else {
        return Vec::new();
    };

    snapshot.json_ld_helpers.clone()
}

/// `route_page_paths`: route page paths.
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

/// `endpoint_paths`: endpoint paths.
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

/// `is_included_file`: is included file.
fn is_included_file(entry: &g3_workspace_crawl::G3RsWorkspaceEntry) -> bool {
    entry.kind == g3_workspace_crawl::G3RsWorkspaceEntryKind::File
        && entry.ignore_state == g3_workspace_crawl::G3RsWorkspaceIgnoreState::Included
}

/// Returns `true` when `rel_path` ends with the given extension (case-insensitive).
fn has_extension(rel_path: &str, extension: &str) -> bool {
    std::path::Path::new(rel_path)
        .extension()
        .is_some_and(|value| value.eq_ignore_ascii_case(extension))
}

/// `is_route_page_file`: is route page file.
fn is_route_page_file(rel_path: &str) -> bool {
    matches!(
        std::path::Path::new(rel_path)
            .extension()
            .and_then(std::ffi::OsStr::to_str),
        Some(ext) if ext.eq_ignore_ascii_case("astro")
            || ext.eq_ignore_ascii_case("md")
            || ext.eq_ignore_ascii_case("mdx")
            || ext.eq_ignore_ascii_case("html")
    )
}

/// `is_endpoint_file`: is endpoint file.
fn is_endpoint_file(rel_path: &str) -> bool {
    if rel_path.to_ascii_lowercase().ends_with(".d.ts") {
        return false;
    }
    has_extension(rel_path, "js") || has_extension(rel_path, "ts")
}

/// `rule_setting_has_route_and_endpoint_coverage`: rule setting has route and endpoint coverage.
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
