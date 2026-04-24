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
use std::collections::BTreeSet;

const ESLINT_CONFIG_PATTERN: &str = "eslint.config.*";
const PACKAGE_JSON_REL_PATH: &str = "package.json";
const ROUTE_SCOPED_PIPELINE_RULES: [&str; 6] = [
    "astro-pipeline/no-authored-content-fs-read",
    "astro-pipeline/no-authored-content-glob",
    "astro-pipeline/no-content-data-modules-in-routes",
    "astro-pipeline/no-direct-astro-content-in-routes",
    "astro-pipeline/no-side-loader-imports",
    "astro-pipeline/no-velite-imports",
];
const CONTENT_DATA_PIPELINE_RULES: [&str; 1] = ["astro-pipeline/no-content-data-modules-in-routes"];

pub fn ingest_for_config_checks(crawl: &G3WorkspaceCrawl) -> G3TsAstroConfigChecksInput {
    let app_roots = astro_app_roots(crawl);

    if app_roots.is_empty() {
            return G3TsAstroConfigChecksInput {
            integration_contracts: Vec::new(),
            eslint_contracts: Vec::new(),
        };
    }

    G3TsAstroConfigChecksInput {
        integration_contracts: app_roots
            .iter()
            .map(|app_root_rel_path| G3TsAstroIntegrationContractInput {
                app_root_rel_path: app_root_rel_path.clone(),
                content_mode: classify_content_mode(crawl, app_root_rel_path),
                package: ingest_package_surface(crawl, app_root_rel_path),
                requires_source_pipeline_linting: true,
            })
            .collect(),
        eslint_contracts: app_roots
            .iter()
            .map(|app_root_rel_path| G3TsAstroEslintPluginContractInput {
                app_root_rel_path: app_root_rel_path.clone(),
                config: ingest_eslint_surface(crawl, app_root_rel_path),
                requires_source_pipeline_linting: true,
            })
            .collect(),
    }
}

pub fn ingest_for_file_tree_checks(crawl: &G3WorkspaceCrawl) -> G3TsAstroFileTreeChecksInput {
    let app_roots = astro_app_roots(crawl);

    if app_roots.is_empty() {
        return G3TsAstroFileTreeChecksInput {
            app_roots: Vec::new(),
            build_collection_roots: Vec::new(),
            live_collection_roots: Vec::new(),
            route_markdown_pages: Vec::new(),
        };
    }

    let roots: Vec<_> = app_roots
        .iter()
        .map(|app_root_rel_path| G3TsAstroAppRootInput {
            app_root_rel_path: app_root_rel_path.clone(),
            astro_config_rel_path: crate::select::select_astro_config(crawl, app_root_rel_path)
                .map(|entry| entry.path.rel_path.clone()),
            content_config_rel_path: crate::select::select_content_config(crawl, app_root_rel_path)
                .map(|entry| entry.path.rel_path.clone()),
            live_config_rel_path: crate::select::select_live_config(crawl, app_root_rel_path)
                .map(|entry| entry.path.rel_path.clone()),
            velite_config_rel_path: crate::select::select_velite_config(crawl, app_root_rel_path)
                .map(|entry| entry.path.rel_path.clone()),
            velite_output_rel_paths: crate::select::velite_output_paths(crawl, app_root_rel_path),
        })
        .collect();

    G3TsAstroFileTreeChecksInput {
        app_roots: roots.clone(),
        build_collection_roots: roots
            .iter()
            .filter(|root| {
                classify_content_mode(crawl, &root.app_root_rel_path)
                    == G3TsAstroContentMode::BuildCollections
            })
            .cloned()
            .collect(),
        live_collection_roots: roots
            .iter()
            .filter(|root| {
                classify_content_mode(crawl, &root.app_root_rel_path)
                    == G3TsAstroContentMode::LiveCollections
            })
            .cloned()
            .collect(),
        route_markdown_pages: app_roots
            .iter()
            .flat_map(|app_root_rel_path| crate::select::route_markdown_pages(crawl, app_root_rel_path))
            .into_iter()
            .map(|rel_path| G3TsAstroRouteMarkdownPageInput { rel_path })
            .collect(),
    }
}

fn astro_app_roots(crawl: &G3WorkspaceCrawl) -> Vec<String> {
    let mut roots: BTreeSet<String> = crate::select::select_astro_app_roots(crawl).into_iter().collect();

    for entry in crawl.entries.iter().filter(|entry| {
        entry.kind == g3_workspace_crawl::G3RsWorkspaceEntryKind::File
            && entry.ignore_state == g3_workspace_crawl::G3RsWorkspaceIgnoreState::Included
            && (entry.path.rel_path.ends_with("/package.json")
                || entry.path.rel_path == PACKAGE_JSON_REL_PATH)
    }) {
        let app_root_rel_path = if entry.path.rel_path == PACKAGE_JSON_REL_PATH {
            ".".to_owned()
        } else {
            std::path::Path::new(&entry.path.rel_path)
                .parent()
                .and_then(|parent| parent.to_str())
                .unwrap_or(".")
                .to_owned()
        };

        if package_surface_has_astro_dependency(&ingest_package_surface(crawl, &app_root_rel_path)) {
            let _ = roots.insert(app_root_rel_path);
        }
    }

    roots.into_iter().collect()
}

fn classify_content_mode(crawl: &G3WorkspaceCrawl, app_root_rel_path: &str) -> G3TsAstroContentMode {
    if crate::select::select_live_config(crawl, app_root_rel_path).is_some() {
        G3TsAstroContentMode::LiveCollections
    } else if crate::select::select_content_config(crawl, app_root_rel_path).is_some()
        || crate::select::has_content_files(crawl, app_root_rel_path)
    {
        G3TsAstroContentMode::BuildCollections
    } else {
        G3TsAstroContentMode::None
    }
}

fn ingest_package_surface(
    crawl: &G3WorkspaceCrawl,
    app_root_rel_path: &str,
) -> G3TsAstroPackageSurfaceState {
    let Some(entry) = crate::select::select_package_json(crawl, app_root_rel_path) else {
        return G3TsAstroPackageSurfaceState::Missing {
            rel_path: if app_root_rel_path == "." {
                PACKAGE_JSON_REL_PATH.to_owned()
            } else {
                format!("{app_root_rel_path}/{PACKAGE_JSON_REL_PATH}")
            },
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

fn ingest_eslint_surface(
    crawl: &G3WorkspaceCrawl,
    app_root_rel_path: &str,
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
    let astro_source_probe =
        active_probe(&typed, eslint_config_parser::types::EslintProbeKind::AstroSource);
    let ts_source_probe =
        active_probe(&typed, eslint_config_parser::types::EslintProbeKind::TsSource);
    let tsx_source_probe =
        active_probe(&typed, eslint_config_parser::types::EslintProbeKind::TsxSource);

    G3TsAstroEslintSurfaceState::Parsed {
        snapshot: G3TsAstroEslintSurfaceSnapshot {
            rel_path: entry.path.rel_path.clone(),
            astro_source_probe_present: astro_source_probe.is_some(),
            ts_source_probe_present: ts_source_probe.is_some(),
            tsx_source_probe_present: tsx_source_probe.is_some(),
            astro_source_plugins: astro_source_probe
                .map(|probe| probe.plugins.clone())
                .unwrap_or_default(),
            ts_source_plugins: ts_source_probe
                .map(|probe| probe.plugins.clone())
                .unwrap_or_default(),
            tsx_source_plugins: tsx_source_probe
                .map(|probe| probe.plugins.clone())
                .unwrap_or_default(),
            astro_source_error_rules: active_error_rules(astro_source_probe),
            ts_source_error_rules: active_error_rules(ts_source_probe),
            tsx_source_error_rules: active_error_rules(tsx_source_probe),
            astro_source_effective_route_scoped_pipeline_rules:
                effective_route_scoped_pipeline_rules(astro_source_probe),
            ts_source_effective_route_scoped_pipeline_rules:
                effective_route_scoped_pipeline_rules(ts_source_probe),
            tsx_source_effective_route_scoped_pipeline_rules:
                effective_route_scoped_pipeline_rules(tsx_source_probe),
            astro_source_effective_content_data_pipeline_rules:
                effective_content_data_pipeline_rules(astro_source_probe),
            ts_source_effective_content_data_pipeline_rules:
                effective_content_data_pipeline_rules(ts_source_probe),
            tsx_source_effective_content_data_pipeline_rules:
                effective_content_data_pipeline_rules(tsx_source_probe),
        },
    }
}

fn active_probe<'a>(
    typed: &'a eslint_config_parser::types::EslintConfigSnapshot,
    kind: eslint_config_parser::types::EslintProbeKind,
) -> Option<&'a eslint_config_parser::types::EslintEffectiveConfigProbe> {
    typed
        .probes
        .iter()
        .find(|probe| probe.probe == kind && !probe.ignored)
}

fn active_error_rules(
    probe: Option<&eslint_config_parser::types::EslintEffectiveConfigProbe>,
) -> Vec<String> {
    probe
        .map(|probe| {
            probe
                .rules
                .iter()
                .filter_map(|(rule_name, setting)| {
                    (setting.severity == eslint_config_parser::types::EslintRuleSeverity::Error)
                        .then_some(rule_name.clone())
                })
                .collect()
        })
        .unwrap_or_default()
}

fn effective_route_scoped_pipeline_rules(
    probe: Option<&eslint_config_parser::types::EslintEffectiveConfigProbe>,
) -> Vec<String> {
    let Some(probe) = probe else {
        return Vec::new();
    };

    ROUTE_SCOPED_PIPELINE_RULES
        .iter()
        .filter(|rule_name| {
            probe
                .rules
                .get(**rule_name)
                .is_some_and(rule_setting_has_route_or_endpoint_scope)
        })
        .map(|rule_name| (*rule_name).to_owned())
        .collect()
}

fn effective_content_data_pipeline_rules(
    probe: Option<&eslint_config_parser::types::EslintEffectiveConfigProbe>,
) -> Vec<String> {
    let Some(probe) = probe else {
        return Vec::new();
    };

    CONTENT_DATA_PIPELINE_RULES
        .iter()
        .filter(|rule_name| {
            probe
                .rules
                .get(**rule_name)
                .is_some_and(rule_setting_has_content_data_scope)
        })
        .map(|rule_name| (*rule_name).to_owned())
        .collect()
}

fn rule_setting_has_route_or_endpoint_scope(
    setting: &eslint_config_parser::types::EslintRuleSetting,
) -> bool {
    setting.options.iter().any(|option| {
        option.as_object().is_some_and(|object| {
            has_non_empty_string_array_option(object.get("routeGlobs"))
                || has_non_empty_string_array_option(object.get("endpointGlobs"))
        })
    })
}

fn rule_setting_has_content_data_scope(
    setting: &eslint_config_parser::types::EslintRuleSetting,
) -> bool {
    setting.options.iter().any(|option| {
        option.as_object().is_some_and(|object| {
            has_non_empty_string_array_option(object.get("contentDataModuleGlobs"))
        })
    })
}

fn has_non_empty_string_array_option(option: Option<&serde_json::Value>) -> bool {
    option
        .and_then(serde_json::Value::as_array)
        .is_some_and(|values| {
            !values.is_empty()
                && values
                    .iter()
                    .all(|value| value.as_str().is_some_and(|text| !text.trim().is_empty()))
        })
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
