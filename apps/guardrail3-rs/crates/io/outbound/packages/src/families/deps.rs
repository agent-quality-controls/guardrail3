use g3rs_deps_config_checks::check as check_config;
use g3rs_deps_filetree_checks::check as check_filetree;
use g3rs_deps_ingestion::{ingest_for_config_checks, ingest_for_file_tree_checks};
use g3rs_workspace_crawl::G3RsWorkspaceCrawl;
use guardrail3_check_types::G3CheckResult;

pub(crate) fn run(crawl: &G3RsWorkspaceCrawl) -> Result<Vec<G3CheckResult>, String> {
    let config_input = ingest_for_config_checks(crawl).map_err(|error| format!("{error:?}"))?;
    let filetree_input = ingest_for_file_tree_checks(crawl).map_err(|error| format!("{error:?}"))?;

    let mut results = Vec::new();
    results.extend(config_input.into_iter().flat_map(|input| check_config(&input)));
    results.extend(check_filetree(&filetree_input));

    Ok(results)
}
