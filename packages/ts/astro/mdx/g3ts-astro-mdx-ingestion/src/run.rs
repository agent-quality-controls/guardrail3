use g3_workspace_crawl::G3RsWorkspaceCrawl as G3WorkspaceCrawl;
use g3ts_astro_types::{
    G3TsAstroEslintPluginContractInput, G3TsAstroMdxConfigChecksInput,
    G3TsAstroMdxIntegrationContractInput,
};

#[must_use]
pub fn ingest_for_config_checks(crawl: &G3WorkspaceCrawl) -> G3TsAstroMdxConfigChecksInput {
    let app_roots = g3ts_astro_check_support::surfaces::astro_app_roots(crawl);
    G3TsAstroMdxConfigChecksInput {
        integration_contracts: app_roots
            .iter()
            .map(|app_root_rel_path| {
                let astro_policy =
                    g3ts_astro_check_support::surfaces::ingest_astro_policy_surface(crawl, app_root_rel_path);
                G3TsAstroMdxIntegrationContractInput {
                    app_root_rel_path: app_root_rel_path.clone(),
                    mdx_sources: g3ts_astro_check_support::surfaces::mdx_component_map_sources(
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
