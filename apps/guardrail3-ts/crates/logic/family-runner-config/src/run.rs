use g3_workspace_crawl::G3RsWorkspaceCrawl as G3WorkspaceCrawl;
use guardrail3_ts_app_types::{FamilyResults, FamilyRunError, SupportedFamily};

/// Runs the supported config family group against the prepared crawl.
///
/// # Errors
///
/// Returns [`FamilyRunError`] when ingestion for the selected family fails.
pub fn run(
    family: SupportedFamily,
    crawl: &G3WorkspaceCrawl,
) -> Result<FamilyResults, FamilyRunError> {
    match family {
        SupportedFamily::Eslint => Ok(g3ts_eslint_config_checks::check(
            &g3ts_eslint_ingestion::ingest_for_config_checks(crawl),
        )),
        SupportedFamily::AstroSetup
        | SupportedFamily::AstroContent
        | SupportedFamily::AstroMdx
        | SupportedFamily::AstroI18n
        | SupportedFamily::AstroSeo
        | SupportedFamily::AstroState => Err(FamilyRunError {
            message: "config group does not handle Astro structure families".to_owned(),
        }),
        SupportedFamily::Arch => Err(FamilyRunError {
            message: "config group does not handle Arch".to_owned(),
        }),
        SupportedFamily::Apparch => Err(FamilyRunError {
            message: "config group does not handle Apparch".to_owned(),
        }),
        SupportedFamily::Tsconfig => Ok(g3ts_tsconfig_config_checks::check(
            &g3ts_tsconfig_ingestion::ingest_for_config_checks(crawl),
        )),
        SupportedFamily::Package => Ok(g3ts_package_config_checks::check(
            &g3ts_package_ingestion::ingest_for_config_checks(crawl),
        )),
        SupportedFamily::Npmrc => Ok(g3ts_npmrc_config_checks::check(
            &g3ts_npmrc_ingestion::ingest_for_config_checks(crawl),
        )),
        SupportedFamily::Jscpd => Ok(g3ts_jscpd_config_checks::check(
            &g3ts_jscpd_ingestion::ingest_for_config_checks(crawl),
        )),
        SupportedFamily::Hooks => Err(FamilyRunError {
            message: "config group does not handle hooks".to_owned(),
        }),
    }
}
