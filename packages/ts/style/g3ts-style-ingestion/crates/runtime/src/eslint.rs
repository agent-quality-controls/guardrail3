use g3_workspace_crawl::G3WorkspaceCrawl;
use g3ts_style_types::{
    G3TsStyleEslintProbeDisablePolicySnapshot, G3TsStyleEslintSurfaceSnapshot,
    G3TsStyleEslintSurfaceState, G3TsStylePolicySurfaceState,
};

/// Recognized `ESLint` config file names, ordered by preference.
const CONFIG_CANDIDATES: [&str; 6] = [
    "eslint.config.mjs",
    "eslint.config.js",
    "eslint.config.cjs",
    "eslint.config.ts",
    "eslint.config.mts",
    "eslint.config.cts",
];

/// Read and parse the nearest `ESLint` config under `app_root_rel_path`
/// from `crawl`, using `policy` to derive probe targets.
#[expect(
    clippy::too_many_lines,
    reason = "Linear single-function ingestion expressing distinct ESLint config-surface \
              variants with their result emissions; splitting would require shared mutable \
              state plumbing that would obscure the linear control flow"
)]
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
    let source_warn_or_error_rules = snapshot
        .probes
        .iter()
        .flat_map(active_warn_or_error_rules)
        .collect::<std::collections::BTreeSet<_>>()
        .into_iter()
        .collect();
    let source_restricted_disable_patterns = snapshot
        .probes
        .iter()
        .flat_map(restricted_disable_patterns)
        .collect::<std::collections::BTreeSet<_>>()
        .into_iter()
        .collect();
    let source_probe_disable_policies = snapshot.probes.iter().map(probe_disable_policy).collect();

    G3TsStyleEslintSurfaceState::Parsed {
        snapshot: G3TsStyleEslintSurfaceSnapshot {
            rel_path: entry.path.rel_path.clone(),
            source_probe_present,
            source_probe_ignored,
            source_plugins,
            source_plugin_package_names,
            style_policy_plugin_effective,
            style_policy_rule_effective,
            source_warn_or_error_rules,
            source_restricted_disable_patterns,
            source_probe_disable_policies,
        },
    }
}

/// Build a disable-policy snapshot for a single probe.
fn probe_disable_policy(
    probe: &eslint_config_parser::types::EslintEffectiveConfigProbe,
) -> G3TsStyleEslintProbeDisablePolicySnapshot {
    G3TsStyleEslintProbeDisablePolicySnapshot {
        rel_path: probe.rel_path.clone(),
        ignored: probe.ignored,
        warn_or_error_rules: active_warn_or_error_rules(probe),
        restricted_disable_patterns: restricted_disable_patterns(probe),
    }
}

/// List rule names that are configured at warn-or-error severity in the
/// probe's effective config.
fn active_warn_or_error_rules(
    probe: &eslint_config_parser::types::EslintEffectiveConfigProbe,
) -> Vec<String> {
    if probe.ignored {
        return Vec::new();
    }
    probe
        .rules
        .iter()
        .filter_map(|(rule_name, setting)| {
            (setting.severity >= eslint_config_parser::types::EslintRuleSeverity::Warn)
                .then_some(rule_name.clone())
        })
        .collect()
}

/// List patterns configured for `@eslint-community/eslint-comments/no-restricted-disable`.
fn restricted_disable_patterns(
    probe: &eslint_config_parser::types::EslintEffectiveConfigProbe,
) -> Vec<String> {
    if probe.ignored {
        return Vec::new();
    }
    let Some(setting) = probe
        .rules
        .get("@eslint-community/eslint-comments/no-restricted-disable")
    else {
        return Vec::new();
    };
    if setting.severity < eslint_config_parser::types::EslintRuleSeverity::Warn {
        return Vec::new();
    }
    setting
        .options
        .iter()
        .filter_map(serde_json::Value::as_str)
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_owned)
        .collect()
}

/// Whether `rule` configures the `style-policy/*` plugin with non-empty options.
fn rule_has_effective_style_policy(rule: &eslint_config_parser::types::EslintRuleSetting) -> bool {
    rule.severity == eslint_config_parser::types::EslintRuleSeverity::Error
        && rule
            .options
            .first()
            .is_some_and(option_has_non_empty_style_policy)
}

/// Whether every probe in the config snapshot lists the owned
/// `style-policy` plugin.
fn all_probes_use_owned_style_policy_plugin(
    probes: &[eslint_config_parser::types::EslintEffectiveConfigProbe],
) -> bool {
    probes.iter().all(|probe| {
        probe
            .plugin_package_names
            .get("style-policy")
            .is_some_and(|packages| {
                packages
                    .iter()
                    .any(|package| package == "g3ts-eslint-plugin-style-policy")
            })
    })
}

/// Whether `option` is an object with a non-empty `style-policy` string list.
fn option_has_non_empty_style_policy(option: &serde_json::Value) -> bool {
    ["denyList", "denyPrefixes", "denyPatterns"]
        .iter()
        .any(|key| option_has_non_empty_string_array(option, key))
}

/// Whether `option[key]` is a non-empty array of non-empty strings.
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

/// Derive `ESLint` probe targets from `policy.source_globs` (or a fallback
/// when policy is unavailable).
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

/// Default probe target list used when the policy declares no source globs.
fn fallback_probe_targets(
    app_root_rel_path: &str,
) -> Vec<eslint_config_parser::types::EslintProbeTarget> {
    vec![target(
        app_root_rel_path,
        "src/__g3ts_style_probe__.tsx",
        eslint_config_parser::types::EslintProbeKind::TsxSource,
    )]
}

/// Derive a list of probe targets from a single `glob` pattern.
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

/// Whether `glob` textually mentions `extension`.
fn glob_mentions_extension(glob: &str, extension: &str) -> bool {
    glob.contains(&format!(".{extension}"))
        || glob.contains(&format!("{{{extension}"))
        || glob.contains(&format!(",{extension}"))
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

/// Map a source-file `extension` to its `ESLint` probe-kind variant.
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

/// Build an `EslintProbeTarget` for `local_rel_path` scoped to `app_root_rel_path`.
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

/// Drop duplicate `(probe, rel_path)` pairs from `targets`, preserving order.
fn dedupe_targets(
    targets: Vec<eslint_config_parser::types::EslintProbeTarget>,
) -> Vec<eslint_config_parser::types::EslintProbeTarget> {
    let mut seen = std::collections::BTreeSet::new();
    targets
        .into_iter()
        .filter(|target| seen.insert((target.probe, target.rel_path.clone())))
        .collect()
}

/// Find the nearest `ESLint` config rel-path by walking up from
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
