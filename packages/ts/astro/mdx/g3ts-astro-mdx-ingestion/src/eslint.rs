use g3_workspace_crawl::G3WorkspaceCrawl;
use g3ts_astro_check_support::surfaces::{
    G3TsAstroRawEslintConfigState, read_eslint_config_surface,
};
use g3ts_astro_mdx_types::{
    G3TsAstroMdxEslintSurfaceSnapshot, G3TsAstroMdxEslintSurfaceState,
    G3TsAstroMdxPolicySurfaceState,
};

/// `MDX_COMPONENT_MAP_PIPELINE_RULE` constant.
const MDX_COMPONENT_MAP_PIPELINE_RULE: &str =
    "astro-pipeline/mdx-component-imports-from-approved-map";
/// `MDX_NAMED_COMPONENT_IMPORT_RULE` constant.
const MDX_NAMED_COMPONENT_IMPORT_RULE: &str = "astro-pipeline/mdx-imports-only-approved-components";
/// `MDX_NO_RAW_IMAGE_RULE` constant.
const MDX_NO_RAW_IMAGE_RULE: &str = "astro-pipeline/no-raw-mdx-images";
/// `MDX_NO_RAW_UI_EXPORT_RULE` constant.
const MDX_NO_RAW_UI_EXPORT_RULE: &str = "astro-pipeline/mdx-component-map-no-raw-ui-exports";
/// `MDX_WRAPPER_ZOD_PARSE_RULE` constant.
const MDX_WRAPPER_ZOD_PARSE_RULE: &str = "astro-pipeline/mdx-component-wrapper-requires-zod-parse";

#[must_use]
/// `ingest_mdx_eslint_surface` helper.
pub(crate) fn ingest_mdx_eslint_surface(
    crawl: &G3WorkspaceCrawl,
    app_root_rel_path: &str,
    astro_policy: &G3TsAstroMdxPolicySurfaceState,
) -> G3TsAstroMdxEslintSurfaceState {
    let raw = read_eslint_config_surface(
        crawl,
        app_root_rel_path,
        &probe_targets(crawl, app_root_rel_path, astro_policy),
    );
    let G3TsAstroRawEslintConfigState::Parsed { rel_path, snapshot } = raw else {
        return map_raw_state(raw);
    };

    let mdx_probe = active_probe(
        &snapshot,
        eslint_config_parser::types::EslintProbeKind::MdxContent,
    );
    let component_map_probe = active_probe(
        &snapshot,
        eslint_config_parser::types::EslintProbeKind::TsxSource,
    );
    let mdx_paths = mdx_content_paths(crawl, app_root_rel_path, astro_policy);
    let component_maps = mdx_component_map_policy_paths(astro_policy);

    G3TsAstroMdxEslintSurfaceState::Parsed {
        snapshot: G3TsAstroMdxEslintSurfaceSnapshot {
            rel_path,
            mdx_content_probe_present: mdx_probe.is_some(),
            mdx_content_plugins: plugins(mdx_probe),
            mdx_content_plugin_package_names: plugin_package_names(mdx_probe),
            mdx_content_error_rules: crate::eslint_suppression::active_error_rules(mdx_probe),
            mdx_content_warn_or_error_rules: crate::eslint_suppression::active_warn_or_error_rules(
                mdx_probe,
            ),
            mdx_content_restricted_disable_patterns:
                crate::eslint_suppression::restricted_disable_patterns(mdx_probe),
            mdx_content_unused_disable_fail_closed:
                crate::eslint_suppression::unused_disable_fail_closed(mdx_probe),
            mdx_content_effective_mdx_component_map_rules: effective_mdx_component_map_rules(
                mdx_probe,
                &mdx_paths,
                &component_maps,
            ),
            mdx_content_effective_named_component_import_rules:
                effective_mdx_named_component_import_rules(mdx_probe, &mdx_paths, &component_maps),
            mdx_content_effective_no_raw_image_rules: effective_mdx_no_raw_image_rules(
                mdx_probe, &mdx_paths,
            ),
            component_map_probe_present: component_map_probe.is_some(),
            component_map_plugins: plugins(component_map_probe),
            component_map_plugin_package_names: plugin_package_names(component_map_probe),
            component_map_error_rules: crate::eslint_suppression::active_error_rules(
                component_map_probe,
            ),
            component_map_warn_or_error_rules:
                crate::eslint_suppression::active_warn_or_error_rules(component_map_probe),
            component_map_restricted_disable_patterns:
                crate::eslint_suppression::restricted_disable_patterns(component_map_probe),
            component_map_unused_disable_fail_closed:
                crate::eslint_suppression::unused_disable_fail_closed(component_map_probe),
            component_map_effective_no_raw_ui_export_rules:
                effective_component_map_no_raw_ui_export_rules(component_map_probe, &component_maps),
            component_map_effective_wrapper_zod_parse_rules:
                effective_component_map_wrapper_zod_parse_rules(
                    component_map_probe,
                    &component_maps,
                ),
            component_map_probe_ignored: probe_ignored(
                &snapshot,
                eslint_config_parser::types::EslintProbeKind::TsxSource,
            ),
            mdx_content_probe_ignored: probe_ignored(
                &snapshot,
                eslint_config_parser::types::EslintProbeKind::MdxContent,
            ),
        },
    }
}

/// `probe_targets` helper.
fn probe_targets(
    crawl: &G3WorkspaceCrawl,
    app_root_rel_path: &str,
    astro_policy: &G3TsAstroMdxPolicySurfaceState,
) -> Vec<eslint_config_parser::types::EslintProbeTarget> {
    vec![
        eslint_config_parser::types::EslintProbeTarget {
            probe: eslint_config_parser::types::EslintProbeKind::MdxContent,
            rel_path: mdx_content_paths(crawl, app_root_rel_path, astro_policy)
                .into_iter()
                .next()
                .unwrap_or_else(|| {
                    g3ts_astro_check_support::surfaces::scoped_rel_path(
                        app_root_rel_path,
                        "content/__g3ts_probe__.mdx",
                    )
                }),
        },
        eslint_config_parser::types::EslintProbeTarget {
            probe: eslint_config_parser::types::EslintProbeKind::TsxSource,
            rel_path: mdx_component_map_policy_paths(astro_policy)
                .into_iter()
                .next()
                .map_or_else(
                    || {
                        g3ts_astro_check_support::surfaces::scoped_rel_path(
                            app_root_rel_path,
                            "src/mdx-components.tsx",
                        )
                    },
                    |path| {
                        g3ts_astro_check_support::surfaces::scoped_rel_path(
                            app_root_rel_path,
                            &path,
                        )
                    },
                ),
        },
    ]
}

/// `map_raw_state` helper.
fn map_raw_state(raw: G3TsAstroRawEslintConfigState) -> G3TsAstroMdxEslintSurfaceState {
    match raw {
        G3TsAstroRawEslintConfigState::Missing { rel_path } => {
            G3TsAstroMdxEslintSurfaceState::Missing { rel_path }
        }
        G3TsAstroRawEslintConfigState::Unreadable { rel_path, reason } => {
            G3TsAstroMdxEslintSurfaceState::Unreadable { rel_path, reason }
        }
        G3TsAstroRawEslintConfigState::ParseError { rel_path, reason } => {
            G3TsAstroMdxEslintSurfaceState::ParseError { rel_path, reason }
        }
        G3TsAstroRawEslintConfigState::Parsed { rel_path, .. } => {
            G3TsAstroMdxEslintSurfaceState::ParseError {
                rel_path,
                reason: "parsed raw eslint state reached mdx error mapper".to_owned(),
            }
        }
    }
}

/// `active_probe` helper.
fn active_probe(
    typed: &eslint_config_parser::types::EslintConfigSnapshot,
    kind: eslint_config_parser::types::EslintProbeKind,
) -> Option<&eslint_config_parser::types::EslintEffectiveConfigProbe> {
    probe_by_kind(typed, kind).filter(|probe| !probe.ignored)
}

/// `probe_by_kind` helper.
fn probe_by_kind(
    typed: &eslint_config_parser::types::EslintConfigSnapshot,
    kind: eslint_config_parser::types::EslintProbeKind,
) -> Option<&eslint_config_parser::types::EslintEffectiveConfigProbe> {
    typed.probes.iter().find(|probe| probe.probe == kind)
}

/// `probe_ignored` helper.
fn probe_ignored(
    typed: &eslint_config_parser::types::EslintConfigSnapshot,
    kind: eslint_config_parser::types::EslintProbeKind,
) -> bool {
    probe_by_kind(typed, kind).is_none_or(|probe| probe.ignored)
}

/// `plugins` helper.
fn plugins(probe: Option<&eslint_config_parser::types::EslintEffectiveConfigProbe>) -> Vec<String> {
    probe.map_or_else(Vec::new, |probe| probe.plugins.clone())
}

/// Mapping from public `ESLint` plugin alias to package name(s).
type PluginPackageNames = std::collections::BTreeMap<String, Vec<String>>;

/// `plugin_package_names` helper.
fn plugin_package_names(
    probe: Option<&eslint_config_parser::types::EslintEffectiveConfigProbe>,
) -> PluginPackageNames {
    probe.map_or_else(std::collections::BTreeMap::new, |probe| {
        probe.plugin_package_names.clone()
    })
}

/// `effective_mdx_component_map_rules` helper.
fn effective_mdx_component_map_rules(
    probe: Option<&eslint_config_parser::types::EslintEffectiveConfigProbe>,
    mdx_content_paths: &[String],
    approved_mdx_component_modules: &[String],
) -> Vec<String> {
    let Some(probe) = probe else {
        return Vec::new();
    };

    probe
        .rules
        .get(MDX_COMPONENT_MAP_PIPELINE_RULE)
        .map_or_else(Vec::new, |setting| {
            if crate::eslint_helpers::rule_setting_is_error(setting)
                && crate::eslint_helpers::probe_has_pipeline_plugin_package(probe)
                && crate::eslint_helpers::rule_setting_has_option_globs_coverage(
                    setting,
                    "mdxContentGlobs",
                    mdx_content_paths,
                )
                && crate::eslint_helpers::rule_setting_has_expected_module_globs(
                    setting,
                    "approvedMdxComponentModules",
                    approved_mdx_component_modules,
                )
            {
                vec![MDX_COMPONENT_MAP_PIPELINE_RULE.to_owned()]
            } else {
                Vec::new()
            }
        })
}

/// `effective_mdx_named_component_import_rules` helper.
fn effective_mdx_named_component_import_rules(
    probe: Option<&eslint_config_parser::types::EslintEffectiveConfigProbe>,
    mdx_content_paths: &[String],
    approved_mdx_component_modules: &[String],
) -> Vec<String> {
    effective_rule_with_options(
        probe,
        MDX_NAMED_COMPONENT_IMPORT_RULE,
        &[
            (
                "mdxContentGlobs",
                OptionCheck::GlobsCover(mdx_content_paths),
            ),
            (
                "approvedMdxComponentModules",
                OptionCheck::ExpectedModules(approved_mdx_component_modules),
            ),
            ("approvedMdxComponentNames", OptionCheck::NonEmpty),
        ],
    )
}

/// `effective_mdx_no_raw_image_rules` helper.
fn effective_mdx_no_raw_image_rules(
    probe: Option<&eslint_config_parser::types::EslintEffectiveConfigProbe>,
    mdx_content_paths: &[String],
) -> Vec<String> {
    effective_rule_with_options(
        probe,
        MDX_NO_RAW_IMAGE_RULE,
        &[
            (
                "mdxContentGlobs",
                OptionCheck::GlobsCover(mdx_content_paths),
            ),
            ("approvedMdxImageComponents", OptionCheck::NonEmpty),
        ],
    )
}

/// `effective_component_map_no_raw_ui_export_rules` helper.
fn effective_component_map_no_raw_ui_export_rules(
    probe: Option<&eslint_config_parser::types::EslintEffectiveConfigProbe>,
    approved_mdx_component_modules: &[String],
) -> Vec<String> {
    effective_rule_with_options(
        probe,
        MDX_NO_RAW_UI_EXPORT_RULE,
        &[
            (
                "approvedMdxComponentModules",
                OptionCheck::ExpectedModules(approved_mdx_component_modules),
            ),
            ("rawUiModuleGlobs", OptionCheck::NonEmpty),
        ],
    )
}

/// `effective_component_map_wrapper_zod_parse_rules` helper.
fn effective_component_map_wrapper_zod_parse_rules(
    probe: Option<&eslint_config_parser::types::EslintEffectiveConfigProbe>,
    approved_mdx_component_modules: &[String],
) -> Vec<String> {
    effective_rule_with_options(
        probe,
        MDX_WRAPPER_ZOD_PARSE_RULE,
        &[
            (
                "approvedMdxComponentModules",
                OptionCheck::ExpectedModules(approved_mdx_component_modules),
            ),
            ("approvedMdxComponentNames", OptionCheck::NonEmpty),
            ("mdxPropsParserName", OptionCheck::NonEmpty),
        ],
    )
}

/// Per-option check predicate applied to an `ESLint` rule's options object.
enum OptionCheck<'a> {
    /// The option must equal exactly the given module list.
    ExpectedModules(&'a [String]),
    /// The option's glob list must cover the given source globs.
    GlobsCover(&'a [String]),
    /// The option must be present and non-empty.
    NonEmpty,
}

/// One named option check applied to a rule.
type OptionCheckEntry<'a> = (&'a str, OptionCheck<'a>);

/// `effective_rule_with_options` helper.
fn effective_rule_with_options(
    probe: Option<&eslint_config_parser::types::EslintEffectiveConfigProbe>,
    rule_name: &str,
    checks: &[OptionCheckEntry<'_>],
) -> Vec<String> {
    let Some(probe) = probe else {
        return Vec::new();
    };

    probe.rules.get(rule_name).map_or_else(Vec::new, |setting| {
        let checks_pass = checks.iter().all(|(key, check)| match check {
            OptionCheck::ExpectedModules(paths) => {
                crate::eslint_helpers::rule_setting_has_expected_module_globs(setting, key, paths)
            }
            OptionCheck::GlobsCover(paths) => {
                crate::eslint_helpers::rule_setting_has_option_globs_coverage(setting, key, paths)
            }
            OptionCheck::NonEmpty => {
                crate::eslint_helpers::rule_setting_option_globs_are_valid(setting, key)
            }
        });

        if crate::eslint_helpers::rule_setting_is_error(setting)
            && crate::eslint_helpers::probe_has_pipeline_plugin_package(probe)
            && checks_pass
        {
            vec![rule_name.to_owned()]
        } else {
            Vec::new()
        }
    })
}

/// `mdx_content_paths` helper.
fn mdx_content_paths(
    crawl: &G3WorkspaceCrawl,
    app_root_rel_path: &str,
    astro_policy: &G3TsAstroMdxPolicySurfaceState,
) -> Vec<String> {
    let content_root = match astro_policy {
        G3TsAstroMdxPolicySurfaceState::Parsed { snapshot } => snapshot.content_root.as_deref(),
        G3TsAstroMdxPolicySurfaceState::Missing { .. }
        | G3TsAstroMdxPolicySurfaceState::Unreadable { .. }
        | G3TsAstroMdxPolicySurfaceState::ParseError { .. }
        | G3TsAstroMdxPolicySurfaceState::MissingAstroPolicy { .. } => None,
    }
    .unwrap_or("src/content")
    .trim_end_matches('/');
    let scoped_content_root =
        g3ts_astro_check_support::surfaces::scoped_rel_path(app_root_rel_path, content_root);
    let scoped_prefix = format!("{scoped_content_root}/");

    crawl
        .entries
        .iter()
        .filter(|entry| {
            entry.kind == g3_workspace_crawl::G3WorkspaceEntryKind::File
                && entry.ignore_state == g3_workspace_crawl::G3WorkspaceIgnoreState::Included
                && entry.path.rel_path.starts_with(&scoped_prefix)
                && std::path::Path::new(&entry.path.rel_path)
                    .extension()
                    .is_some_and(|ext| ext.eq_ignore_ascii_case("mdx"))
        })
        .map(|entry| {
            g3ts_astro_check_support::surfaces::app_relative_path(
                &entry.path.rel_path,
                app_root_rel_path,
            )
        })
        .collect()
}

/// `mdx_component_map_policy_paths` helper.
fn mdx_component_map_policy_paths(astro_policy: &G3TsAstroMdxPolicySurfaceState) -> Vec<String> {
    let G3TsAstroMdxPolicySurfaceState::Parsed { snapshot } = astro_policy else {
        return Vec::new();
    };

    snapshot.mdx_component_maps.clone()
}
