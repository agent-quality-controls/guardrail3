use super::settings;
use super::targets;

use g3_workspace_crawl::G3WorkspaceCrawl;
use g3ts_astro_check_support::surfaces::{
    G3TsAstroRawEslintConfigState, read_eslint_config_surface,
};
use g3ts_astro_i18n_types::{
    G3TsAstroI18nEslintSurfaceSnapshot, G3TsAstroI18nEslintSurfaceState,
    G3TsAstroI18nPolicySurfaceState,
};

/// Reads the `ESLint` config surface for an Astro app and projects it into the i18n surface state.
pub(crate) fn ingest_i18n_eslint_surface(
    crawl: &G3WorkspaceCrawl,
    app_root_rel_path: &str,
    astro_policy: &G3TsAstroI18nPolicySurfaceState,
) -> G3TsAstroI18nEslintSurfaceState {
    let targets = targets::probe_targets(app_root_rel_path, astro_policy);
    let raw = read_eslint_config_surface(crawl, app_root_rel_path, &targets);
    let G3TsAstroRawEslintConfigState::Parsed { rel_path, snapshot } = raw else {
        return map_raw_state(raw);
    };

    let public_targets = targets::public_probe_targets(app_root_rel_path, astro_policy);
    let helper_targets = targets::helper_probe_targets(app_root_rel_path, astro_policy);
    let public_probes = active_probes(&snapshot, &public_targets);
    let helper_probes = active_probes(&snapshot, &helper_targets);

    G3TsAstroI18nEslintSurfaceState::Parsed {
        snapshot: G3TsAstroI18nEslintSurfaceSnapshot {
            rel_path,
            public_probe_present: !public_targets.is_empty()
                && public_probes.len() == public_targets.len(),
            public_probe_ignored: probes_missing_or_ignored(&snapshot, &public_targets),
            helper_probe_present: !helper_targets.is_empty()
                && helper_probes.len() == helper_targets.len(),
            helper_probe_ignored: probes_missing_or_ignored(&snapshot, &helper_targets),
            public_plugins: settings::common_plugins(&public_probes),
            public_plugin_package_names: settings::common_plugin_package_names(&public_probes),
            public_error_rules: settings::common_error_rules(&public_probes),
            public_restricted_disable_patterns: settings::common_restricted_disable_patterns(
                &public_probes,
            ),
            public_i18n_policy_rules: settings::common_effective_i18n_policy_rules(
                &public_probes,
                astro_policy,
            ),
            public_no_restricted_syntax_selectors: settings::common_no_restricted_syntax_selectors(
                &public_probes,
            ),
            helper_no_restricted_syntax_selectors: settings::union_no_restricted_syntax_selectors(
                &helper_probes,
            ),
        },
    }
}

/// Maps the raw `ESLint` config surface into the i18n-specific surface state.
fn map_raw_state(raw: G3TsAstroRawEslintConfigState) -> G3TsAstroI18nEslintSurfaceState {
    match raw {
        G3TsAstroRawEslintConfigState::Missing { rel_path } => {
            G3TsAstroI18nEslintSurfaceState::Missing { rel_path }
        }
        G3TsAstroRawEslintConfigState::Unreadable { rel_path, reason } => {
            G3TsAstroI18nEslintSurfaceState::Unreadable { rel_path, reason }
        }
        G3TsAstroRawEslintConfigState::ParseError { rel_path, reason } => {
            G3TsAstroI18nEslintSurfaceState::ParseError { rel_path, reason }
        }
        G3TsAstroRawEslintConfigState::Parsed { rel_path, .. } => {
            G3TsAstroI18nEslintSurfaceState::ParseError {
                rel_path,
                reason: "parsed raw eslint state reached i18n error mapper".to_owned(),
            }
        }
    }
}

/// Returns probes from the snapshot that match the requested targets and are not ignored.
fn active_probes<'a>(
    typed: &'a eslint_config_parser::types::EslintConfigSnapshot,
    targets: &[eslint_config_parser::types::EslintProbeTarget],
) -> Vec<&'a eslint_config_parser::types::EslintEffectiveConfigProbe> {
    targets
        .iter()
        .filter_map(|target| {
            typed
                .probes
                .iter()
                .find(|probe| probe.probe == target.probe && probe.rel_path == target.rel_path)
                .filter(|probe| !probe.ignored)
        })
        .collect()
}

/// Returns true when any requested target is missing or ignored in the snapshot.
fn probes_missing_or_ignored(
    typed: &eslint_config_parser::types::EslintConfigSnapshot,
    targets: &[eslint_config_parser::types::EslintProbeTarget],
) -> bool {
    targets.iter().any(|target| {
        typed
            .probes
            .iter()
            .find(|probe| probe.probe == target.probe && probe.rel_path == target.rel_path)
            .is_none_or(|probe| probe.ignored)
    })
}
