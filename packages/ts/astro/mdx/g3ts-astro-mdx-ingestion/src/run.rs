use g3_workspace_crawl::G3RsWorkspaceCrawl as G3WorkspaceCrawl;
use g3ts_astro_mdx_types::{
    G3TsAstroMdxConfigChecksInput, G3TsAstroMdxEslintPluginContractInput,
    G3TsAstroMdxIntegrationContractInput, G3TsAstroMdxMissingComponentMapInput,
    G3TsAstroMdxPolicySurfaceState,
};

#[must_use]
pub fn ingest_for_config_checks(crawl: &G3WorkspaceCrawl) -> G3TsAstroMdxConfigChecksInput {
    let app_roots = crate::roots::astro_app_roots(crawl);
    let policies = app_roots
        .iter()
        .map(|app_root_rel_path| {
            (
                app_root_rel_path.clone(),
                crate::policy::ingest_mdx_policy_surface(crawl, app_root_rel_path),
            )
        })
        .collect::<Vec<_>>();
    let integration_contracts = policies
        .iter()
        .map(
            |(app_root_rel_path, astro_policy)| G3TsAstroMdxIntegrationContractInput {
                app_root_rel_path: app_root_rel_path.clone(),
                mdx_sources: crate::sources::mdx_component_map_sources(
                    crawl,
                    app_root_rel_path,
                    astro_policy,
                ),
                package: crate::package::ingest_package_surface(crawl, app_root_rel_path),
                astro_policy: astro_policy.clone(),
            },
        )
        .collect::<Vec<_>>();
    let missing_component_map_sources = integration_contracts
        .iter()
        .flat_map(missing_component_map_sources)
        .collect();

    G3TsAstroMdxConfigChecksInput {
        integration_contracts,
        eslint_contracts: policies
            .iter()
            .map(
                |(app_root_rel_path, astro_policy)| G3TsAstroMdxEslintPluginContractInput {
                    app_root_rel_path: app_root_rel_path.clone(),
                    config: crate::eslint::ingest_mdx_eslint_surface(
                        crawl,
                        app_root_rel_path,
                        astro_policy,
                    ),
                },
            )
            .collect(),
        missing_component_map_sources,
    }
}

fn missing_component_map_sources(
    contract: &G3TsAstroMdxIntegrationContractInput,
) -> Vec<G3TsAstroMdxMissingComponentMapInput> {
    let policy_rel_path = match &contract.astro_policy {
        G3TsAstroMdxPolicySurfaceState::Parsed { snapshot } => snapshot.rel_path.clone(),
        G3TsAstroMdxPolicySurfaceState::Missing { rel_path }
        | G3TsAstroMdxPolicySurfaceState::Unreadable { rel_path, .. }
        | G3TsAstroMdxPolicySurfaceState::ParseError { rel_path, .. }
        | G3TsAstroMdxPolicySurfaceState::MissingAstroPolicy { rel_path } => rel_path.clone(),
    };

    contract
        .mdx_sources
        .missing_mdx_component_maps
        .iter()
        .map(|configured_path| G3TsAstroMdxMissingComponentMapInput {
            policy_rel_path: policy_rel_path.clone(),
            configured_path: configured_path.clone(),
        })
        .collect()
}
