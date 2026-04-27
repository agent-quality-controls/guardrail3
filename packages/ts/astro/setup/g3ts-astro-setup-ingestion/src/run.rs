use g3_workspace_crawl::G3RsWorkspaceCrawl as G3WorkspaceCrawl;
use g3ts_astro_setup_types::{
    G3TsAstroSetupAppRootInput, G3TsAstroSetupConfigChecksInput,
    G3TsAstroSetupEslintPluginContractInput, G3TsAstroSetupFileTreeChecksInput,
    G3TsAstroSetupIntegrationContractInput,
};

#[must_use]
pub fn ingest_for_config_checks(crawl: &G3WorkspaceCrawl) -> G3TsAstroSetupConfigChecksInput {
    let app_roots = crate::roots::astro_app_roots(crawl);
    G3TsAstroSetupConfigChecksInput {
        integration_contracts: app_roots
            .iter()
            .map(|app_root_rel_path| {
                let package = crate::package::ingest_package_surface(crawl, app_root_rel_path);
                G3TsAstroSetupIntegrationContractInput {
                    app_root_rel_path: app_root_rel_path.clone(),
                    syncpack_config: crate::syncpack::ingest_syncpack_config_surface(
                        crawl,
                        app_root_rel_path,
                        &package,
                    ),
                    astro_config: crate::astro_config::ingest_astro_config_surface(
                        crawl,
                        app_root_rel_path,
                    ),
                    package,
                    required_syncpack_pins: crate::syncpack::required_syncpack_pins(),
                    forbidden_syncpack_deps: crate::syncpack::forbidden_syncpack_deps(),
                }
            })
            .collect(),
        eslint_contracts: app_roots
            .iter()
            .map(
                |app_root_rel_path| G3TsAstroSetupEslintPluginContractInput {
                    app_root_rel_path: app_root_rel_path.clone(),
                    config: crate::eslint::ingest_setup_eslint_surface(crawl, app_root_rel_path),
                },
            )
            .collect(),
    }
}

#[must_use]
pub fn ingest_for_file_tree_checks(crawl: &G3WorkspaceCrawl) -> G3TsAstroSetupFileTreeChecksInput {
    let app_roots: Vec<G3TsAstroSetupAppRootInput> = crate::roots::astro_app_roots(crawl)
        .iter()
        .map(|app_root_rel_path| G3TsAstroSetupAppRootInput {
            app_root_rel_path: app_root_rel_path.clone(),
            astro_config_rel_path: crate::roots::select_astro_config(crawl, app_root_rel_path)
                .map(|entry| entry.path.rel_path.clone()),
        })
        .collect();
    G3TsAstroSetupFileTreeChecksInput { app_roots }
}
