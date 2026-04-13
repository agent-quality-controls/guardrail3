use g3rs_workspace_crawl::G3RsWorkspaceCrawl;
use guardrail3_check_types::G3CheckResult;

pub(crate) fn run(crawl: &G3RsWorkspaceCrawl) -> Result<Vec<G3CheckResult>, String> {
    let config_input = g3rs_garde_ingestion::ingest_for_config_checks(crawl)
        .map_err(|error| format!("{error:?}"))?;
    let source_input = g3rs_garde_ingestion::ingest_for_source_checks(crawl)
        .map_err(|error| format!("{error:?}"))?;

    let mut results = g3rs_garde_config_checks::check(&config_input);
    results.extend(g3rs_garde_source_checks::check(&source_input));

    Ok(results)
}
