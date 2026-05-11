use g3_workspace_crawl::G3WorkspaceCrawl;
use g3ts_style_types::{
    G3TsStylePolicySurfaceState, G3TsStylelintConfigSnapshot, G3TsStylelintConfigSurfaceState,
};

/// Recognized Stylelint config file names, ordered by preference.
const CONFIG_CANDIDATES: [&str; 6] = [
    ".stylelintrc.mjs",
    ".stylelintrc.js",
    ".stylelintrc.cjs",
    "stylelint.config.mjs",
    "stylelint.config.js",
    "stylelint.config.cjs",
];

/// Read and parse the nearest Stylelint config under `app_root_rel_path`
/// from `crawl`, using `policy` to derive probe targets.
pub(crate) fn ingest_stylelint_config(
    crawl: &G3WorkspaceCrawl,
    app_root_rel_path: &str,
    policy: &G3TsStylePolicySurfaceState,
) -> G3TsStylelintConfigSurfaceState {
    let Some(rel_path) = nearest_config(crawl, app_root_rel_path) else {
        return G3TsStylelintConfigSurfaceState::Missing {
            rel_path: crate::roots::scoped_rel_path(app_root_rel_path, "stylelint.config.*"),
        };
    };
    let Some(entry) = crawl
        .entries
        .iter()
        .find(|entry| entry.path.rel_path == rel_path)
    else {
        return G3TsStylelintConfigSurfaceState::Missing { rel_path };
    };
    if !entry.readable {
        return G3TsStylelintConfigSurfaceState::Unreadable {
            rel_path: entry.path.rel_path.clone(),
            reason: "workspace crawl marked the Stylelint config unreadable".to_owned(),
        };
    }

    let probes = probe_targets(app_root_rel_path, policy);
    let snapshot =
        match stylelint_config_parser::parse(&crawl.root_abs_path, &entry.path.rel_path, &probes) {
            Ok(snapshot) => snapshot,
            Err(error) => {
                return G3TsStylelintConfigSurfaceState::ParseError {
                    rel_path: entry.path.rel_path.clone(),
                    reason: error.to_string(),
                };
            }
        };
    let probe_present = !snapshot.probes.is_empty();
    let probe_ignored = snapshot.probes.iter().any(|probe| probe.ignored);
    G3TsStylelintConfigSurfaceState::Parsed {
        snapshot: G3TsStylelintConfigSnapshot {
            rel_path: entry.path.rel_path.clone(),
            raw_extends: snapshot.raw_extends,
            raw_plugins: snapshot.raw_plugins,
            resolved_extends: snapshot
                .probes
                .iter()
                .flat_map(|probe| probe.extends.iter().cloned())
                .collect::<std::collections::BTreeSet<_>>()
                .into_iter()
                .collect(),
            resolved_plugins: snapshot
                .probes
                .iter()
                .flat_map(|probe| probe.plugins.iter().cloned())
                .collect::<std::collections::BTreeSet<_>>()
                .into_iter()
                .collect(),
            resolved_rule_names: snapshot
                .probes
                .iter()
                .flat_map(|probe| probe.rules.keys().cloned())
                .collect::<std::collections::BTreeSet<_>>()
                .into_iter()
                .collect(),
            probe_present,
            probe_ignored,
        },
    }
}

/// Derive Stylelint probe targets from `policy.stylelint_css_globs` (or a
/// fallback when policy is unavailable).
fn probe_targets(
    app_root_rel_path: &str,
    policy: &G3TsStylePolicySurfaceState,
) -> Vec<stylelint_config_parser::types::StylelintProbeTarget> {
    let G3TsStylePolicySurfaceState::Parsed { snapshot } = policy else {
        return fallback_probe_targets(app_root_rel_path);
    };
    let targets = snapshot
        .stylelint_css_globs
        .iter()
        .filter_map(|glob| {
            Some(stylelint_config_parser::types::StylelintProbeTarget {
                rel_path: crate::roots::scoped_rel_path(
                    app_root_rel_path,
                    &format!("{}/__g3ts_style_probe__.css", glob_prefix_directory(glob)?),
                ),
            })
        })
        .collect::<Vec<_>>();
    if targets.is_empty() {
        fallback_probe_targets(app_root_rel_path)
    } else {
        targets
    }
}

/// Default probe target list used when the policy declares no CSS globs.
fn fallback_probe_targets(
    app_root_rel_path: &str,
) -> Vec<stylelint_config_parser::types::StylelintProbeTarget> {
    vec![stylelint_config_parser::types::StylelintProbeTarget {
        rel_path: crate::roots::scoped_rel_path(app_root_rel_path, "src/__g3ts_style_probe__.css"),
    }]
}

/// Extract the static directory prefix of `glob` (the portion preceding
/// the first glob metacharacter), returning None when the prefix is empty.
fn glob_prefix_directory(glob: &str) -> Option<String> {
    let prefix = glob
        .split(['*', '?', '[', '{'])
        .next()?
        .trim_end_matches('/');
    if prefix.is_empty() {
        return None;
    }
    if std::path::Path::new(prefix).extension().is_some() {
        return std::path::Path::new(prefix)
            .parent()
            .and_then(std::path::Path::to_str)
            .filter(|parent| !parent.is_empty())
            .map(str::to_owned);
    }
    Some(prefix.to_owned())
}

/// Find the nearest Stylelint config rel-path by walking up from
/// `app_root_rel_path` and matching each ancestor against `CONFIG_CANDIDATES`.
fn nearest_config(crawl: &G3WorkspaceCrawl, app_root_rel_path: &str) -> Option<String> {
    ancestors(app_root_rel_path).into_iter().find_map(|scope| {
        CONFIG_CANDIDATES
            .iter()
            .map(|candidate| crate::roots::scoped_rel_path(&scope, candidate))
            .find(|rel_path| {
                crawl
                    .entries
                    .iter()
                    .any(|entry| entry.path.rel_path == *rel_path)
            })
    })
}

/// Enumerate `app_root_rel_path` and each of its ancestor directories, in
/// nearest-first order, using `"."` for the workspace root.
fn ancestors(app_root_rel_path: &str) -> Vec<String> {
    let mut ancestors = Vec::new();
    let mut current = std::path::Path::new(app_root_rel_path);
    loop {
        ancestors.push(
            current
                .to_str()
                .filter(|path| !path.is_empty())
                .map_or_else(|| ".".to_owned(), str::to_owned),
        );
        let Some(parent) = current.parent() else {
            break;
        };
        if parent == current {
            break;
        }
        current = parent;
    }
    ancestors
}
