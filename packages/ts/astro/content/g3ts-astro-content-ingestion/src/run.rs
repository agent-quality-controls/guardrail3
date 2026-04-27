use g3_workspace_crawl::G3RsWorkspaceCrawl as G3WorkspaceCrawl;
use g3ts_astro_types::{
    G3TsAstroContentConfigChecksInput, G3TsAstroContentFileTreeChecksInput,
    G3TsAstroContentIntegrationContractInput, G3TsAstroEslintPluginContractInput,
};

#[must_use]
pub fn ingest_for_config_checks(crawl: &G3WorkspaceCrawl) -> G3TsAstroContentConfigChecksInput {
    let app_roots = g3ts_astro_check_support::surfaces::astro_app_roots(crawl);
    G3TsAstroContentConfigChecksInput {
        integration_contracts: app_roots
            .iter()
            .map(|app_root_rel_path| {
                let astro_policy =
                    g3ts_astro_check_support::surfaces::ingest_astro_policy_surface(crawl, app_root_rel_path);
                G3TsAstroContentIntegrationContractInput {
                    app_root_rel_path: app_root_rel_path.clone(),
                    route_page_paths: g3ts_astro_check_support::select::route_page_paths(
                        crawl,
                        app_root_rel_path,
                    ),
                    endpoint_paths: g3ts_astro_check_support::select::endpoint_paths(
                        crawl,
                        app_root_rel_path,
                    ),
                    content_adapter_sources: g3ts_astro_check_support::surfaces::content_adapter_sources(
                            crawl,
                            app_root_rel_path,
                            &astro_policy,
                        ),
                    package:
                        g3ts_astro_check_support::surfaces::ingest_package_surface(crawl, app_root_rel_path),
                    astro_policy,
                }
            })
            .collect(),
        eslint_contracts: app_roots
            .iter()
            .map(|app_root_rel_path| {
                let astro_policy =
                    g3ts_astro_check_support::surfaces::ingest_astro_policy_surface(crawl, app_root_rel_path);
                G3TsAstroEslintPluginContractInput {
                    app_root_rel_path: app_root_rel_path.clone(),
                    config: g3ts_astro_check_support::surfaces::ingest_eslint_surface(
                        crawl,
                        app_root_rel_path,
                        &astro_policy,
                    ),
                }
            })
            .collect(),
    }
}

#[must_use]
pub fn ingest_for_file_tree_checks(crawl: &G3WorkspaceCrawl) -> G3TsAstroContentFileTreeChecksInput {
    let app_root_rel_paths = g3ts_astro_check_support::surfaces::astro_app_roots(crawl);
    let app_roots = g3ts_astro_check_support::surfaces::app_root_inputs(crawl);
    G3TsAstroContentFileTreeChecksInput {
        build_collection_roots: g3ts_astro_check_support::surfaces::build_collection_roots(
            crawl,
            &app_roots,
        ),
        live_collection_roots: g3ts_astro_check_support::surfaces::live_collection_roots(
            crawl,
            &app_roots,
        ),
        route_markdown_pages: g3ts_astro_check_support::surfaces::route_markdown_page_inputs(
            crawl,
            &app_root_rel_paths,
        ),
        app_roots,
    }
}
