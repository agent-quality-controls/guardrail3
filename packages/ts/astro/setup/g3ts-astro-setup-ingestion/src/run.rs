use g3_workspace_crawl::G3RsWorkspaceCrawl as G3WorkspaceCrawl;
use g3ts_astro_types::{
    G3TsAstroEslintPluginContractInput, G3TsAstroSetupConfigChecksInput,
    G3TsAstroSetupFileTreeChecksInput, G3TsAstroSetupIntegrationContractInput,
};

#[must_use]
pub fn ingest_for_config_checks(crawl: &G3WorkspaceCrawl) -> G3TsAstroSetupConfigChecksInput {
    let app_roots = g3ts_astro_check_support::surfaces::astro_app_roots(crawl);
    G3TsAstroSetupConfigChecksInput {
        integration_contracts: app_roots
            .iter()
            .map(|app_root_rel_path| {
                let package =
                    g3ts_astro_check_support::surfaces::ingest_package_surface(crawl, app_root_rel_path);
                G3TsAstroSetupIntegrationContractInput {
                    app_root_rel_path: app_root_rel_path.clone(),
                    content_mode: g3ts_astro_check_support::surfaces::classify_content_mode(
                        crawl,
                        app_root_rel_path,
                    ),
                    syncpack_config:
                        g3ts_astro_check_support::surfaces::ingest_syncpack_config_surface(
                            crawl,
                            app_root_rel_path,
                            &package,
                        ),
                    astro_config: g3ts_astro_check_support::surfaces::ingest_astro_config_surface(
                        crawl,
                        app_root_rel_path,
                    ),
                    package,
                    required_syncpack_pins: g3ts_astro_check_support::surfaces::required_syncpack_pins(),
                    forbidden_syncpack_deps: g3ts_astro_check_support::surfaces::forbidden_syncpack_deps(),
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
pub fn ingest_for_file_tree_checks(crawl: &G3WorkspaceCrawl) -> G3TsAstroSetupFileTreeChecksInput {
    let app_roots = g3ts_astro_check_support::surfaces::app_root_inputs(crawl);
    G3TsAstroSetupFileTreeChecksInput {
        live_collection_roots: g3ts_astro_check_support::surfaces::live_collection_roots(
            crawl,
            &app_roots,
        ),
        app_roots,
    }
}
