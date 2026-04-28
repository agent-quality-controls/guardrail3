use g3_workspace_crawl::G3RsWorkspaceCrawl as G3WorkspaceCrawl;
use g3ts_hooks_contract_types::G3TsHookRequirement;
use guardrail3_ts_app_types::{FamilyResults, FamilyRunError, SupportedFamily};

/// Runs the TS hooks family against the prepared crawl.
///
/// # Errors
///
/// Returns [`FamilyRunError`] when called for a non-hooks family.
pub fn run(
    family: SupportedFamily,
    crawl: &G3WorkspaceCrawl,
) -> Result<FamilyResults, FamilyRunError> {
    match family {
        SupportedFamily::Hooks => {
            let requirements = hook_contracts();
            let mut config_input = g3ts_hooks_ingestion::ingest_for_config_checks(crawl);
            config_input.requirements.clone_from(&requirements);
            let file_tree_input = g3ts_hooks_ingestion::ingest_for_file_tree_checks(crawl);
            let mut source_inputs = g3ts_hooks_ingestion::ingest_for_source_checks(crawl);
            for input in &mut source_inputs {
                input.requirements.clone_from(&requirements);
            }

            let mut results = Vec::new();
            results.extend(g3ts_hooks_config_checks::check(&config_input));
            results.extend(g3ts_hooks_file_tree_checks::check(&file_tree_input));
            results.extend(g3ts_hooks_source_checks::check_effective(&source_inputs));
            Ok(results)
        }
        SupportedFamily::Eslint
        | SupportedFamily::AstroSetup
        | SupportedFamily::AstroContent
        | SupportedFamily::AstroMdx
        | SupportedFamily::AstroSeo
        | SupportedFamily::AstroState
        | SupportedFamily::Arch
        | SupportedFamily::Apparch
        | SupportedFamily::Tsconfig
        | SupportedFamily::Package
        | SupportedFamily::Npmrc
        | SupportedFamily::Jscpd => Err(FamilyRunError {
            message: format!("hooks runner does not handle {family:?}"),
        }),
    }
}

/// Collects hook contracts from families that own TypeScript hook requirements.
fn hook_contracts() -> Vec<G3TsHookRequirement> {
    let mut requirements = Vec::new();
    requirements.extend(g3ts_astro_setup_hook_contract::hook_contract());
    requirements.extend(g3ts_astro_content_hook_contract::hook_contract());
    requirements.extend(g3ts_astro_mdx_hook_contract::hook_contract());
    requirements.extend(g3ts_astro_seo_hook_contract::hook_contract());
    requirements
}
