use g3_workspace_crawl::G3RsWorkspaceCrawl as G3WorkspaceCrawl;
use g3ts_astro_i18n_types::{
    G3TsAstroI18nConfigChecksInput, G3TsAstroI18nEslintPluginContractInput,
    G3TsAstroI18nIntegrationContractInput,
};

/// Ingests workspace inputs required by Astro i18n config checks.
#[must_use]
pub fn ingest_for_config_checks(crawl: &G3WorkspaceCrawl) -> G3TsAstroI18nConfigChecksInput {
    let app_roots = crate::roots::astro_app_roots(crawl);
    let policies = app_roots
        .iter()
        .map(|app_root_rel_path| {
            (
                app_root_rel_path.clone(),
                crate::policy::ingest_i18n_policy_surface(crawl, app_root_rel_path),
            )
        })
        .collect::<Vec<_>>();

    G3TsAstroI18nConfigChecksInput {
        integration_contracts: policies
            .iter()
            .map(
                |(app_root_rel_path, astro_policy)| G3TsAstroI18nIntegrationContractInput {
                    app_root_rel_path: app_root_rel_path.clone(),
                    package: crate::package::ingest_package_surface(crawl, app_root_rel_path),
                    astro_policy: astro_policy.clone(),
                },
            )
            .collect(),
        eslint_contracts: policies
            .iter()
            .map(
                |(app_root_rel_path, astro_policy)| G3TsAstroI18nEslintPluginContractInput {
                    app_root_rel_path: app_root_rel_path.clone(),
                    config: crate::eslint::ingest_i18n_eslint_surface(
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
