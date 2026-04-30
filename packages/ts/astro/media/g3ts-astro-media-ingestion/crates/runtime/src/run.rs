use g3_workspace_crawl::G3RsWorkspaceCrawl as G3WorkspaceCrawl;
use g3ts_astro_media_types::{
    G3TsAstroMediaConfigChecksInput, G3TsAstroMediaEslintPluginContractInput,
    G3TsAstroMediaIntegrationContractInput,
};

#[must_use]
pub fn ingest_for_config_checks(crawl: &G3WorkspaceCrawl) -> G3TsAstroMediaConfigChecksInput {
    let app_roots = crate::roots::astro_app_roots(crawl);
    let policies = app_roots
        .iter()
        .map(|app_root_rel_path| {
            (
                app_root_rel_path.clone(),
                crate::policy::ingest_media_policy_surface(crawl, app_root_rel_path),
            )
        })
        .collect::<Vec<_>>();

    G3TsAstroMediaConfigChecksInput {
        integration_contracts: policies
            .iter()
            .map(
                |(app_root_rel_path, astro_policy)| G3TsAstroMediaIntegrationContractInput {
                    app_root_rel_path: app_root_rel_path.clone(),
                    package: crate::package::ingest_package_surface(crawl, app_root_rel_path),
                    astro_config: crate::astro_config::ingest_astro_config_surface(
                        crawl,
                        app_root_rel_path,
                    ),
                    astro_policy: astro_policy.clone(),
                },
            )
            .collect(),
        eslint_contracts: policies
            .iter()
            .map(
                |(app_root_rel_path, astro_policy)| G3TsAstroMediaEslintPluginContractInput {
                    app_root_rel_path: app_root_rel_path.clone(),
                    config: crate::eslint::ingest_media_eslint_surface(
                        crawl,
                        app_root_rel_path,
                        astro_policy,
                    ),
                    astro_policy: astro_policy.clone(),
                },
            )
            .collect(),
    }
}
