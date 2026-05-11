use g3_workspace_crawl::G3WorkspaceCrawl;
use g3ts_astro_check_support::surfaces::{
    G3TsAstroRawEslintConfigState, read_eslint_config_surface,
};
use g3ts_astro_setup_types::{
    G3TsAstroSetupEslintSurfaceSnapshot, G3TsAstroSetupEslintSurfaceState,
};
use std::collections::BTreeMap;

#[must_use]
/// `ingest_setup_eslint_surface`: ingest setup eslint surface.
pub(crate) fn ingest_setup_eslint_surface(
    crawl: &G3WorkspaceCrawl,
    app_root_rel_path: &str,
) -> G3TsAstroSetupEslintSurfaceState {
    let raw = read_eslint_config_surface(
        crawl,
        app_root_rel_path,
        &probe_targets(crawl, app_root_rel_path),
    );
    let G3TsAstroRawEslintConfigState::Parsed { rel_path, snapshot } = raw else {
        return map_raw_state(raw);
    };

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

    G3TsAstroSetupEslintSurfaceState::Parsed {
        snapshot: G3TsAstroSetupEslintSurfaceSnapshot {
            rel_path,
            astro_source_probe_present: astro.is_some(),
            ts_source_probe_present: ts.is_some(),
            tsx_source_probe_present: tsx.is_some(),
            astro_source_plugins: plugins(astro),
            ts_source_plugins: plugins(ts),
            tsx_source_plugins: plugins(tsx),
            astro_source_plugin_meta_names: plugin_meta_names(astro),
            ts_source_plugin_meta_names: plugin_meta_names(ts),
            tsx_source_plugin_meta_names: plugin_meta_names(tsx),
            astro_source_plugin_package_names: plugin_package_names(astro),
            ts_source_plugin_package_names: plugin_package_names(ts),
            tsx_source_plugin_package_names: plugin_package_names(tsx),
            astro_source_error_rules: active_error_rules(astro),
            ts_source_error_rules: active_error_rules(ts),
            tsx_source_error_rules: active_error_rules(tsx),
            astro_source_warn_or_error_rules: active_warn_or_error_rules(astro),
            ts_source_warn_or_error_rules: active_warn_or_error_rules(ts),
            tsx_source_warn_or_error_rules: active_warn_or_error_rules(tsx),
            astro_source_restricted_disable_patterns: restricted_disable_patterns(astro),
            ts_source_restricted_disable_patterns: restricted_disable_patterns(ts),
            tsx_source_restricted_disable_patterns: restricted_disable_patterns(tsx),
            astro_source_unused_disable_fail_closed: unused_disable_fail_closed(astro),
            ts_source_unused_disable_fail_closed: unused_disable_fail_closed(ts),
            tsx_source_unused_disable_fail_closed: unused_disable_fail_closed(tsx),
            astro_source_probe_ignored: probe_ignored(
                &snapshot,
                eslint_config_parser::types::EslintProbeKind::AstroSource,
            ),
            ts_source_probe_ignored: probe_ignored(
                &snapshot,
                eslint_config_parser::types::EslintProbeKind::TsSource,
            ),
            tsx_source_probe_ignored: probe_ignored(
                &snapshot,
                eslint_config_parser::types::EslintProbeKind::TsxSource,
            ),
        },
    }
}

/// `probe_targets`: probe targets.
fn probe_targets(
    crawl: &G3WorkspaceCrawl,
    app_root_rel_path: &str,
) -> Vec<eslint_config_parser::types::EslintProbeTarget> {
    vec![
        probe(
            eslint_config_parser::types::EslintProbeKind::AstroSource,
            first_matching_app_rel_path(crawl, app_root_rel_path, |rel_path| {
                rel_path.starts_with("src/") && has_extension(rel_path, "astro")
            })
            .unwrap_or_else(|| {
                g3ts_astro_check_support::surfaces::scoped_rel_path(
                    app_root_rel_path,
                    "src/__g3ts_probe__.astro",
                )
            }),
        ),
        probe(
            eslint_config_parser::types::EslintProbeKind::TsSource,
            first_matching_app_rel_path(crawl, app_root_rel_path, |rel_path| {
                rel_path.starts_with("src/") && has_extension(rel_path, "ts")
            })
            .unwrap_or_else(|| {
                g3ts_astro_check_support::surfaces::scoped_rel_path(
                    app_root_rel_path,
                    "src/index.ts",
                )
            }),
        ),
        probe(
            eslint_config_parser::types::EslintProbeKind::TsxSource,
            first_matching_app_rel_path(crawl, app_root_rel_path, |rel_path| {
                rel_path.starts_with("src/") && has_extension(rel_path, "tsx")
            })
            .unwrap_or_else(|| {
                g3ts_astro_check_support::surfaces::scoped_rel_path(
                    app_root_rel_path,
                    "src/__g3ts_probe__.tsx",
                )
            }),
        ),
    ]
}

/// Returns `true` when `rel_path` ends with the given extension (case-insensitive).
fn has_extension(rel_path: &str, extension: &str) -> bool {
    std::path::Path::new(rel_path)
        .extension()
        .is_some_and(|value| value.eq_ignore_ascii_case(extension))
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
            entry.kind == g3_workspace_crawl::G3WorkspaceEntryKind::File
                && entry.ignore_state == g3_workspace_crawl::G3WorkspaceIgnoreState::Included
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

/// `probe`: probe.
const fn probe(
    probe: eslint_config_parser::types::EslintProbeKind,
    rel_path: String,
) -> eslint_config_parser::types::EslintProbeTarget {
    eslint_config_parser::types::EslintProbeTarget { probe, rel_path }
}

/// `map_raw_state`: map raw state.
fn map_raw_state(raw: G3TsAstroRawEslintConfigState) -> G3TsAstroSetupEslintSurfaceState {
    match raw {
        G3TsAstroRawEslintConfigState::Missing { rel_path } => {
            G3TsAstroSetupEslintSurfaceState::Missing { rel_path }
        }
        G3TsAstroRawEslintConfigState::Unreadable { rel_path, reason } => {
            G3TsAstroSetupEslintSurfaceState::Unreadable { rel_path, reason }
        }
        G3TsAstroRawEslintConfigState::ParseError { rel_path, reason } => {
            G3TsAstroSetupEslintSurfaceState::ParseError { rel_path, reason }
        }
        G3TsAstroRawEslintConfigState::Parsed { rel_path, .. } => {
            G3TsAstroSetupEslintSurfaceState::ParseError {
                rel_path,
                reason: "parsed raw eslint state reached setup error mapper".to_owned(),
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

/// `probe_ignored`: probe ignored.
fn probe_ignored(
    typed: &eslint_config_parser::types::EslintConfigSnapshot,
    kind: eslint_config_parser::types::EslintProbeKind,
) -> bool {
    probe_by_kind(typed, kind).is_none_or(|probe| probe.ignored)
}

/// `plugins`: plugins.
fn plugins(probe: Option<&eslint_config_parser::types::EslintEffectiveConfigProbe>) -> Vec<String> {
    probe.map_or_else(Vec::new, |probe| probe.plugins.clone())
}

/// `plugin_meta_names`: plugin meta names.
fn plugin_meta_names(
    probe: Option<&eslint_config_parser::types::EslintEffectiveConfigProbe>,
) -> BTreeMap<String, String> {
    probe.map_or_else(BTreeMap::new, |probe| probe.plugin_meta_names.clone())
}

/// Alias for the eslint plugin-to-packages mapping.
type PluginPackageNames = BTreeMap<String, Vec<String>>;

/// `plugin_package_names`: plugin package names.
fn plugin_package_names(
    probe: Option<&eslint_config_parser::types::EslintEffectiveConfigProbe>,
) -> PluginPackageNames {
    probe.map_or_else(BTreeMap::new, |probe| probe.plugin_package_names.clone())
}

/// `active_error_rules`: active error rules.
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

/// `active_warn_or_error_rules`: active warn or error rules.
fn active_warn_or_error_rules(
    probe: Option<&eslint_config_parser::types::EslintEffectiveConfigProbe>,
) -> Vec<String> {
    probe.map_or_else(Vec::new, |probe| {
        probe
            .rules
            .iter()
            .filter_map(|(rule_name, setting)| {
                (setting.severity >= eslint_config_parser::types::EslintRuleSeverity::Warn)
                    .then_some(rule_name.clone())
            })
            .collect()
    })
}

/// `restricted_disable_patterns`: restricted disable patterns.
fn restricted_disable_patterns(
    probe: Option<&eslint_config_parser::types::EslintEffectiveConfigProbe>,
) -> Vec<String> {
    let Some(setting) = probe.and_then(|probe| {
        probe
            .rules
            .get("@eslint-community/eslint-comments/no-restricted-disable")
    }) else {
        return Vec::new();
    };

    if setting.severity < eslint_config_parser::types::EslintRuleSeverity::Warn {
        return Vec::new();
    }

    setting
        .options
        .iter()
        .filter_map(|value| value.as_str())
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_owned)
        .collect()
}

/// `unused_disable_fail_closed`: unused disable fail closed.
fn unused_disable_fail_closed(
    probe: Option<&eslint_config_parser::types::EslintEffectiveConfigProbe>,
) -> bool {
    let Some(probe) = probe else {
        return false;
    };

    probe
        .rules
        .get("@eslint-community/eslint-comments/no-unused-disable")
        .is_some_and(|setting| {
            setting.severity == eslint_config_parser::types::EslintRuleSeverity::Error
        })
        || probe.linter_options_report_unused_disable_directives
            == Some(eslint_config_parser::types::EslintReportUnusedSetting::Error)
}
