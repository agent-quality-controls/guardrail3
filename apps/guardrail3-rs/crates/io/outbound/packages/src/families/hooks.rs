use g3rs_hooks_config_checks::check as check_config;
use g3rs_hooks_file_tree_checks::check as check_filetree;
use g3rs_hooks_ingestion::{
    ingest_for_config_checks, ingest_for_file_tree_checks, ingest_for_source_checks,
};
use g3rs_hooks_source_checks::check as check_source;
use g3rs_workspace_crawl::G3RsWorkspaceCrawl;
use guardrail3_check_types::G3CheckResult;

pub(crate) fn run(crawl: &G3RsWorkspaceCrawl) -> Result<Vec<G3CheckResult>, String> {
    let config_input = ingest_for_config_checks(crawl).map_err(|error| format!("{error:?}"))?;
    let filetree_input = ingest_for_file_tree_checks(crawl).map_err(|error| format!("{error:?}"))?;
    let source_inputs = ingest_for_source_checks(crawl).map_err(|error| format!("{error:?}"))?;

    let mut results = Vec::new();
    results.extend(check_config(&config_input));
    results.extend(check_filetree(&filetree_input));
    results.extend(source_inputs.iter().flat_map(check_source));
    Ok(results)
}
