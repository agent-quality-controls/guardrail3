use g3_workspace_crawl::G3WorkspaceCrawl;
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
    }
}
