use g3ts_astro_media_types::{G3TsAstroMediaPolicySnapshot, G3TsAstroMediaPolicySurfaceState};

/// `POLICY_RULES` constant.
const POLICY_RULES: [&str; 4] = [
    "astro-media-policy/no-raw-public-image-paths",
    "astro-media-policy/no-inline-image-alt",
    "astro-media-policy/require-content-image-key",
    "astro-media-policy/require-approved-media-helper",
];

/// `common_plugins`: common plugins.
pub(crate) fn common_plugins(
    probes: &[&eslint_config_parser::types::EslintEffectiveConfigProbe],
) -> Vec<String> {
    common_values(probes, |probe| probe.plugins.clone())
}

/// Alias for the eslint plugin-to-packages mapping.
type PluginPackageNames = std::collections::BTreeMap<String, Vec<String>>;

/// `common_plugin_package_names`: common plugin package names.
pub(crate) fn common_plugin_package_names(
    probes: &[&eslint_config_parser::types::EslintEffectiveConfigProbe],
) -> PluginPackageNames {
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

/// `common_error_rules`: common error rules.
pub(crate) fn common_error_rules(
    probes: &[&eslint_config_parser::types::EslintEffectiveConfigProbe],
) -> Vec<String> {
    common_values(probes, active_error_rules)
}

/// `common_effective_media_policy_rules`: common effective media policy rules.
pub(crate) fn common_effective_media_policy_rules(
    probes: &[&eslint_config_parser::types::EslintEffectiveConfigProbe],
    astro_policy: &G3TsAstroMediaPolicySurfaceState,
) -> Vec<String> {
    let G3TsAstroMediaPolicySurfaceState::Parsed { snapshot } = astro_policy else {
        return Vec::new();
    };

    POLICY_RULES
        .into_iter()
        .filter(|rule| {
            !probes.is_empty()
                && probes
                    .iter()
                    .all(|probe| probe_enforces_rule(probe, rule, snapshot))
        })
        .map(str::to_owned)
        .collect()
}

/// Returns `true` when the probe enforces `rule` at error severity with options that match `snapshot`.
fn probe_enforces_rule(
    probe: &eslint_config_parser::types::EslintEffectiveConfigProbe,
    rule: &str,
    snapshot: &G3TsAstroMediaPolicySnapshot,
) -> bool {
    probe.rules.get(rule).is_some_and(|setting| {
        setting.severity == eslint_config_parser::types::EslintRuleSeverity::Error
            && setting
                .options
                .first()
                .is_some_and(|options| rule_options_match_policy(rule, options, snapshot))
    })
}

/// `common_restricted_disable_patterns`: common restricted disable patterns.
pub(crate) fn common_restricted_disable_patterns(
    probes: &[&eslint_config_parser::types::EslintEffectiveConfigProbe],
) -> Vec<String> {
    common_values(probes, restricted_disable_patterns)
}

/// `active_error_rules`: active error rules.
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

/// `common_values`: common values.
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

/// `rule_options_match_policy`: rule options match policy.
fn rule_options_match_policy(
    rule: &str,
    options: &serde_json::Value,
    policy: &g3ts_astro_media_types::G3TsAstroMediaPolicySnapshot,
) -> bool {
    if !all_policy_options_match(options, policy) {
        return false;
    }

    match rule {
        "astro-media-policy/no-raw-public-image-paths" => {
            string_array_option_matches(
                options,
                "allowedPublicImagePaths",
                &policy.allowed_public_image_paths,
            ) && string_array_option_matches(
                options,
                "checkedImageExtensions",
                &policy.checked_image_extensions,
            ) && string_array_option_matches(
                options,
                "approvedMediaHelpers",
                &policy.approved_media_helpers,
            )
        }
        "astro-media-policy/no-inline-image-alt" => {
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
        "astro-media-policy/require-content-image-key" => {
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
        "astro-media-policy/require-approved-media-helper" => {
            string_array_option_matches(options, "mediaHelperModules", &policy.media_helper_modules)
                && string_array_option_matches(
                    options,
                    "approvedMediaHelpers",
                    &policy.approved_media_helpers,
                )
                && string_array_option_matches(
                    options,
                    "metadataImagePropertyNames",
                    &policy.metadata_image_property_names,
                )
        }
        _ => false,
    }
}

/// `all_policy_options_match`: all policy options match.
fn all_policy_options_match(
    options: &serde_json::Value,
    policy: &g3ts_astro_media_types::G3TsAstroMediaPolicySnapshot,
) -> bool {
    let Some(object) = options.as_object() else {
        return false;
    };
    let allowed_keys = [
        "publicSourceGlobs",
        "mediaHelperModules",
        "approvedMediaHelpers",
        "contentImageComponents",
        "contentImageKeyProps",
        "bannedImageSourceProps",
        "bannedImageAltProps",
        "allowedPublicImagePaths",
        "checkedImageExtensions",
        "metadataImagePropertyNames",
    ];

    object
        .keys()
        .all(|key| allowed_keys.iter().any(|allowed| allowed == key))
        && string_array_option_matches(options, "publicSourceGlobs", &policy.public_source_globs)
        && string_array_option_matches(options, "mediaHelperModules", &policy.media_helper_modules)
        && string_array_option_matches(
            options,
            "approvedMediaHelpers",
            &policy.approved_media_helpers,
        )
        && string_array_option_matches(
            options,
            "contentImageComponents",
            &policy.content_image_components,
        )
        && string_array_option_matches(
            options,
            "contentImageKeyProps",
            &policy.content_image_key_props,
        )
        && string_array_option_matches(
            options,
            "bannedImageSourceProps",
            &policy.banned_image_source_props,
        )
        && string_array_option_matches(
            options,
            "bannedImageAltProps",
            &policy.banned_image_alt_props,
        )
        && string_array_option_matches(
            options,
            "allowedPublicImagePaths",
            &policy.allowed_public_image_paths,
        )
        && string_array_option_matches(
            options,
            "checkedImageExtensions",
            &policy.checked_image_extensions,
        )
        && string_array_option_matches(
            options,
            "metadataImagePropertyNames",
            &policy.metadata_image_property_names,
        )
}

/// `string_array_option_matches`: string array option matches.
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

/// `normalized_string_set`: normalized string set.
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

/// `restricted_disable_patterns`: restricted disable patterns.
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

/// `disable_patterns_from_option`: disable patterns from option.
fn disable_patterns_from_option(option: &serde_json::Value) -> Vec<String> {
    if let Some(items) = option.as_array() {
        return items.iter().filter_map(disable_pattern).collect();
    }

    disable_pattern(option).into_iter().collect()
}

/// `disable_pattern`: disable pattern.
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
