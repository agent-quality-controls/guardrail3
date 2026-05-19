use g3_workspace_crawl::G3WorkspaceCrawl;
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
    enabled_families: &[SupportedFamily],
) -> Result<FamilyResults, FamilyRunError> {
    match family {
        SupportedFamily::Hooks => {
            let requirements = hook_contracts(enabled_families);
            let mut config_input = g3ts_hooks_ingestion::ingest_for_config_checks(crawl);
            config_input.replace_requirements(requirements.clone());
            let file_tree_input = g3ts_hooks_ingestion::ingest_for_file_tree_checks(crawl);
            let mut source_inputs = g3ts_hooks_ingestion::ingest_for_source_checks(crawl);
            for input in &mut source_inputs {
                input.replace_requirements(requirements.clone());
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
        | SupportedFamily::AstroI18n
        | SupportedFamily::AstroMedia
        | SupportedFamily::AstroSeo
        | SupportedFamily::AstroState
        | SupportedFamily::Arch
        | SupportedFamily::Apparch
        | SupportedFamily::Style
        | SupportedFamily::Fmt
        | SupportedFamily::Spelling
        | SupportedFamily::Typecov
        | SupportedFamily::Tsconfig
        | SupportedFamily::Package
        | SupportedFamily::Npmrc
        | SupportedFamily::Jscpd
        | SupportedFamily::Topology => Err(FamilyRunError {
            message: format!("hooks runner does not handle {family:?}"),
        }),
    }
}

/// Collects hook contracts from enabled families that own TypeScript hook requirements.
fn hook_contracts(enabled_families: &[SupportedFamily]) -> Vec<G3TsHookRequirement> {
    let mut requirements = Vec::new();
    if enabled_families.contains(&SupportedFamily::AstroSetup) {
        requirements.extend(g3ts_astro_setup_hook_contract::hook_contract());
    }
    if enabled_families.contains(&SupportedFamily::AstroContent) {
        requirements.extend(g3ts_astro_content_hook_contract::hook_contract());
    }
    if enabled_families.contains(&SupportedFamily::AstroMdx) {
        requirements.extend(g3ts_astro_mdx_hook_contract::hook_contract());
    }
    if enabled_families.contains(&SupportedFamily::AstroI18n) {
        requirements.extend(g3ts_astro_i18n_hook_contract::hook_contract());
    }
    if enabled_families.contains(&SupportedFamily::AstroMedia) {
        requirements.extend(g3ts_astro_media_hook_contract::hook_contract());
    }
    if enabled_families.contains(&SupportedFamily::AstroSeo) {
        requirements.extend(g3ts_astro_seo_hook_contract::hook_contract());
    }
    if enabled_families.contains(&SupportedFamily::Fmt) {
        requirements.extend(g3ts_fmt_hook_contract::hook_contract());
    }
    if enabled_families.contains(&SupportedFamily::Spelling) {
        requirements.extend(g3ts_spelling_hook_contract::hook_contract());
    }
    if enabled_families.contains(&SupportedFamily::Typecov) {
        requirements.extend(g3ts_typecov_hook_contract::hook_contract());
    }
    if enabled_families.contains(&SupportedFamily::Style) {
        requirements.extend(g3ts_style_hook_contract::hook_contract());
    }
    requirements
}
