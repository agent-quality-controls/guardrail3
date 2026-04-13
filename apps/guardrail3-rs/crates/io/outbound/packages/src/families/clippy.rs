use g3rs_clippy_ingestion::G3RsClippyIngestionError;
use g3rs_workspace_crawl::G3RsWorkspaceCrawl;
use guardrail3_check_types::G3CheckResult;

pub(crate) fn run(crawl: &G3RsWorkspaceCrawl) -> Result<Vec<G3CheckResult>, String> {
    let mut results = match g3rs_clippy_ingestion::ingest_for_config_checks(crawl) {
        Ok(config_input) => g3rs_clippy_config_checks::check(&config_input),
        Err(G3RsClippyIngestionError::ClippyTomlNotFound) => Vec::new(),
        Err(error) => return Err(format!("{error:?}")),
    };
    let filetree_input = g3rs_clippy_ingestion::ingest_for_file_tree_checks(crawl)
        .map_err(|error| format!("{error:?}"))?;
    results.extend(g3rs_clippy_filetree_checks::check(&filetree_input));

    Ok(results)
}
