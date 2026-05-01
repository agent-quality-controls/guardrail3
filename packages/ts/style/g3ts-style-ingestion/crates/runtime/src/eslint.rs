use g3_workspace_crawl::G3RsWorkspaceCrawl as G3WorkspaceCrawl;
use g3ts_style_types::{
    G3TsStyleEslintSurfaceSnapshot, G3TsStyleEslintSurfaceState, G3TsStylePolicySurfaceState,
};

const CONFIG_CANDIDATES: [&str; 6] = [
    "eslint.config.mjs",
    "eslint.config.js",
    "eslint.config.cjs",
    "eslint.config.ts",
    "eslint.config.mts",
    "eslint.config.cts",
];

pub(crate) fn ingest_eslint_config(
    crawl: &G3WorkspaceCrawl,
    app_root_rel_path: &str,
    policy: &G3TsStylePolicySurfaceState,
) -> G3TsStyleEslintSurfaceState {
    let Some(rel_path) = nearest_config(crawl, app_root_rel_path) else {
        return G3TsStyleEslintSurfaceState::Missing {
            rel_path: crate::roots::scoped_rel_path(app_root_rel_path, "eslint.config.*"),
        };
    };
    let Some(entry) = crawl
        .entries
        .iter()
        .find(|entry| entry.path.rel_path == rel_path)
    else {
        return G3TsStyleEslintSurfaceState::Missing { rel_path };
    };
    if !entry.readable {
        return G3TsStyleEslintSurfaceState::Unreadable {
            rel_path: entry.path.rel_path.clone(),
            reason: "workspace crawl marked the ESLint config unreadable".to_owned(),
        };
    }

    let probes = probe_targets(app_root_rel_path, policy);
    let snapshot =
        match eslint_config_parser::parse(&crawl.root_abs_path, &entry.path.rel_path, &probes) {
            Ok(snapshot) => snapshot,
            Err(error) => {
                return G3TsStyleEslintSurfaceState::ParseError {
                    rel_path: entry.path.rel_path.clone(),
                    reason: error.to_string(),
                };
            }
        };
    let source_probe_present = !snapshot.probes.is_empty();
    let source_probe_ignored = snapshot.probes.iter().any(|probe| probe.ignored);
    let source_plugins = snapshot
        .probes
        .iter()
        .flat_map(|probe| probe.plugins.iter().cloned())
        .collect::<std::collections::BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    let source_plugin_package_names = snapshot
        .probes
        .iter()
        .flat_map(|probe| probe.plugin_package_names.clone())
        .collect();
    let style_policy_plugin_effective = all_probes_use_owned_style_policy_plugin(&snapshot.probes);
    let style_policy_rule_effective = snapshot.probes.iter().all(|probe| {
        let Some(rule) = probe.rules.get("style-policy/no-denied-class-tokens") else {
            return false;
        };
        rule_has_effective_style_policy(rule)
    });

    G3TsStyleEslintSurfaceState::Parsed {
        snapshot: G3TsStyleEslintSurfaceSnapshot {
            rel_path: entry.path.rel_path.clone(),
            source_probe_present,
            source_probe_ignored,
            source_plugins,
            source_plugin_package_names,
            style_policy_plugin_effective,
            style_policy_rule_effective,
        },
    }
}

fn rule_has_effective_style_policy(
    rule: &eslint_config_parser::types::EslintRuleSetting,
) -> bool {
    rule.severity == eslint_config_parser::types::EslintRuleSeverity::Error
        && rule
            .options
            .first()
            .is_some_and(option_has_non_empty_style_policy)
}

fn all_probes_use_owned_style_policy_plugin(
    probes: &[eslint_config_parser::types::EslintEffectiveConfigProbe],
) -> bool {
    probes.iter().all(|probe| {
        probe.plugin_package_names
            .get("style-policy")
            .is_some_and(|packages| {
                packages
                    .iter()
                    .any(|package| package == "g3ts-eslint-plugin-style-policy")
            })
    })
}

fn option_has_non_empty_style_policy(option: &serde_json::Value) -> bool {
    ["denyList", "denyPrefixes", "denyPatterns"]
        .iter()
        .any(|key| option_has_non_empty_string_array(option, key))
}

fn option_has_non_empty_string_array(option: &serde_json::Value, key: &str) -> bool {
    option
        .get(key)
        .and_then(serde_json::Value::as_array)
        .is_some_and(|values| {
            values
                .iter()
                .any(|item| item.as_str().is_some_and(|value| !value.trim().is_empty()))
        })
}

fn probe_targets(
    app_root_rel_path: &str,
    policy: &G3TsStylePolicySurfaceState,
) -> Vec<eslint_config_parser::types::EslintProbeTarget> {
    let G3TsStylePolicySurfaceState::Parsed { snapshot } = policy else {
        return fallback_probe_targets(app_root_rel_path);
    };
    let targets = snapshot
        .source_globs
        .iter()
        .flat_map(|glob| probe_targets_from_glob(app_root_rel_path, glob))
        .collect::<Vec<_>>();
    if targets.is_empty() {
        fallback_probe_targets(app_root_rel_path)
    } else {
        dedupe_targets(targets)
    }
}

fn fallback_probe_targets(
    app_root_rel_path: &str,
) -> Vec<eslint_config_parser::types::EslintProbeTarget> {
    vec![target(
        app_root_rel_path,
        "src/__g3ts_style_probe__.tsx",
        eslint_config_parser::types::EslintProbeKind::TsxSource,
    )]
}

fn probe_targets_from_glob(
    app_root_rel_path: &str,
    glob: &str,
) -> Vec<eslint_config_parser::types::EslintProbeTarget> {
    ["astro", "ts", "tsx", "jsx", "js"]
        .into_iter()
        .filter(|extension| glob_mentions_extension(glob, extension))
        .filter_map(|extension| {
            Some(target(
                app_root_rel_path,
                &format!(
                    "{}/__g3ts_style_probe__.{extension}",
                    glob_prefix_directory(glob)?
                ),
                probe_kind_for_extension(extension)?,
            ))
        })
        .collect()
}

fn glob_mentions_extension(glob: &str, extension: &str) -> bool {
    glob.contains(&format!(".{extension}"))
        || glob.contains(&format!("{{{extension}"))
        || glob.contains(&format!(",{extension}"))
}

fn glob_prefix_directory(glob: &str) -> Option<String> {
    let prefix = glob
        .split(|character| matches!(character, '*' | '?' | '[' | '{'))
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

fn probe_kind_for_extension(
    extension: &str,
) -> Option<eslint_config_parser::types::EslintProbeKind> {
    match extension {
        "astro" => Some(eslint_config_parser::types::EslintProbeKind::AstroSource),
        "ts" => Some(eslint_config_parser::types::EslintProbeKind::TsSource),
        "tsx" => Some(eslint_config_parser::types::EslintProbeKind::TsxSource),
        "js" | "jsx" => Some(eslint_config_parser::types::EslintProbeKind::JsSource),
        _ => None,
    }
}

fn target(
    app_root_rel_path: &str,
    local_rel_path: &str,
    probe: eslint_config_parser::types::EslintProbeKind,
) -> eslint_config_parser::types::EslintProbeTarget {
    eslint_config_parser::types::EslintProbeTarget {
        probe,
        rel_path: crate::roots::scoped_rel_path(app_root_rel_path, local_rel_path),
    }
}

fn dedupe_targets(
    targets: Vec<eslint_config_parser::types::EslintProbeTarget>,
) -> Vec<eslint_config_parser::types::EslintProbeTarget> {
    let mut seen = std::collections::BTreeSet::new();
    targets
        .into_iter()
        .filter(|target| seen.insert((target.probe, target.rel_path.clone())))
        .collect()
}

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

#[cfg(test)]
#[path = "eslint_tests/mod.rs"]
// reason: keep private ESLint style ingestion tests in the owned sidecar directory.
mod eslint_tests;

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
