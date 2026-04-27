use g3_workspace_crawl::G3RsWorkspaceCrawl as G3WorkspaceCrawl;
use g3ts_astro_content_types::{
    G3TsAstroContentAdapterRootInput, G3TsAstroContentAdapterSourceInput,
    G3TsAstroContentAppRootInput, G3TsAstroContentConfigChecksInput,
    G3TsAstroContentEslintPluginContractInput, G3TsAstroContentFileTreeChecksInput,
    G3TsAstroContentIntegrationContractInput, G3TsAstroContentPolicyEslintContractInput,
    G3TsAstroContentPolicySurfaceState,
};

#[must_use]
pub fn ingest_for_config_checks(crawl: &G3WorkspaceCrawl) -> G3TsAstroContentConfigChecksInput {
    let app_roots = crate::roots::astro_app_roots(crawl);
    let policies = app_roots
        .iter()
        .map(|app_root_rel_path| {
            (
                app_root_rel_path.clone(),
                crate::policy::ingest_content_policy_surface(crawl, app_root_rel_path),
            )
        })
        .collect::<Vec<_>>();
    let integration_contracts = policies
        .iter()
        .map(
            |(app_root_rel_path, astro_policy)| G3TsAstroContentIntegrationContractInput {
                app_root_rel_path: app_root_rel_path.clone(),
                route_page_paths: crate::policy::route_page_paths(crawl, app_root_rel_path),
                endpoint_paths: crate::policy::endpoint_paths(crawl, app_root_rel_path),
                content_adapter_sources: crate::sources::content_adapter_sources(
                    crawl,
                    app_root_rel_path,
                    astro_policy,
                ),
                package: crate::package::ingest_package_surface(crawl, app_root_rel_path),
                astro_policy: astro_policy.clone(),
            },
        )
        .collect::<Vec<_>>();
    let adapter_root_contracts = integration_contracts
        .iter()
        .flat_map(adapter_root_contracts)
        .collect();
    let adapter_source_contracts = integration_contracts
        .iter()
        .flat_map(adapter_source_contracts)
        .collect();

    G3TsAstroContentConfigChecksInput {
        integration_contracts,
        eslint_contracts: policies
            .iter()
            .map(
                |(app_root_rel_path, astro_policy)| G3TsAstroContentEslintPluginContractInput {
                    app_root_rel_path: app_root_rel_path.clone(),
                    config: crate::eslint::ingest_content_eslint_surface(
                        crawl,
                        app_root_rel_path,
                        astro_policy,
                    ),
                },
            )
            .collect(),
        policy_eslint_contracts: policies
            .iter()
            .map(
                |(app_root_rel_path, astro_policy)| G3TsAstroContentPolicyEslintContractInput {
                    app_root_rel_path: app_root_rel_path.clone(),
                    route_page_paths: crate::policy::route_page_paths(crawl, app_root_rel_path),
                    endpoint_paths: crate::policy::endpoint_paths(crawl, app_root_rel_path),
                    astro_policy: astro_policy.clone(),
                    eslint_config: crate::eslint::ingest_content_eslint_surface(
                        crawl,
                        app_root_rel_path,
                        astro_policy,
                    ),
                },
            )
            .collect(),
        adapter_root_contracts,
        adapter_source_contracts,
    }
}

fn adapter_root_contracts(
    contract: &G3TsAstroContentIntegrationContractInput,
) -> Vec<G3TsAstroContentAdapterRootInput> {
    let G3TsAstroContentPolicySurfaceState::Parsed { snapshot: policy } = &contract.astro_policy
    else {
        return Vec::new();
    };

    policy
        .content_adapters
        .iter()
        .map(|configured_adapter| {
            let source_exists = adapter_has_source(
                configured_adapter,
                &contract.content_adapter_sources.content_adapter,
            );
            G3TsAstroContentAdapterRootInput {
                policy_rel_path: policy.rel_path.clone(),
                configured_adapter: configured_adapter.clone(),
                source_exists,
            }
        })
        .collect()
}

fn adapter_source_contracts(
    contract: &G3TsAstroContentIntegrationContractInput,
) -> Vec<G3TsAstroContentAdapterSourceInput> {
    let G3TsAstroContentPolicySurfaceState::Parsed { snapshot: policy } = &contract.astro_policy
    else {
        return Vec::new();
    };

    contract
        .content_adapter_sources
        .content_adapter
        .iter()
        .map(|source_rel_path| G3TsAstroContentAdapterSourceInput {
            policy_rel_path: policy.rel_path.clone(),
            source_rel_path: source_rel_path.clone(),
            imports_astro_content: contract
                .content_adapter_sources
                .content_adapter_astro_content
                .iter()
                .any(|path| path == source_rel_path),
        })
        .collect()
}

fn adapter_has_source(adapter: &str, source_paths: &[String]) -> bool {
    let adapter = adapter.trim_end_matches('/');
    let adapter_prefix = format!("{adapter}/");
    source_paths
        .iter()
        .any(|path| path == adapter || path.starts_with(&adapter_prefix))
}

#[must_use]
pub fn ingest_for_file_tree_checks(
    crawl: &G3WorkspaceCrawl,
) -> G3TsAstroContentFileTreeChecksInput {
    let app_root_rel_paths = crate::roots::astro_app_roots(crawl);
    let app_roots: Vec<G3TsAstroContentAppRootInput> = app_root_rel_paths
        .iter()
        .map(|app_root_rel_path| G3TsAstroContentAppRootInput {
            app_root_rel_path: app_root_rel_path.clone(),
            content_config_rel_path: crate::policy::select_content_config(crawl, app_root_rel_path)
                .map(|entry| entry.path.rel_path.clone()),
            live_config_rel_path: crate::roots::select_live_config(crawl, app_root_rel_path)
                .map(|entry| entry.path.rel_path.clone()),
            velite_config_rel_path: crate::policy::select_velite_config(crawl, app_root_rel_path)
                .map(|entry| entry.path.rel_path.clone()),
        })
        .collect();
    let velite_output_paths = app_roots
        .iter()
        .filter(|root| {
            crate::policy::classify_content_mode(crawl, &root.app_root_rel_path)
                != g3ts_astro_content_types::G3TsAstroContentMode::None
        })
        .flat_map(|root| {
            crate::policy::velite_output_paths(crawl, &root.app_root_rel_path, &app_root_rel_paths)
                .into_iter()
                .map(
                    |rel_path| g3ts_astro_content_types::G3TsAstroContentVeliteOutputInput {
                        app_root_rel_path: root.app_root_rel_path.clone(),
                        rel_path,
                    },
                )
        })
        .collect();
    G3TsAstroContentFileTreeChecksInput {
        build_collection_roots: app_roots
            .iter()
            .filter(|root| {
                crate::policy::classify_content_mode(crawl, &root.app_root_rel_path)
                    == g3ts_astro_content_types::G3TsAstroContentMode::BuildCollections
            })
            .cloned()
            .collect(),
        live_collection_roots: app_roots
            .iter()
            .filter(|root| {
                crate::policy::classify_content_mode(crawl, &root.app_root_rel_path)
                    == g3ts_astro_content_types::G3TsAstroContentMode::LiveCollections
            })
            .cloned()
            .collect(),
        route_markdown_pages: app_root_rel_paths
            .iter()
            .flat_map(|app_root_rel_path| {
                crate::policy::route_markdown_pages(crawl, app_root_rel_path)
            })
            .map(|rel_path| g3ts_astro_content_types::G3TsAstroRouteMarkdownPageInput { rel_path })
            .collect(),
        velite_output_paths,
        app_roots,
    }
}
