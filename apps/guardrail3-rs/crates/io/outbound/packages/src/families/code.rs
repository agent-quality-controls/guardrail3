use g3rs_workspace_crawl::G3RsWorkspaceCrawl;
use guardrail3_check_types::G3CheckResult;

pub(crate) fn run(crawl: &G3RsWorkspaceCrawl) -> Result<Vec<G3CheckResult>, String> {
    let config_input =
        g3rs_code_ingestion::ingest_for_config_checks(crawl).map_err(|error| format!("{error:?}"))?;
    let mut results = g3rs_code_config_checks::check(&config_input);

    let file_tree_input =
        g3rs_code_ingestion::ingest_for_file_tree_checks(crawl).map_err(|error| format!("{error:?}"))?;
    results.extend(g3rs_code_file_tree_checks::check(&file_tree_input));

    let source_inputs =
        g3rs_code_ingestion::ingest_for_source_checks(crawl).map_err(|error| format!("{error:?}"))?;
    for input in source_inputs {
        results.extend(g3rs_code_source_checks::check(&input));
    }

    Ok(results)
}
