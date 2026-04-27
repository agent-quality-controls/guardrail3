use g3_workspace_crawl::G3RsWorkspaceCrawl as G3WorkspaceCrawl;
use g3ts_astro_check_support::surfaces::{
    G3TsAstroRawEslintConfigState, read_eslint_config_surface,
};
use g3ts_astro_setup_types::{
    G3TsAstroSetupEslintSurfaceSnapshot, G3TsAstroSetupEslintSurfaceState,
};
use std::collections::BTreeMap;

#[must_use]
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

fn probe_targets(
    crawl: &G3WorkspaceCrawl,
    app_root_rel_path: &str,
) -> Vec<eslint_config_parser::types::EslintProbeTarget> {
    vec![
        probe(
            eslint_config_parser::types::EslintProbeKind::AstroSource,
            first_matching_app_rel_path(crawl, app_root_rel_path, |rel_path| {
                rel_path.starts_with("src/") && rel_path.ends_with(".astro")
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
                rel_path.starts_with("src/") && rel_path.ends_with(".ts")
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
                rel_path.starts_with("src/") && rel_path.ends_with(".tsx")
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

fn probe(
    probe: eslint_config_parser::types::EslintProbeKind,
    rel_path: String,
) -> eslint_config_parser::types::EslintProbeTarget {
    eslint_config_parser::types::EslintProbeTarget { probe, rel_path }
}

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

fn plugin_meta_names(
    probe: Option<&eslint_config_parser::types::EslintEffectiveConfigProbe>,
) -> BTreeMap<String, String> {
    probe.map_or_else(BTreeMap::new, |probe| probe.plugin_meta_names.clone())
}

fn plugin_package_names(
    probe: Option<&eslint_config_parser::types::EslintEffectiveConfigProbe>,
) -> BTreeMap<String, Vec<String>> {
    probe.map_or_else(BTreeMap::new, |probe| probe.plugin_package_names.clone())
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
