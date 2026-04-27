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
        SupportedFamily::Astro => {
            let config_input = g3ts_astro_ingestion::ingest_for_config_checks(crawl);
            let file_tree_input = g3ts_astro_ingestion::ingest_for_file_tree_checks(crawl);

            let mut results = Vec::new();
            results.extend(g3ts_astro_config_checks::check_setup(&config_input));
            results.extend(g3ts_astro_file_tree_checks::check_setup(&file_tree_input));
            results.extend(g3ts_astro_config_checks::check_content(&config_input));
            results.extend(g3ts_astro_file_tree_checks::check_content(&file_tree_input));
            results.extend(g3ts_astro_config_checks::check_mdx(&config_input));
            results.extend(g3ts_astro_config_checks::check_seo(&config_input));
            results.extend(g3ts_astro_file_tree_checks::check_state(&file_tree_input));
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
        | SupportedFamily::Jscpd => Err(FamilyRunError {
            message: format!("structure group does not handle {family:?}"),
        }),
    }
}
