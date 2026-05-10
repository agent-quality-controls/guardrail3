use super::settings;
use super::targets;

use g3_workspace_crawl::G3RsWorkspaceCrawl as G3WorkspaceCrawl;
use g3ts_astro_check_support::surfaces::{
    G3TsAstroRawEslintConfigState, read_eslint_config_surface,
};
use g3ts_astro_media_types::{
    G3TsAstroMediaEslintSurfaceSnapshot, G3TsAstroMediaEslintSurfaceState,
    G3TsAstroMediaPolicySurfaceState,
};

/// `ingest_media_eslint_surface`: ingest media eslint surface.
pub(crate) fn ingest_media_eslint_surface(
    crawl: &G3WorkspaceCrawl,
    app_root_rel_path: &str,
    astro_policy: &G3TsAstroMediaPolicySurfaceState,
) -> G3TsAstroMediaEslintSurfaceState {
    let targets = targets::probe_targets(app_root_rel_path, astro_policy);
    let raw = read_eslint_config_surface(crawl, app_root_rel_path, &targets);
    let G3TsAstroRawEslintConfigState::Parsed { rel_path, snapshot } = raw else {
        return map_raw_state(raw);
    };

    let public_targets = targets::public_probe_targets(app_root_rel_path, astro_policy);
    let public_probes = active_probes(&snapshot, &public_targets);

    G3TsAstroMediaEslintSurfaceState::Parsed {
        snapshot: G3TsAstroMediaEslintSurfaceSnapshot {
            rel_path,
            public_probe_present: !public_targets.is_empty()
                && public_probes.len() == public_targets.len(),
            public_probe_ignored: probes_missing_or_ignored(&snapshot, &public_targets),
            public_plugins: settings::common_plugins(&public_probes),
            public_plugin_package_names: settings::common_plugin_package_names(&public_probes),
            public_error_rules: settings::common_error_rules(&public_probes),
            public_restricted_disable_patterns: settings::common_restricted_disable_patterns(
                &public_probes,
            ),
            public_media_policy_rules: settings::common_effective_media_policy_rules(
                &public_probes,
                astro_policy,
            ),
        },
    }
}

/// `map_raw_state`: map raw state.
fn map_raw_state(raw: G3TsAstroRawEslintConfigState) -> G3TsAstroMediaEslintSurfaceState {
    match raw {
        G3TsAstroRawEslintConfigState::Missing { rel_path } => {
            G3TsAstroMediaEslintSurfaceState::Missing { rel_path }
        }
        G3TsAstroRawEslintConfigState::Unreadable { rel_path, reason } => {
            G3TsAstroMediaEslintSurfaceState::Unreadable { rel_path, reason }
        }
        G3TsAstroRawEslintConfigState::ParseError { rel_path, reason } => {
            G3TsAstroMediaEslintSurfaceState::ParseError { rel_path, reason }
        }
        G3TsAstroRawEslintConfigState::Parsed { rel_path, .. } => {
            G3TsAstroMediaEslintSurfaceState::ParseError {
                rel_path,
                reason: "parsed raw eslint state reached media error mapper".to_owned(),
            }
        }
    }
}

/// `active_probes`: active probes.
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

/// `probes_missing_or_ignored`: probes missing or ignored.
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
