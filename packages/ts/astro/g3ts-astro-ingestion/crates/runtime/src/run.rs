use eslint_config_parser::{parse_document, parse_error_reason as eslint_parse_error_reason};
use g3_workspace_crawl::G3RsWorkspaceCrawl as G3WorkspaceCrawl;
use g3ts_astro_types::{
    G3TsAstroAppRootInput, G3TsAstroConfigChecksInput, G3TsAstroContentMode,
    G3TsAstroEslintPluginContractInput, G3TsAstroEslintSurfaceSnapshot,
    G3TsAstroEslintSurfaceState, G3TsAstroFileTreeChecksInput,
    G3TsAstroIntegrationContractInput, G3TsAstroPackageSurfaceSnapshot,
    G3TsAstroPackageSurfaceState, G3TsAstroRouteMarkdownPageInput,
};
use package_json_parser::{from_path_document, parse_error_reason as package_parse_error_reason};

const APP_ROOT_REL_PATH: &str = ".";
const ESLINT_CONFIG_PATTERN: &str = "eslint.config.*";
const PACKAGE_JSON_REL_PATH: &str = "package.json";

pub fn ingest_for_config_checks(crawl: &G3WorkspaceCrawl) -> G3TsAstroConfigChecksInput {
    if !astro_policy_applies(crawl) {
        return G3TsAstroConfigChecksInput {
            integration_contracts: Vec::new(),
            eslint_contracts: Vec::new(),
        };
    }

    let package = ingest_package_surface(crawl);
    let content_mode = classify_content_mode(crawl);

    G3TsAstroConfigChecksInput {
        integration_contracts: vec![G3TsAstroIntegrationContractInput {
            app_root_rel_path: APP_ROOT_REL_PATH.to_owned(),
            content_mode,
            package,
            requires_render_validator: content_mode != G3TsAstroContentMode::None,
            requires_source_pipeline_linting: content_mode != G3TsAstroContentMode::None,
        }],
        eslint_contracts: vec![G3TsAstroEslintPluginContractInput {
            app_root_rel_path: APP_ROOT_REL_PATH.to_owned(),
            config: ingest_eslint_surface(crawl),
            requires_source_pipeline_linting: content_mode != G3TsAstroContentMode::None,
        }],
    }
}

pub fn ingest_for_file_tree_checks(crawl: &G3WorkspaceCrawl) -> G3TsAstroFileTreeChecksInput {
    if !astro_policy_applies(crawl) {
        return G3TsAstroFileTreeChecksInput {
            app_roots: Vec::new(),
            build_collection_roots: Vec::new(),
            live_collection_roots: Vec::new(),
            route_markdown_pages: Vec::new(),
            cross_root_side_loaders: Vec::new(),
        };
    }

    let root = G3TsAstroAppRootInput {
        app_root_rel_path: APP_ROOT_REL_PATH.to_owned(),
        astro_config_rel_path: crate::select::select_root_astro_config(crawl)
            .map(|entry| entry.path.rel_path.clone()),
        content_config_rel_path: crate::select::select_content_config(crawl)
            .map(|entry| entry.path.rel_path.clone()),
        live_config_rel_path: crate::select::select_live_config(crawl)
            .map(|entry| entry.path.rel_path.clone()),
    };

    let content_mode = classify_content_mode(crawl);

    G3TsAstroFileTreeChecksInput {
        app_roots: vec![root.clone()],
        build_collection_roots: if content_mode == G3TsAstroContentMode::BuildCollections {
            vec![root.clone()]
        } else {
            Vec::new()
        },
        live_collection_roots: if content_mode == G3TsAstroContentMode::LiveCollections {
            vec![root]
        } else {
            Vec::new()
        },
        route_markdown_pages: crate::select::route_markdown_pages(crawl)
            .into_iter()
            .map(|rel_path| G3TsAstroRouteMarkdownPageInput { rel_path })
            .collect(),
        cross_root_side_loaders: Vec::new(),
    }
}

fn astro_policy_applies(crawl: &G3WorkspaceCrawl) -> bool {
    crate::select::select_root_astro_config(crawl).is_some()
        || package_surface_has_astro_dependency(&ingest_package_surface(crawl))
}

fn classify_content_mode(crawl: &G3WorkspaceCrawl) -> G3TsAstroContentMode {
    if crate::select::select_live_config(crawl).is_some() {
        G3TsAstroContentMode::LiveCollections
    } else if crate::select::select_content_config(crawl).is_some()
        || crate::select::has_content_files(crawl)
    {
        G3TsAstroContentMode::BuildCollections
    } else {
        G3TsAstroContentMode::None
    }
}

fn ingest_package_surface(crawl: &G3WorkspaceCrawl) -> G3TsAstroPackageSurfaceState {
    let Some(entry) = crate::select::select_root_package_json(crawl) else {
        return G3TsAstroPackageSurfaceState::Missing {
            rel_path: PACKAGE_JSON_REL_PATH.to_owned(),
        };
    };

    if !entry.readable {
        return G3TsAstroPackageSurfaceState::Unreadable {
            rel_path: entry.path.rel_path.clone(),
            reason: "workspace crawl marked the package manifest unreadable".to_owned(),
        };
    }

    let document = match from_path_document(&entry.path.abs_path) {
        Ok(document) => document,
        Err(error) => {
            return G3TsAstroPackageSurfaceState::ParseError {
                rel_path: entry.path.rel_path.clone(),
                reason: error.to_string(),
            };
        }
    };

    if let Some(reason) = package_parse_error_reason(&document) {
        return G3TsAstroPackageSurfaceState::ParseError {
            rel_path: entry.path.rel_path.clone(),
            reason: reason.to_owned(),
        };
    }

    let typed = package_json_parser::typed(&document)
        .expect("parsed package.json document should stay typed");
    G3TsAstroPackageSurfaceState::Parsed {
        snapshot: G3TsAstroPackageSurfaceSnapshot {
            rel_path: entry.path.rel_path.clone(),
            dependencies: typed.dependencies.clone(),
            dev_dependencies: typed.dev_dependencies.clone(),
            script_names: typed.scripts.keys().cloned().collect(),
            script_bodies: typed
                .scripts
                .iter()
                .map(|(name, body)| (name.clone(), body.clone()))
                .collect(),
        },
    }
}

fn ingest_eslint_surface(crawl: &G3WorkspaceCrawl) -> G3TsAstroEslintSurfaceState {
    let Some(entry) = crate::select::select_active_root_eslint_config(crawl) else {
        return G3TsAstroEslintSurfaceState::Missing {
            rel_path: ESLINT_CONFIG_PATTERN.to_owned(),
        };
    };

    if !entry.readable {
        return G3TsAstroEslintSurfaceState::Unreadable {
            rel_path: entry.path.rel_path.clone(),
            reason: "workspace crawl marked the selected eslint config unreadable".to_owned(),
        };
    }

    let probes = crate::select::probe_targets(crawl, &entry.path.rel_path);
    let document = match parse_document(&crawl.root_abs_path, &entry.path.rel_path, &probes) {
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
    let ts_source_plugins = typed
        .probes
        .iter()
        .find(|probe| probe.probe == eslint_config_parser::types::EslintProbeKind::TsSource)
        .map(|probe| probe.plugins.clone())
        .unwrap_or_default();
    let tsx_source_plugins = typed
        .probes
        .iter()
        .find(|probe| probe.probe == eslint_config_parser::types::EslintProbeKind::TsxSource)
        .map(|probe| probe.plugins.clone())
        .unwrap_or_default();

    G3TsAstroEslintSurfaceState::Parsed {
        snapshot: G3TsAstroEslintSurfaceSnapshot {
            rel_path: entry.path.rel_path.clone(),
            ts_source_probe_present: typed
                .probes
                .iter()
                .any(|probe| probe.probe == eslint_config_parser::types::EslintProbeKind::TsSource),
            tsx_source_probe_present: typed
                .probes
                .iter()
                .any(|probe| probe.probe == eslint_config_parser::types::EslintProbeKind::TsxSource),
            ts_source_plugins,
            tsx_source_plugins,
            ts_source_error_rules: typed
                .probes
                .iter()
                .find(|probe| probe.probe == eslint_config_parser::types::EslintProbeKind::TsSource)
                .map(|probe| {
                    probe
                        .rules
                        .iter()
                        .filter_map(|(rule_name, setting)| {
                            (setting.severity
                                == eslint_config_parser::types::EslintRuleSeverity::Error)
                                .then_some(rule_name.clone())
                        })
                        .collect()
                })
                .unwrap_or_default(),
            tsx_source_error_rules: typed
                .probes
                .iter()
                .find(|probe| probe.probe == eslint_config_parser::types::EslintProbeKind::TsxSource)
                .map(|probe| {
                    probe
                        .rules
                        .iter()
                        .filter_map(|(rule_name, setting)| {
                            (setting.severity
                                == eslint_config_parser::types::EslintRuleSeverity::Error)
                                .then_some(rule_name.clone())
                        })
                        .collect()
                })
                .unwrap_or_default(),
        },
    }
}

fn package_surface_has_astro_dependency(package: &G3TsAstroPackageSurfaceState) -> bool {
    match package {
        G3TsAstroPackageSurfaceState::Parsed { snapshot } => snapshot
            .dependencies
            .iter()
            .chain(snapshot.dev_dependencies.iter())
            .any(|dependency| dependency == "astro"),
        G3TsAstroPackageSurfaceState::Missing { .. }
        | G3TsAstroPackageSurfaceState::Unreadable { .. }
        | G3TsAstroPackageSurfaceState::ParseError { .. } => false,
    }
}

#[cfg(test)]
#[path = "run_tests/mod.rs"]
mod run_tests;
