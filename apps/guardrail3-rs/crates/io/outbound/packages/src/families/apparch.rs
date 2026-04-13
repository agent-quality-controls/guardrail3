use g3rs_apparch_config_checks::check as check_config;
use g3rs_apparch_ingestion::{ingest_for_config_checks, ingest_for_source_checks};
use g3rs_apparch_source_checks::check as check_source;
use g3rs_workspace_crawl::G3RsWorkspaceCrawl;
use guardrail3_check_types::G3CheckResult;

pub(crate) fn run(crawl: &G3RsWorkspaceCrawl) -> Result<Vec<G3CheckResult>, String> {
    let config_input =
        ingest_for_config_checks(crawl).map_err(|error| format!("{error:?}"))?;
    let source_input =
        ingest_for_source_checks(crawl).map_err(|error| format!("{error:?}"))?;

    let mut results = check_config(&config_input);
    results.extend(check_source(&source_input));

    Ok(results)
}
