use g3_workspace_crawl::G3RsWorkspaceCrawl as G3WorkspaceCrawl;
use g3ts_astro_check_support::surfaces::{
    G3TsAstroRawEslintConfigState, read_eslint_config_surface,
};
use g3ts_astro_i18n_types::{
    G3TsAstroI18nEslintSurfaceSnapshot, G3TsAstroI18nEslintSurfaceState,
    G3TsAstroI18nPolicySurfaceState,
};

const POLICY_RULES: [&str; 3] = [
    "astro-i18n-policy/no-unlocalized-internal-hrefs",
    "astro-i18n-policy/no-inline-image-alt",
    "astro-i18n-policy/require-content-image-key",
];
const NO_RESTRICTED_SYNTAX: &str = "no-restricted-syntax";

pub(crate) fn ingest_i18n_eslint_surface(
    crawl: &G3WorkspaceCrawl,
    app_root_rel_path: &str,
    astro_policy: &G3TsAstroI18nPolicySurfaceState,
) -> G3TsAstroI18nEslintSurfaceState {
    let raw = read_eslint_config_surface(
        crawl,
        app_root_rel_path,
        &probe_targets(app_root_rel_path, astro_policy),
    );
    let G3TsAstroRawEslintConfigState::Parsed { rel_path, snapshot } = raw else {
        return map_raw_state(raw);
    };

    let public_probe = active_probe(
        &snapshot,
        eslint_config_parser::types::EslintProbeKind::TsxSource,
    );
    let helper_probe = active_probe(
        &snapshot,
        eslint_config_parser::types::EslintProbeKind::TsSource,
    );

    G3TsAstroI18nEslintSurfaceState::Parsed {
        snapshot: G3TsAstroI18nEslintSurfaceSnapshot {
            rel_path,
            public_probe_present: public_probe.is_some(),
            public_probe_ignored: probe_ignored(
                &snapshot,
                eslint_config_parser::types::EslintProbeKind::TsxSource,
            ),
            helper_probe_present: helper_probe.is_some(),
            helper_probe_ignored: probe_ignored(
                &snapshot,
                eslint_config_parser::types::EslintProbeKind::TsSource,
            ),
            public_plugins: plugins(public_probe),
            public_plugin_package_names: plugin_package_names(public_probe),
            public_error_rules: active_error_rules(public_probe),
            public_restricted_disable_patterns: restricted_disable_patterns(public_probe),
            public_i18n_policy_rules: effective_i18n_policy_rules(public_probe, astro_policy),
            public_no_restricted_syntax_selectors: no_restricted_syntax_selectors(public_probe),
            helper_no_restricted_syntax_selectors: no_restricted_syntax_selectors(helper_probe),
        },
    }
}

fn probe_targets(
    app_root_rel_path: &str,
    astro_policy: &G3TsAstroI18nPolicySurfaceState,
) -> Vec<eslint_config_parser::types::EslintProbeTarget> {
    vec![
        eslint_config_parser::types::EslintProbeTarget {
            probe: eslint_config_parser::types::EslintProbeKind::TsxSource,
            rel_path: g3ts_astro_check_support::surfaces::scoped_rel_path(
                app_root_rel_path,
                &policy_public_probe(astro_policy)
                    .unwrap_or_else(|| "src/pages/__g3ts_i18n_probe__.tsx".to_owned()),
            ),
        },
        eslint_config_parser::types::EslintProbeTarget {
            probe: eslint_config_parser::types::EslintProbeKind::TsSource,
            rel_path: g3ts_astro_check_support::surfaces::scoped_rel_path(
                app_root_rel_path,
                &policy_helper_probe(astro_policy)
                    .unwrap_or_else(|| "src/i18n/__g3ts_i18n_helper_probe__.ts".to_owned()),
            ),
        },
    ]
}

fn policy_public_probe(policy: &G3TsAstroI18nPolicySurfaceState) -> Option<String> {
    let G3TsAstroI18nPolicySurfaceState::Parsed { snapshot } = policy else {
        return None;
    };

    snapshot
        .public_source_globs
        .iter()
        .find_map(|glob| probe_from_glob(glob, "tsx"))
}

fn policy_helper_probe(policy: &G3TsAstroI18nPolicySurfaceState) -> Option<String> {
    let G3TsAstroI18nPolicySurfaceState::Parsed { snapshot } = policy else {
        return None;
    };

    snapshot
        .helper_source_globs
        .iter()
        .find_map(|glob| probe_from_glob(glob, "ts"))
}

fn probe_from_glob(glob: &str, extension: &str) -> Option<String> {
    let prefix = glob
        .split('*')
        .next()
        .map(str::trim)
        .filter(|prefix| !prefix.is_empty())?
        .trim_end_matches('/')
        .to_owned();

    Some(format!("{prefix}/__g3ts_i18n_probe__.{extension}"))
}

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

fn active_probe(
    typed: &eslint_config_parser::types::EslintConfigSnapshot,
    kind: eslint_config_parser::types::EslintProbeKind,
) -> Option<&eslint_config_parser::types::EslintEffectiveConfigProbe> {
    typed
        .probes
        .iter()
        .find(|probe| probe.probe == kind)
        .filter(|probe| !probe.ignored)
}

fn probe_ignored(
    typed: &eslint_config_parser::types::EslintConfigSnapshot,
    kind: eslint_config_parser::types::EslintProbeKind,
) -> bool {
    typed
        .probes
        .iter()
        .find(|probe| probe.probe == kind)
        .is_none_or(|probe| probe.ignored)
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
    let Some(probe) = probe else {
        return Vec::new();
    };

    probe
        .rules
        .iter()
        .filter(|(_, setting)| {
            setting.severity == eslint_config_parser::types::EslintRuleSeverity::Error
        })
        .map(|(rule, _)| rule.clone())
        .collect()
}

fn effective_i18n_policy_rules(
    probe: Option<&eslint_config_parser::types::EslintEffectiveConfigProbe>,
    astro_policy: &G3TsAstroI18nPolicySurfaceState,
) -> Vec<String> {
    let Some(probe) = probe else {
        return Vec::new();
    };
    let G3TsAstroI18nPolicySurfaceState::Parsed { snapshot } = astro_policy else {
        return Vec::new();
    };

    POLICY_RULES
        .into_iter()
        .filter(|rule| {
            probe.rules.get(*rule).is_some_and(|setting| {
                setting.severity == eslint_config_parser::types::EslintRuleSeverity::Error
                    && setting
                        .options
                        .first()
                        .is_some_and(|options| rule_options_match_policy(rule, options, snapshot))
            })
        })
        .map(str::to_owned)
        .collect()
}

fn rule_options_match_policy(
    rule: &str,
    options: &serde_json::Value,
    policy: &g3ts_astro_i18n_types::G3TsAstroI18nPolicySnapshot,
) -> bool {
    match rule {
        "astro-i18n-policy/no-unlocalized-internal-hrefs" => {
            string_array_option_matches(options, "locales", &policy.locales)
                && optional_string_option_matches(options, "defaultLocale", &policy.default_locale)
                && bool_option_matches(
                    options,
                    "requireLocalePrefixForContentRoutes",
                    policy.require_locale_prefix_for_content_routes,
                )
                && string_array_option_matches(
                    options,
                    "allowedUnprefixedRoutes",
                    &policy.allowed_unprefixed_routes,
                )
                && string_array_option_matches(
                    options,
                    "contentRoutePrefixes",
                    &policy.content_route_prefixes,
                )
                && string_array_option_matches(
                    options,
                    "approvedInternalLinkHelpers",
                    &policy.approved_internal_link_helpers,
                )
                && string_array_option_matches(
                    options,
                    "approvedLocalizedLinkComponents",
                    &policy.approved_localized_link_components,
                )
        }
        "astro-i18n-policy/no-inline-image-alt" => {
            string_array_option_matches(
                options,
                "contentImageComponents",
                &policy.content_image_components,
            ) && string_array_option_matches(
                options,
                "bannedImageAltProps",
                &policy.banned_image_alt_props,
            )
        }
        "astro-i18n-policy/require-content-image-key" => {
            string_array_option_matches(
                options,
                "contentImageComponents",
                &policy.content_image_components,
            ) && string_array_option_matches(
                options,
                "contentImageKeyProps",
                &policy.content_image_key_props,
            ) && string_array_option_matches(
                options,
                "bannedImageSourceProps",
                &policy.banned_image_source_props,
            )
        }
        _ => false,
    }
}

fn string_array_option_matches(
    options: &serde_json::Value,
    key: &str,
    expected: &[String],
) -> bool {
    let Some(actual) = options.get(key).and_then(serde_json::Value::as_array) else {
        return false;
    };
    let actual = actual
        .iter()
        .filter_map(serde_json::Value::as_str)
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .collect::<std::collections::BTreeSet<_>>();
    let expected = expected
        .iter()
        .map(String::as_str)
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .collect::<std::collections::BTreeSet<_>>();

    actual == expected
}

fn optional_string_option_matches(
    options: &serde_json::Value,
    key: &str,
    expected: &Option<String>,
) -> bool {
    match expected {
        Some(expected) => {
            options.get(key).and_then(serde_json::Value::as_str) == Some(expected.as_str())
        }
        None => !options.get(key).is_some_and(|value| !value.is_null()),
    }
}

fn bool_option_matches(options: &serde_json::Value, key: &str, expected: bool) -> bool {
    options.get(key).and_then(serde_json::Value::as_bool) == Some(expected)
}

fn no_restricted_syntax_selectors(
    probe: Option<&eslint_config_parser::types::EslintEffectiveConfigProbe>,
) -> Vec<String> {
    let Some(probe) = probe else {
        return Vec::new();
    };
    let Some(setting) = probe.rules.get(NO_RESTRICTED_SYNTAX) else {
        return Vec::new();
    };

    setting
        .options
        .iter()
        .filter_map(|option| option.get("selector"))
        .filter_map(serde_json::Value::as_str)
        .map(str::to_owned)
        .collect()
}

fn restricted_disable_patterns(
    probe: Option<&eslint_config_parser::types::EslintEffectiveConfigProbe>,
) -> Vec<String> {
    let Some(probe) = probe else {
        return Vec::new();
    };
    let Some(setting) = probe
        .rules
        .get("@eslint-community/eslint-comments/no-restricted-disable")
    else {
        return Vec::new();
    };

    setting
        .options
        .iter()
        .flat_map(disable_patterns_from_option)
        .collect()
}

fn disable_patterns_from_option(option: &serde_json::Value) -> Vec<String> {
    if let Some(items) = option.as_array() {
        return items.iter().filter_map(disable_pattern).collect();
    }

    disable_pattern(option).into_iter().collect()
}

fn disable_pattern(value: &serde_json::Value) -> Option<String> {
    if let Some(pattern) = value.as_str() {
        return Some(pattern.to_owned());
    }

    value
        .get("rule")
        .or_else(|| value.get("name"))
        .or_else(|| value.get("pattern"))
        .and_then(serde_json::Value::as_str)
        .map(str::to_owned)
}
