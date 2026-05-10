use g3_workspace_crawl::G3RsWorkspaceCrawl as G3WorkspaceCrawl;
use g3ts_astro_check_support::surfaces::{
    G3TsAstroRawEslintConfigState, read_eslint_config_surface,
};
use g3ts_astro_content_types::{
    G3TsAstroContentEslintSurfaceSnapshot, G3TsAstroContentEslintSurfaceState,
    G3TsAstroContentPolicySurfaceState, G3TsAstroPipelineRuleScopeSnapshot,
};

/// Constant `ROUTE_SCOPED_PIPELINE_RULES`.
const ROUTE_SCOPED_PIPELINE_RULES: [&str; 8] = [
    "astro-pipeline/no-authored-content-fs-read",
    "astro-pipeline/no-authored-content-glob",
    "astro-pipeline/no-authored-content-imports",
    "astro-pipeline/no-content-data-modules-in-routes",
    "astro-pipeline/no-direct-astro-content-in-routes",
    "astro-pipeline/require-approved-content-adapter-in-routes",
    "astro-pipeline/no-side-loader-imports",
    "astro-pipeline/no-velite-imports",
];
/// Constant `INLINE_PUBLIC_CONTENT_RULE`.
const INLINE_PUBLIC_CONTENT_RULE: &str = "i18next/no-literal-string";
/// Constant `INLINE_PUBLIC_CONTENT_MESSAGE`.
const INLINE_PUBLIC_CONTENT_MESSAGE: &str = "Inline public copy must live in Astro content entries. Move this text into the content collection, validate it through the collection schema, and pass the typed value into source.";
/// Constant `CONTENT_ADAPTER_PIPELINE_RULE`.
const CONTENT_ADAPTER_PIPELINE_RULE: &str =
    "astro-pipeline/require-approved-content-adapter-in-routes";

/// Ingest the `ESLint` configuration surface for the content family checks
/// rooted at `app_root_rel_path`.
#[must_use]
pub(crate) fn ingest_content_eslint_surface(
    crawl: &G3WorkspaceCrawl,
    app_root_rel_path: &str,
    _astro_policy: &G3TsAstroContentPolicySurfaceState,
) -> G3TsAstroContentEslintSurfaceState {
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

    G3TsAstroContentEslintSurfaceState::Parsed {
        snapshot: G3TsAstroContentEslintSurfaceSnapshot {
            rel_path,
            astro_source_probe_present: astro.is_some(),
            ts_source_probe_present: ts.is_some(),
            tsx_source_probe_present: tsx.is_some(),
            astro_source_plugins: plugins(astro),
            ts_source_plugins: plugins(ts),
            tsx_source_plugins: plugins(tsx),
            astro_source_error_rules: active_error_rules(astro),
            ts_source_error_rules: active_error_rules(ts),
            tsx_source_error_rules: active_error_rules(tsx),
            astro_source_effective_content_adapter_modules: effective_content_adapter_modules(
                astro,
            ),
            ts_source_effective_content_adapter_modules: effective_content_adapter_modules(ts),
            tsx_source_effective_content_adapter_modules: effective_content_adapter_modules(tsx),
            astro_source_route_scoped_pipeline_rule_scopes: route_scoped_pipeline_rule_scopes(
                astro,
            ),
            ts_source_route_scoped_pipeline_rule_scopes: route_scoped_pipeline_rule_scopes(ts),
            tsx_source_route_scoped_pipeline_rule_scopes: route_scoped_pipeline_rule_scopes(tsx),
            astro_source_effective_inline_public_content_rules:
                effective_inline_public_content_rules(astro),
            ts_source_effective_inline_public_content_rules: effective_inline_public_content_rules(
                ts,
            ),
            tsx_source_effective_inline_public_content_rules: effective_inline_public_content_rules(
                tsx,
            ),
            astro_source_warn_or_error_rules: active_warn_or_error_rules(astro),
            ts_source_warn_or_error_rules: active_warn_or_error_rules(ts),
            tsx_source_warn_or_error_rules: active_warn_or_error_rules(tsx),
            astro_source_restricted_disable_patterns: restricted_disable_patterns(astro),
            ts_source_restricted_disable_patterns: restricted_disable_patterns(ts),
            tsx_source_restricted_disable_patterns: restricted_disable_patterns(tsx),
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

/// Helper `probe_targets`.
fn probe_targets(
    crawl: &G3WorkspaceCrawl,
    app_root_rel_path: &str,
) -> Vec<eslint_config_parser::types::EslintProbeTarget> {
    vec![
        source_probe(
            crawl,
            app_root_rel_path,
            eslint_config_parser::types::EslintProbeKind::AstroSource,
            |rel_path| rel_path.starts_with("src/") && has_extension_ascii_ci(rel_path, "astro"),
            "src/__g3ts_probe__.astro",
        ),
        source_probe(
            crawl,
            app_root_rel_path,
            eslint_config_parser::types::EslintProbeKind::TsSource,
            |rel_path| rel_path.starts_with("src/") && has_extension_ascii_ci(rel_path, "ts"),
            "src/index.ts",
        ),
        source_probe(
            crawl,
            app_root_rel_path,
            eslint_config_parser::types::EslintProbeKind::TsxSource,
            |rel_path| rel_path.starts_with("src/") && has_extension_ascii_ci(rel_path, "tsx"),
            "src/__g3ts_probe__.tsx",
        ),
    ]
}

/// Helper `source_probe`.
fn source_probe(
    crawl: &G3WorkspaceCrawl,
    app_root_rel_path: &str,
    probe: eslint_config_parser::types::EslintProbeKind,
    predicate: impl Fn(&str) -> bool,
    fallback_rel_path: &str,
) -> eslint_config_parser::types::EslintProbeTarget {
    eslint_config_parser::types::EslintProbeTarget {
        probe,
        rel_path: first_matching_app_rel_path(crawl, app_root_rel_path, predicate).unwrap_or_else(
            || {
                g3ts_astro_check_support::surfaces::scoped_rel_path(
                    app_root_rel_path,
                    fallback_rel_path,
                )
            },
        ),
    }
}

/// Helper `first_matching_app_rel_path`.
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

/// Helper `map_raw_state`.
fn map_raw_state(raw: G3TsAstroRawEslintConfigState) -> G3TsAstroContentEslintSurfaceState {
    match raw {
        G3TsAstroRawEslintConfigState::Missing { rel_path } => {
            G3TsAstroContentEslintSurfaceState::Missing { rel_path }
        }
        G3TsAstroRawEslintConfigState::Unreadable { rel_path, reason } => {
            G3TsAstroContentEslintSurfaceState::Unreadable { rel_path, reason }
        }
        G3TsAstroRawEslintConfigState::ParseError { rel_path, reason } => {
            G3TsAstroContentEslintSurfaceState::ParseError { rel_path, reason }
        }
        G3TsAstroRawEslintConfigState::Parsed { rel_path, .. } => {
            G3TsAstroContentEslintSurfaceState::ParseError {
                rel_path,
                reason: "parsed raw eslint state reached content error mapper".to_owned(),
            }
        }
    }
}

/// Helper `active_probe`.
fn active_probe(
    typed: &eslint_config_parser::types::EslintConfigSnapshot,
    kind: eslint_config_parser::types::EslintProbeKind,
) -> Option<&eslint_config_parser::types::EslintEffectiveConfigProbe> {
    probe_by_kind(typed, kind).filter(|probe| !probe.ignored)
}

/// Helper `probe_by_kind`.
fn probe_by_kind(
    typed: &eslint_config_parser::types::EslintConfigSnapshot,
    kind: eslint_config_parser::types::EslintProbeKind,
) -> Option<&eslint_config_parser::types::EslintEffectiveConfigProbe> {
    typed.probes.iter().find(|probe| probe.probe == kind)
}

/// Helper `probe_ignored`.
fn probe_ignored(
    typed: &eslint_config_parser::types::EslintConfigSnapshot,
    kind: eslint_config_parser::types::EslintProbeKind,
) -> bool {
    probe_by_kind(typed, kind).is_none_or(|probe| probe.ignored)
}

/// Helper `plugins`.
fn plugins(probe: Option<&eslint_config_parser::types::EslintEffectiveConfigProbe>) -> Vec<String> {
    probe.map_or_else(Vec::new, |probe| probe.plugins.clone())
}

/// Helper `active_error_rules`.
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

/// Helper `active_warn_or_error_rules`.
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

/// Helper `restricted_disable_patterns`.
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

/// Helper `route_scoped_pipeline_rule_scopes`.
fn route_scoped_pipeline_rule_scopes(
    probe: Option<&eslint_config_parser::types::EslintEffectiveConfigProbe>,
) -> Vec<G3TsAstroPipelineRuleScopeSnapshot> {
    let Some(probe) = probe else {
        return Vec::new();
    };

    ROUTE_SCOPED_PIPELINE_RULES
        .iter()
        .filter_map(|rule_name| {
            let setting = probe.rules.get(*rule_name)?;
            if !rule_setting_is_error(setting) {
                return None;
            }
            Some(G3TsAstroPipelineRuleScopeSnapshot {
                rule_name: (*rule_name).to_owned(),
                route_globs: string_array_option(setting, "routeGlobs"),
                endpoint_globs: string_array_option(setting, "endpointGlobs"),
            })
        })
        .collect()
}

/// Helper `effective_content_adapter_modules`.
fn effective_content_adapter_modules(
    probe: Option<&eslint_config_parser::types::EslintEffectiveConfigProbe>,
) -> Vec<String> {
    let Some(probe) = probe else {
        return Vec::new();
    };

    probe
        .rules
        .get(CONTENT_ADAPTER_PIPELINE_RULE)
        .map_or_else(Vec::new, |setting| {
            if rule_setting_is_error(setting)
                && probe_has_pipeline_plugin_package(probe)
                && !string_array_option(setting, "approvedContentAdapterModules").is_empty()
            {
                string_array_option(setting, "approvedContentAdapterModules")
            } else {
                Vec::new()
            }
        })
}

/// Helper `effective_inline_public_content_rules`.
fn effective_inline_public_content_rules(
    probe: Option<&eslint_config_parser::types::EslintEffectiveConfigProbe>,
) -> Vec<String> {
    let Some(probe) = probe else {
        return Vec::new();
    };

    probe
        .rules
        .get(INLINE_PUBLIC_CONTENT_RULE)
        .map_or_else(Vec::new, |setting| {
            if rule_setting_is_error(setting)
                && rule_setting_has_inline_public_content_policy(setting)
            {
                vec![INLINE_PUBLIC_CONTENT_RULE.to_owned()]
            } else {
                Vec::new()
            }
        })
}

/// Helper `probe_has_pipeline_plugin_package`.
fn probe_has_pipeline_plugin_package(
    probe: &eslint_config_parser::types::EslintEffectiveConfigProbe,
) -> bool {
    probe
        .plugin_package_names
        .get("astro-pipeline")
        .is_some_and(|package_names| {
            package_names
                .iter()
                .any(|name| name == "g3ts-eslint-plugin-astro-pipeline")
        })
}

/// Helper `rule_setting_has_inline_public_content_policy`.
fn rule_setting_has_inline_public_content_policy(
    setting: &eslint_config_parser::types::EslintRuleSetting,
) -> bool {
    let Some(object) = setting
        .options
        .first()
        .and_then(serde_json::Value::as_object)
    else {
        return false;
    };

    object.len() == 10
        && inline_public_scalar_options_match(object)
        && inline_public_collection_options_match(object)
}

/// Returns `true` when the scalar option fields of the inline-public-content
/// rule match the expected baseline.
fn inline_public_scalar_options_match(object: JsonObjectRef<'_>) -> bool {
    object_string_value(object.get("framework")) == Some("react")
        && object_string_value(object.get("mode")) == Some("all")
        && object_string_value(object.get("message")) == Some(INLINE_PUBLIC_CONTENT_MESSAGE)
        && object_bool_value(object.get("should-validate-template")) == Some(true)
}

/// Returns `true` when every collection-shaped option field of the
/// inline-public-content rule matches the expected baseline arrays.
fn inline_public_collection_options_match(object: JsonObjectRef<'_>) -> bool {
    object_has_exact_string_arrays(
        object.get("words"),
        "include",
        &[],
        "exclude",
        &["[0-9!-/:-@[-`{-~]+", "[A-Z_-]+"],
    ) && object_has_exact_string_arrays(
        object.get("jsx-components"),
        "include",
        &[],
        "exclude",
        &[],
    ) && object_has_exact_string_arrays(
        object.get("jsx-attributes"),
        "include",
        &[],
        "exclude",
        &[
            "as",
            "class",
            "className",
            "color",
            "data-.+",
            "height",
            "href",
            "id",
            "intent",
            "key",
            "name",
            "rel",
            "role",
            "size",
            "slot",
            "src",
            "style",
            "styleName",
            "target",
            "tone",
            "type",
            "variant",
            "width",
            "aria-hidden",
        ],
    ) && object_has_exact_string_arrays(
        object.get("callees"),
        "include",
        &[],
        "exclude",
        &[
            "require", "clsx", "cn", "cx", "cva", "twMerge", "twJoin", "tv", "URL",
        ],
    ) && object_has_exact_string_arrays(
        object.get("object-properties"),
        "include",
        &[],
        "exclude",
        &["[A-Z_-]+"],
    ) && object_has_exact_string_arrays(
        object.get("class-properties"),
        "include",
        &[],
        "exclude",
        &["displayName"],
    )
}

/// Helper `object_has_exact_string_arrays`.
fn object_has_exact_string_arrays(
    value: Option<&serde_json::Value>,
    first_key: &str,
    first_expected: &[&str],
    second_key: &str,
    second_expected: &[&str],
) -> bool {
    let Some(object) = value.and_then(serde_json::Value::as_object) else {
        return false;
    };

    object.len() == 2
        && string_array_exactly(object.get(first_key), first_expected)
        && string_array_exactly(object.get(second_key), second_expected)
}

/// Helper `string_array_exactly`.
fn string_array_exactly(value: Option<&serde_json::Value>, expected: &[&str]) -> bool {
    let Some(values) = value.and_then(serde_json::Value::as_array) else {
        return false;
    };

    values.len() == expected.len()
        && values
            .iter()
            .zip(expected.iter().copied())
            .all(|(actual, expected_str)| actual.as_str() == Some(expected_str))
}

/// Decode `option` as a borrowed JSON string.
fn object_string_value(option: Option<&serde_json::Value>) -> Option<&str> {
    option.and_then(serde_json::Value::as_str)
}

/// Decode `option` as a JSON bool, treating non-bool values as `None`.
fn object_bool_value(option: Option<&serde_json::Value>) -> Option<bool> {
    option.and_then(serde_json::Value::as_bool)
}

/// Convenience alias for a borrowed JSON object map (`{ String -> Value }`)
/// that backs `ESLint` rule option payloads.
type JsonObjectRef<'a> = &'a serde_json::Map<String, serde_json::Value>;

/// Helper `first_option_object`.
fn first_option_object(
    setting: &eslint_config_parser::types::EslintRuleSetting,
) -> Option<JsonObjectRef<'_>> {
    setting
        .options
        .first()
        .and_then(serde_json::Value::as_object)
}

/// Helper `rule_setting_is_error`.
fn rule_setting_is_error(setting: &eslint_config_parser::types::EslintRuleSetting) -> bool {
    setting.severity == eslint_config_parser::types::EslintRuleSeverity::Error
}

/// Helper `string_array_option`.
fn string_array_option(
    setting: &eslint_config_parser::types::EslintRuleSetting,
    option_name: &str,
) -> Vec<String> {
    first_option_object(setting)
        .and_then(|object| object.get(option_name))
        .and_then(serde_json::Value::as_array)
        .map_or_else(Vec::new, |values| {
            values
                .iter()
                .filter_map(serde_json::Value::as_str)
                .map(str::trim)
                .filter(|value| !value.is_empty())
                .map(str::to_owned)
                .collect()
        })
}

/// Returns `true` when `rel_path` has the given `extension`, comparing the
/// extension byte-wise in an ASCII-case-insensitive manner.
fn has_extension_ascii_ci(rel_path: &str, extension: &str) -> bool {
    std::path::Path::new(rel_path)
        .extension()
        .is_some_and(|actual| actual.eq_ignore_ascii_case(extension))
}
