use g3rs_fmt_ingestion::G3RsFmtIngestionError;
use g3rs_workspace_crawl::G3RsWorkspaceCrawl;
use guardrail3_check_types::G3CheckResult;

pub(crate) fn run(crawl: &G3RsWorkspaceCrawl) -> Result<Vec<G3CheckResult>, String> {
    let mut results = match g3rs_fmt_ingestion::ingest_for_config_checks(crawl) {
        Ok(config_input) => g3rs_fmt_config_checks::check(&config_input),
        Err(G3RsFmtIngestionError::RustfmtTomlNotFound) => Vec::new(),
        Err(error) => return Err(format!("{error:?}")),
    };

    let file_tree_input =
        g3rs_fmt_ingestion::ingest_for_file_tree_checks(crawl).map_err(|error| format!("{error:?}"))?;
    results.extend(g3rs_fmt_filetree_checks::check(&file_tree_input));

    Ok(results)
}
