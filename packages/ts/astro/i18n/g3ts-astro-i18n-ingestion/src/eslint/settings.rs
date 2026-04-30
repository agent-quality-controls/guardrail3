use g3ts_astro_i18n_types::G3TsAstroI18nPolicySurfaceState;

const POLICY_RULES: [&str; 1] = ["astro-i18n-policy/no-unlocalized-internal-hrefs"];
const NO_RESTRICTED_SYNTAX: &str = "no-restricted-syntax";

pub(crate) fn common_plugins(
    probes: &[&eslint_config_parser::types::EslintEffectiveConfigProbe],
) -> Vec<String> {
    common_values(probes, |probe| probe.plugins.clone())
}

pub(crate) fn common_plugin_package_names(
    probes: &[&eslint_config_parser::types::EslintEffectiveConfigProbe],
) -> std::collections::BTreeMap<String, Vec<String>> {
    let Some(first) = probes.first() else {
        return std::collections::BTreeMap::new();
    };
    first
        .plugin_package_names
        .iter()
        .filter_map(|(plugin, packages)| {
            let common = probes.iter().skip(1).fold(
                packages
                    .iter()
                    .cloned()
                    .collect::<std::collections::BTreeSet<_>>(),
                |mut acc, probe| {
                    let current = probe
                        .plugin_package_names
                        .get(plugin)
                        .map(|values| {
                            values
                                .iter()
                                .cloned()
                                .collect::<std::collections::BTreeSet<_>>()
                        })
                        .unwrap_or_default();
                    acc.retain(|value| current.contains(value));
                    acc
                },
            );
            (!common.is_empty()).then(|| (plugin.clone(), common.into_iter().collect()))
        })
        .collect()
}

pub(crate) fn common_error_rules(
    probes: &[&eslint_config_parser::types::EslintEffectiveConfigProbe],
) -> Vec<String> {
    common_values(probes, active_error_rules)
}

pub(crate) fn common_effective_i18n_policy_rules(
    probes: &[&eslint_config_parser::types::EslintEffectiveConfigProbe],
    astro_policy: &G3TsAstroI18nPolicySurfaceState,
) -> Vec<String> {
    let G3TsAstroI18nPolicySurfaceState::Parsed { snapshot } = astro_policy else {
        return Vec::new();
    };

    POLICY_RULES
        .into_iter()
        .filter(|rule| {
            !probes.is_empty()
                && probes.iter().all(|probe| {
                    probe.rules.get(*rule).is_some_and(|setting| {
                        setting.severity == eslint_config_parser::types::EslintRuleSeverity::Error
                            && setting.options.first().is_some_and(|options| {
                                rule_options_match_policy(rule, options, snapshot)
                            })
                    })
                })
        })
        .map(str::to_owned)
        .collect()
}

pub(crate) fn common_no_restricted_syntax_selectors(
    probes: &[&eslint_config_parser::types::EslintEffectiveConfigProbe],
) -> Vec<String> {
    common_values(probes, no_restricted_syntax_selectors)
}

pub(crate) fn union_no_restricted_syntax_selectors(
    probes: &[&eslint_config_parser::types::EslintEffectiveConfigProbe],
) -> Vec<String> {
    probes
        .iter()
        .flat_map(|probe| no_restricted_syntax_selectors(probe))
        .collect::<std::collections::BTreeSet<_>>()
        .into_iter()
        .collect()
}

pub(crate) fn common_restricted_disable_patterns(
    probes: &[&eslint_config_parser::types::EslintEffectiveConfigProbe],
) -> Vec<String> {
    common_values(probes, restricted_disable_patterns)
}

fn active_error_rules(
    probe: &eslint_config_parser::types::EslintEffectiveConfigProbe,
) -> Vec<String> {
    probe
        .rules
        .iter()
        .filter(|(_, setting)| {
            setting.severity == eslint_config_parser::types::EslintRuleSeverity::Error
        })
        .map(|(rule, _)| rule.clone())
        .collect()
}

fn common_values(
    probes: &[&eslint_config_parser::types::EslintEffectiveConfigProbe],
    extract: impl Fn(&eslint_config_parser::types::EslintEffectiveConfigProbe) -> Vec<String>,
) -> Vec<String> {
    let Some(first) = probes.first() else {
        return Vec::new();
    };
    let mut common = extract(first)
        .into_iter()
        .collect::<std::collections::BTreeSet<_>>();
    for probe in probes.iter().skip(1) {
        let current = extract(probe)
            .into_iter()
            .collect::<std::collections::BTreeSet<_>>();
        common.retain(|value| current.contains(value));
    }
    common.into_iter().collect()
}

fn rule_options_match_policy(
    rule: &str,
    options: &serde_json::Value,
    policy: &g3ts_astro_i18n_types::G3TsAstroI18nPolicySnapshot,
) -> bool {
    match rule {
        "astro-i18n-policy/no-unlocalized-internal-hrefs" => {
            let mut expected_keys = vec![
                "locales",
                "requireLocalePrefixForContentRoutes",
                "allowedUnprefixedRoutes",
                "contentRoutePrefixes",
                "checkedInternalLinkHelpers",
                "approvedInternalLinkHelpers",
                "approvedLocalizedLinkComponents",
            ];
            if policy.default_locale.is_some() {
                expected_keys.push("defaultLocale");
            }

            exact_option_keys_match(options, &expected_keys)
                && string_array_option_matches(options, "locales", &policy.locales)
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
                    "checkedInternalLinkHelpers",
                    &policy.checked_internal_link_helpers,
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
        _ => false,
    }
}

fn exact_option_keys_match(options: &serde_json::Value, expected: &[&str]) -> bool {
    let Some(object) = options.as_object() else {
        return false;
    };

    let actual = object
        .keys()
        .map(String::as_str)
        .collect::<std::collections::BTreeSet<_>>();
    let expected = expected
        .iter()
        .copied()
        .collect::<std::collections::BTreeSet<_>>();

    actual == expected
}

fn string_array_option_matches(
    options: &serde_json::Value,
    key: &str,
    expected: &[String],
) -> bool {
    let Some(actual) = options.get(key).and_then(serde_json::Value::as_array) else {
        return false;
    };
    let Some(actual) = normalized_string_set(actual) else {
        return false;
    };
    let expected = expected
        .iter()
        .map(String::as_str)
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .collect::<std::collections::BTreeSet<_>>();

    actual == expected
}

fn normalized_string_set(values: &[serde_json::Value]) -> Option<std::collections::BTreeSet<&str>> {
    values
        .iter()
        .map(|value| {
            value
                .as_str()
                .map(str::trim)
                .filter(|value| !value.is_empty())
        })
        .collect::<Option<std::collections::BTreeSet<_>>>()
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
    probe: &eslint_config_parser::types::EslintEffectiveConfigProbe,
) -> Vec<String> {
    let Some(setting) = probe.rules.get(NO_RESTRICTED_SYNTAX) else {
        return Vec::new();
    };
    if setting.severity != eslint_config_parser::types::EslintRuleSeverity::Error {
        return Vec::new();
    }

    setting
        .options
        .iter()
        .filter_map(|option| option.get("selector"))
        .filter_map(serde_json::Value::as_str)
        .map(str::to_owned)
        .collect()
}

fn restricted_disable_patterns(
    probe: &eslint_config_parser::types::EslintEffectiveConfigProbe,
) -> Vec<String> {
    let Some(setting) = probe
        .rules
        .get("@eslint-community/eslint-comments/no-restricted-disable")
    else {
        return Vec::new();
    };
    if setting.severity != eslint_config_parser::types::EslintRuleSeverity::Error {
        return Vec::new();
    }

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
