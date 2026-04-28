use g3_workspace_crawl::G3RsWorkspaceCrawl as G3WorkspaceCrawl;
use g3ts_astro_seo_types::{
    G3TsAstroSeoConfigChecksInput, G3TsAstroSeoEslintPluginContractInput,
    G3TsAstroSeoIntegrationContractInput, G3TsAstroSeoMissingJsonLdHelperInput,
    G3TsAstroSeoMissingMetadataHelperInput, G3TsAstroSeoPolicySurfaceState,
};

#[must_use]
pub fn ingest_for_config_checks(crawl: &G3WorkspaceCrawl) -> G3TsAstroSeoConfigChecksInput {
    let app_roots = crate::roots::astro_app_roots(crawl);
    let policies = app_roots
        .iter()
        .map(|app_root_rel_path| {
            (
                app_root_rel_path.clone(),
                crate::policy::ingest_seo_policy_surface(crawl, app_root_rel_path),
            )
        })
        .collect::<Vec<_>>();
    let integration_contracts = policies
        .iter()
        .map(
            |(app_root_rel_path, astro_policy)| G3TsAstroSeoIntegrationContractInput {
                app_root_rel_path: app_root_rel_path.clone(),
                seo_sources: crate::sources::seo_helper_sources(
                    crawl,
                    app_root_rel_path,
                    astro_policy,
                ),
                package: crate::package::ingest_package_surface(crawl, app_root_rel_path),
                astro_config: crate::astro_config::ingest_astro_config_surface(
                    crawl,
                    app_root_rel_path,
                ),
                astro_policy: astro_policy.clone(),
            },
        )
        .collect::<Vec<_>>();
    let missing_metadata_helper_sources = integration_contracts
        .iter()
        .flat_map(missing_metadata_helper_sources)
        .collect();
    let missing_json_ld_helper_sources = integration_contracts
        .iter()
        .flat_map(missing_json_ld_helper_sources)
        .collect();

    G3TsAstroSeoConfigChecksInput {
        integration_contracts,
        eslint_contracts: policies
            .iter()
            .map(
                |(app_root_rel_path, astro_policy)| G3TsAstroSeoEslintPluginContractInput {
                    app_root_rel_path: app_root_rel_path.clone(),
                    config: crate::eslint::ingest_seo_eslint_surface(
                        crawl,
                        app_root_rel_path,
                        astro_policy,
                    ),
                },
            )
            .collect(),
        missing_metadata_helper_sources,
        missing_json_ld_helper_sources,
        eslint_directives: app_roots
            .iter()
            .flat_map(|app_root_rel_path| {
                crate::eslint_directives::eslint_directives(crawl, app_root_rel_path)
            })
            .collect(),
    }
}

fn missing_metadata_helper_sources(
    contract: &G3TsAstroSeoIntegrationContractInput,
) -> Vec<G3TsAstroSeoMissingMetadataHelperInput> {
    let policy_rel_path = policy_rel_path(&contract.astro_policy);
    contract
        .seo_sources
        .missing_metadata_helpers
        .iter()
        .map(|configured_path| G3TsAstroSeoMissingMetadataHelperInput {
            policy_rel_path: policy_rel_path.clone(),
            configured_path: configured_path.clone(),
        })
        .collect()
}

fn missing_json_ld_helper_sources(
    contract: &G3TsAstroSeoIntegrationContractInput,
) -> Vec<G3TsAstroSeoMissingJsonLdHelperInput> {
    let policy_rel_path = policy_rel_path(&contract.astro_policy);
    contract
        .seo_sources
        .missing_json_ld_helpers
        .iter()
        .map(|configured_path| G3TsAstroSeoMissingJsonLdHelperInput {
            policy_rel_path: policy_rel_path.clone(),
            configured_path: configured_path.clone(),
        })
        .collect()
}

fn policy_rel_path(policy: &G3TsAstroSeoPolicySurfaceState) -> String {
    match policy {
        G3TsAstroSeoPolicySurfaceState::Parsed { snapshot } => snapshot.rel_path.clone(),
        G3TsAstroSeoPolicySurfaceState::Missing { rel_path }
        | G3TsAstroSeoPolicySurfaceState::Unreadable { rel_path, .. }
        | G3TsAstroSeoPolicySurfaceState::ParseError { rel_path, .. }
        | G3TsAstroSeoPolicySurfaceState::MissingAstroPolicy { rel_path } => rel_path.clone(),
    }
}
