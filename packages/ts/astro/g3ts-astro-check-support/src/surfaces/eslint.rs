use super::content::{content_adapter_policy_paths, policy_configured_paths};
use super::eslint_effective::*;
use super::prelude::*;
use super::constants::*;
use super::roots::{app_relative_path, scoped_rel_path};

fn mdx_content_paths(
    crawl: &G3WorkspaceCrawl,
    app_root_rel_path: &str,
    astro_policy: &G3TsAstroPolicySurfaceState,
) -> Vec<String> {
    let content_root = match astro_policy {
        G3TsAstroPolicySurfaceState::Parsed { snapshot } => snapshot.content_root.as_deref(),
        G3TsAstroPolicySurfaceState::Missing { .. }
        | G3TsAstroPolicySurfaceState::Unreadable { .. }
        | G3TsAstroPolicySurfaceState::ParseError { .. }
        | G3TsAstroPolicySurfaceState::MissingAstroPolicy { .. } => None,
    }
    .unwrap_or("src/content")
    .trim_end_matches('/');
    let scoped_content_root = scoped_rel_path(app_root_rel_path, content_root);
    let scoped_prefix = format!("{scoped_content_root}/");

    crawl
        .entries
        .iter()
        .filter(|entry| {
            entry.kind == g3_workspace_crawl::G3RsWorkspaceEntryKind::File
                && entry.ignore_state == g3_workspace_crawl::G3RsWorkspaceIgnoreState::Included
                && entry.path.rel_path.starts_with(&scoped_prefix)
                && entry.path.rel_path.ends_with(".mdx")
        })
        .map(|entry| app_relative_path(&entry.path.rel_path, app_root_rel_path))
        .collect()
}

pub fn ingest_eslint_surface(
    crawl: &G3WorkspaceCrawl,
    app_root_rel_path: &str,
    astro_policy: &G3TsAstroPolicySurfaceState,
) -> G3TsAstroEslintSurfaceState {
    let Some(entry) = crate::select::select_active_eslint_config(crawl, app_root_rel_path) else {
        return G3TsAstroEslintSurfaceState::Missing {
            rel_path: if app_root_rel_path == "." {
                ESLINT_CONFIG_PATTERN.to_owned()
            } else {
                format!("{app_root_rel_path}/{ESLINT_CONFIG_PATTERN}")
            },
        };
    };

    if !entry.readable {
        return G3TsAstroEslintSurfaceState::Unreadable {
            rel_path: entry.path.rel_path.clone(),
            reason: "workspace crawl marked the selected eslint config unreadable".to_owned(),
        };
    }

    let probes = crate::select::probe_targets(crawl, app_root_rel_path, &entry.path.rel_path);
    let document = match parse_eslint_document(&crawl.root_abs_path, &entry.path.rel_path, &probes)
    {
        Ok(document) => document,
        Err(error) => {
            return G3TsAstroEslintSurfaceState::ParseError {
                rel_path: entry.path.rel_path.clone(),
                reason: error.to_string(),
            };
        }
    };

    if let Some(reason) = eslint_parse_error_reason(&document) {
        return G3TsAstroEslintSurfaceState::ParseError {
            rel_path: entry.path.rel_path.clone(),
            reason: reason.to_owned(),
        };
    }

    let typed = eslint_config_parser::typed(&document)
        .expect("parsed eslint config document should stay typed");
    let route_page_paths = crate::select::route_page_paths(crawl, app_root_rel_path);
    let endpoint_paths = crate::select::endpoint_paths(crawl, app_root_rel_path);
    let content_adapter_policy_paths = content_adapter_policy_paths(astro_policy);
    let mdx_component_map_policy_paths =
        policy_configured_paths(astro_policy, |policy| &policy.mdx_component_maps);
    let metadata_helper_policy_paths =
        policy_configured_paths(astro_policy, |policy| &policy.metadata_helpers);
    let json_ld_helper_policy_paths =
        policy_configured_paths(astro_policy, |policy| &policy.json_ld_helpers);
    let mdx_content_paths = mdx_content_paths(crawl, app_root_rel_path, astro_policy);
    let astro_source_probe = active_probe(
        &typed,
        eslint_config_parser::types::EslintProbeKind::AstroSource,
    );
    let ts_source_probe = active_probe(
        &typed,
        eslint_config_parser::types::EslintProbeKind::TsSource,
    );
    let tsx_source_probe = active_probe(
        &typed,
        eslint_config_parser::types::EslintProbeKind::TsxSource,
    );
    let mdx_content_probe = active_probe(
        &typed,
        eslint_config_parser::types::EslintProbeKind::MdxContent,
    );

    G3TsAstroEslintSurfaceState::Parsed {
        snapshot: G3TsAstroEslintSurfaceSnapshot {
            rel_path: entry.path.rel_path.clone(),
            astro_source_probe_present: astro_source_probe.is_some(),
            ts_source_probe_present: ts_source_probe.is_some(),
            tsx_source_probe_present: tsx_source_probe.is_some(),
            mdx_content_probe_present: mdx_content_probe.is_some(),
            astro_source_plugins: astro_source_probe
                .map(|probe| probe.plugins.clone())
                .unwrap_or_default(),
            ts_source_plugins: ts_source_probe
                .map(|probe| probe.plugins.clone())
                .unwrap_or_default(),
            tsx_source_plugins: tsx_source_probe
                .map(|probe| probe.plugins.clone())
                .unwrap_or_default(),
            mdx_content_plugins: mdx_content_probe
                .map(|probe| probe.plugins.clone())
                .unwrap_or_default(),
            astro_source_plugin_meta_names: astro_source_probe
                .map(|probe| probe.plugin_meta_names.clone())
                .unwrap_or_default(),
            ts_source_plugin_meta_names: ts_source_probe
                .map(|probe| probe.plugin_meta_names.clone())
                .unwrap_or_default(),
            tsx_source_plugin_meta_names: tsx_source_probe
                .map(|probe| probe.plugin_meta_names.clone())
                .unwrap_or_default(),
            mdx_content_plugin_meta_names: mdx_content_probe
                .map(|probe| probe.plugin_meta_names.clone())
                .unwrap_or_default(),
            astro_source_plugin_package_names: astro_source_probe
                .map(|probe| probe.plugin_package_names.clone())
                .unwrap_or_default(),
            ts_source_plugin_package_names: ts_source_probe
                .map(|probe| probe.plugin_package_names.clone())
                .unwrap_or_default(),
            tsx_source_plugin_package_names: tsx_source_probe
                .map(|probe| probe.plugin_package_names.clone())
                .unwrap_or_default(),
            mdx_content_plugin_package_names: mdx_content_probe
                .map(|probe| probe.plugin_package_names.clone())
                .unwrap_or_default(),
            astro_source_error_rules: active_error_rules(astro_source_probe),
            ts_source_error_rules: active_error_rules(ts_source_probe),
            tsx_source_error_rules: active_error_rules(tsx_source_probe),
            mdx_content_error_rules: active_error_rules(mdx_content_probe),
            astro_source_effective_route_scoped_pipeline_rules:
                effective_route_scoped_pipeline_rules(
                    astro_source_probe,
                    &route_page_paths,
                    &endpoint_paths,
                ),
            ts_source_effective_route_scoped_pipeline_rules: effective_route_scoped_pipeline_rules(
                ts_source_probe,
                &route_page_paths,
                &endpoint_paths,
            ),
            tsx_source_effective_route_scoped_pipeline_rules: effective_route_scoped_pipeline_rules(
                tsx_source_probe,
                &route_page_paths,
                &endpoint_paths,
            ),
            astro_source_effective_content_adapter_modules: effective_content_adapter_modules(
                astro_source_probe,
            ),
            ts_source_effective_content_adapter_modules: effective_content_adapter_modules(
                ts_source_probe,
            ),
            tsx_source_effective_content_adapter_modules: effective_content_adapter_modules(
                tsx_source_probe,
            ),
            astro_source_route_scoped_pipeline_rule_scopes: route_scoped_pipeline_rule_scopes(
                astro_source_probe,
            ),
            ts_source_route_scoped_pipeline_rule_scopes: route_scoped_pipeline_rule_scopes(
                ts_source_probe,
            ),
            tsx_source_route_scoped_pipeline_rule_scopes: route_scoped_pipeline_rule_scopes(
                tsx_source_probe,
            ),
            astro_source_effective_content_data_pipeline_rules:
                effective_content_data_pipeline_rules(
                    astro_source_probe,
                    &route_page_paths,
                    &endpoint_paths,
                ),
            ts_source_effective_content_data_pipeline_rules: effective_content_data_pipeline_rules(
                ts_source_probe,
                &route_page_paths,
                &endpoint_paths,
            ),
            tsx_source_effective_content_data_pipeline_rules: effective_content_data_pipeline_rules(
                tsx_source_probe,
                &route_page_paths,
                &endpoint_paths,
            ),
            astro_source_effective_content_source_pipeline_rules:
                effective_content_source_pipeline_rules(
                    astro_source_probe,
                    &route_page_paths,
                    &endpoint_paths,
                ),
            ts_source_effective_content_source_pipeline_rules:
                effective_content_source_pipeline_rules(
                    ts_source_probe,
                    &route_page_paths,
                    &endpoint_paths,
                ),
            tsx_source_effective_content_source_pipeline_rules:
                effective_content_source_pipeline_rules(
                    tsx_source_probe,
                    &route_page_paths,
                    &endpoint_paths,
                ),
            astro_source_effective_inline_public_content_rules:
                effective_inline_public_content_rules(astro_source_probe),
            ts_source_effective_inline_public_content_rules: effective_inline_public_content_rules(
                ts_source_probe,
            ),
            tsx_source_effective_inline_public_content_rules: effective_inline_public_content_rules(
                tsx_source_probe,
            ),
            mdx_content_effective_mdx_component_map_rules: effective_mdx_component_map_rules(
                mdx_content_probe,
                &mdx_content_paths,
                &mdx_component_map_policy_paths,
            ),
            astro_source_effective_metadata_helper_rules: effective_metadata_helper_rules(
                astro_source_probe,
                &route_page_paths,
                &endpoint_paths,
                &metadata_helper_policy_paths,
                &content_adapter_policy_paths,
            ),
            ts_source_effective_metadata_helper_rules: effective_metadata_helper_rules(
                ts_source_probe,
                &route_page_paths,
                &endpoint_paths,
                &metadata_helper_policy_paths,
                &content_adapter_policy_paths,
            ),
            tsx_source_effective_metadata_helper_rules: effective_metadata_helper_rules(
                tsx_source_probe,
                &route_page_paths,
                &endpoint_paths,
                &metadata_helper_policy_paths,
                &content_adapter_policy_paths,
            ),
            astro_source_effective_json_ld_helper_rules: effective_json_ld_helper_rules(
                astro_source_probe,
                &route_page_paths,
                &endpoint_paths,
                &json_ld_helper_policy_paths,
            ),
            ts_source_effective_json_ld_helper_rules: effective_json_ld_helper_rules(
                ts_source_probe,
                &route_page_paths,
                &endpoint_paths,
                &json_ld_helper_policy_paths,
            ),
            tsx_source_effective_json_ld_helper_rules: effective_json_ld_helper_rules(
                tsx_source_probe,
                &route_page_paths,
                &endpoint_paths,
                &json_ld_helper_policy_paths,
            ),
            astro_source_probe_ignored: probe_ignored(
                &typed,
                eslint_config_parser::types::EslintProbeKind::AstroSource,
            ),
            ts_source_probe_ignored: probe_ignored(
                &typed,
                eslint_config_parser::types::EslintProbeKind::TsSource,
            ),
            tsx_source_probe_ignored: probe_ignored(
                &typed,
                eslint_config_parser::types::EslintProbeKind::TsxSource,
            ),
            mdx_content_probe_ignored: probe_ignored(
                &typed,
                eslint_config_parser::types::EslintProbeKind::MdxContent,
            ),
        },
    }
}
