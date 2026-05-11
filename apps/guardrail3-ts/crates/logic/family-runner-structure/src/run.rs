use g3_workspace_crawl::G3WorkspaceCrawl;
use guardrail3_ts_app_types::{FamilyResults, FamilyRunError, SupportedFamily};

/// Runs the TS structure family group against the prepared crawl.
///
/// # Errors
///
/// Returns [`FamilyRunError`] when ingestion for the selected family fails.
pub fn run(
    family: SupportedFamily,
    crawl: &G3WorkspaceCrawl,
) -> Result<FamilyResults, FamilyRunError> {
    match family {
        SupportedFamily::AstroSetup => Ok(run_astro_setup(crawl)),
        SupportedFamily::AstroContent => Ok(run_astro_content(crawl)),
        SupportedFamily::AstroMdx => Ok(run_astro_mdx(crawl)),
        SupportedFamily::AstroI18n => Ok(run_astro_i18n(crawl)),
        SupportedFamily::AstroMedia => Ok(run_astro_media(crawl)),
        SupportedFamily::AstroSeo => Ok(run_astro_seo(crawl)),
        SupportedFamily::AstroState => Ok(run_astro_state(crawl)),
        SupportedFamily::Arch => Ok(run_arch(crawl)),
        SupportedFamily::Apparch => run_apparch(crawl),
        SupportedFamily::Eslint
        | SupportedFamily::Tsconfig
        | SupportedFamily::Package
        | SupportedFamily::Npmrc
        | SupportedFamily::Jscpd
        | SupportedFamily::Style
        | SupportedFamily::Fmt
        | SupportedFamily::Spelling
        | SupportedFamily::Typecov
        | SupportedFamily::Hooks
        | SupportedFamily::Topology => Err(FamilyRunError {
            message: format!("structure group does not handle {family:?}"),
        }),
    }
}

/// Runs astro-setup config + file-tree checks against `crawl`.
fn run_astro_setup(crawl: &G3WorkspaceCrawl) -> FamilyResults {
    let setup_config_input = g3ts_astro_setup_ingestion::ingest_for_config_checks(crawl);
    let setup_file_tree_input = g3ts_astro_setup_ingestion::ingest_for_file_tree_checks(crawl);
    let mut results = Vec::new();
    results.extend(g3ts_astro_setup_config_checks::check(&setup_config_input));
    results.extend(g3ts_astro_setup_file_tree_checks::check(
        &setup_file_tree_input,
    ));
    results
}

/// Runs astro-content config + file-tree checks against `crawl`.
fn run_astro_content(crawl: &G3WorkspaceCrawl) -> FamilyResults {
    let content_config_input = g3ts_astro_content_ingestion::ingest_for_config_checks(crawl);
    let content_file_tree_input = g3ts_astro_content_ingestion::ingest_for_file_tree_checks(crawl);
    let mut results = Vec::new();
    results.extend(g3ts_astro_content_config_checks::check(
        &content_config_input,
    ));
    results.extend(g3ts_astro_content_file_tree_checks::check(
        &content_file_tree_input,
    ));
    results
}

/// Runs astro-mdx config checks against `crawl`.
fn run_astro_mdx(crawl: &G3WorkspaceCrawl) -> FamilyResults {
    let mdx_config_input = g3ts_astro_mdx_ingestion::ingest_for_config_checks(crawl);
    let mut results = Vec::new();
    results.extend(g3ts_astro_mdx_config_checks::check(&mdx_config_input));
    results
}

/// Runs astro-i18n config checks against `crawl`.
fn run_astro_i18n(crawl: &G3WorkspaceCrawl) -> FamilyResults {
    let i18n_config_input = g3ts_astro_i18n_ingestion::ingest_for_config_checks(crawl);
    let mut results = Vec::new();
    results.extend(g3ts_astro_i18n_config_checks::check(&i18n_config_input));
    results
}

/// Runs astro-media config checks against `crawl`.
fn run_astro_media(crawl: &G3WorkspaceCrawl) -> FamilyResults {
    let media_config_input = g3ts_astro_media_ingestion::ingest_for_config_checks(crawl);
    let mut results = Vec::new();
    results.extend(g3ts_astro_media_config_checks::check(&media_config_input));
    results
}

/// Runs astro-seo config checks against `crawl`.
fn run_astro_seo(crawl: &G3WorkspaceCrawl) -> FamilyResults {
    let seo_config_input = g3ts_astro_seo_ingestion::ingest_for_config_checks(crawl);
    let mut results = Vec::new();
    results.extend(g3ts_astro_seo_config_checks::check(&seo_config_input));
    results
}

/// Runs astro-state file-tree checks against `crawl`.
fn run_astro_state(crawl: &G3WorkspaceCrawl) -> FamilyResults {
    let state_file_tree_input = g3ts_astro_state_ingestion::ingest_for_file_tree_checks(crawl);
    let mut results = Vec::new();
    results.extend(g3ts_astro_state_file_tree_checks::check(
        &state_file_tree_input,
    ));
    results
}

/// Runs arch config + file-tree + per-facade source checks against `crawl`.
fn run_arch(crawl: &G3WorkspaceCrawl) -> FamilyResults {
    let config_inputs = g3ts_arch_ingestion::ingest_for_config_checks(crawl);
    let file_tree_input = g3ts_arch_ingestion::ingest_for_file_tree_checks(crawl);
    let source_inputs = g3ts_arch_ingestion::ingest_for_source_checks(crawl);

    let mut results = Vec::new();
    results.extend(g3ts_arch_config_checks::check(&config_inputs));
    results.extend(g3ts_arch_file_tree_checks::check(&file_tree_input));
    for source_input in source_inputs {
        results.extend(g3ts_arch_source_checks::check(&source_input));
    }
    results
}

/// Runs apparch config + source checks against `crawl`.
fn run_apparch(crawl: &G3WorkspaceCrawl) -> Result<FamilyResults, FamilyRunError> {
    let config_input =
        g3ts_apparch_ingestion::ingest_for_config_checks(crawl).map_err(|error| {
            FamilyRunError {
                message: format!("{error:?}"),
            }
        })?;
    let source_input =
        g3ts_apparch_ingestion::ingest_for_source_checks(crawl).map_err(|error| {
            FamilyRunError {
                message: format!("{error:?}"),
            }
        })?;

    let mut results = Vec::new();
    results.extend(g3ts_apparch_config_checks::check(&config_input));
    results.extend(g3ts_apparch_source_checks::check(&source_input));
    Ok(results)
}
