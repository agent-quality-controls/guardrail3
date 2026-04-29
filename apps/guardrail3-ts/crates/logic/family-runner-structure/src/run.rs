use g3_workspace_crawl::G3RsWorkspaceCrawl as G3WorkspaceCrawl;
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
        SupportedFamily::AstroSetup => {
            let setup_config_input = g3ts_astro_setup_ingestion::ingest_for_config_checks(crawl);
            let setup_file_tree_input =
                g3ts_astro_setup_ingestion::ingest_for_file_tree_checks(crawl);
            let mut results = Vec::new();
            results.extend(g3ts_astro_setup_config_checks::check(&setup_config_input));
            results.extend(g3ts_astro_setup_file_tree_checks::check(
                &setup_file_tree_input,
            ));
            Ok(results)
        }
        SupportedFamily::AstroContent => {
            let content_config_input =
                g3ts_astro_content_ingestion::ingest_for_config_checks(crawl);
            let content_file_tree_input =
                g3ts_astro_content_ingestion::ingest_for_file_tree_checks(crawl);
            let mut results = Vec::new();
            results.extend(g3ts_astro_content_config_checks::check(
                &content_config_input,
            ));
            results.extend(g3ts_astro_content_file_tree_checks::check(
                &content_file_tree_input,
            ));
            Ok(results)
        }
        SupportedFamily::AstroMdx => {
            let mdx_config_input = g3ts_astro_mdx_ingestion::ingest_for_config_checks(crawl);
            let mut results = Vec::new();
            results.extend(g3ts_astro_mdx_config_checks::check(&mdx_config_input));
            Ok(results)
        }
        SupportedFamily::AstroI18n => {
            let i18n_config_input = g3ts_astro_i18n_ingestion::ingest_for_config_checks(crawl);
            let mut results = Vec::new();
            results.extend(g3ts_astro_i18n_config_checks::check(&i18n_config_input));
            Ok(results)
        }
        SupportedFamily::AstroSeo => {
            let seo_config_input = g3ts_astro_seo_ingestion::ingest_for_config_checks(crawl);
            let mut results = Vec::new();
            results.extend(g3ts_astro_seo_config_checks::check(&seo_config_input));
            Ok(results)
        }
        SupportedFamily::AstroState => {
            let state_file_tree_input =
                g3ts_astro_state_ingestion::ingest_for_file_tree_checks(crawl);
            let mut results = Vec::new();
            results.extend(g3ts_astro_state_file_tree_checks::check(
                &state_file_tree_input,
            ));
            Ok(results)
        }
        SupportedFamily::Arch => {
            let config_inputs =
                g3ts_arch_ingestion::ingest_for_config_checks(crawl).map_err(|error| {
                    FamilyRunError {
                        message: format!("{error:?}"),
                    }
                })?;
            let file_tree_input =
                g3ts_arch_ingestion::ingest_for_file_tree_checks(crawl).map_err(|error| {
                    FamilyRunError {
                        message: format!("{error:?}"),
                    }
                })?;
            let source_inputs =
                g3ts_arch_ingestion::ingest_for_source_checks(crawl).map_err(|error| {
                    FamilyRunError {
                        message: format!("{error:?}"),
                    }
                })?;

            let mut results = Vec::new();
            results.extend(g3ts_arch_config_checks::check(&config_inputs));
            results.extend(g3ts_arch_file_tree_checks::check(&file_tree_input));
            for source_input in source_inputs {
                results.extend(g3ts_arch_source_checks::check(&source_input));
            }
            Ok(results)
        }
        SupportedFamily::Apparch => {
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
        SupportedFamily::Eslint
        | SupportedFamily::Tsconfig
        | SupportedFamily::Package
        | SupportedFamily::Npmrc
        | SupportedFamily::Jscpd
        | SupportedFamily::Hooks => Err(FamilyRunError {
            message: format!("structure group does not handle {family:?}"),
        }),
    }
}
