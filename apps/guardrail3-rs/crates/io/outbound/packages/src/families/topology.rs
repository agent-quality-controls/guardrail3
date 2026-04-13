use g3rs_topology_file_tree_checks::check as check_file_tree;
use g3rs_topology_ingestion::ingest_for_file_tree_checks;
use g3rs_workspace_crawl::G3RsWorkspaceCrawl;
use guardrail3_check_types::G3CheckResult;

pub(crate) fn run(crawl: &G3RsWorkspaceCrawl) -> Result<Vec<G3CheckResult>, String> {
    let input = ingest_for_file_tree_checks(crawl).map_err(|error| format!("{error:?}"))?;
    Ok(check_file_tree(&input))
}
