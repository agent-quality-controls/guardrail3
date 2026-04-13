use g3rs_arch_config_checks::check as check_config;
use g3rs_arch_file_tree_checks::check as check_file_tree;
use g3rs_arch_ingestion::{ingest_for_config_checks, ingest_for_file_tree_checks, ingest_for_source_checks};
use g3rs_arch_source_checks::check as check_source;
use g3rs_workspace_crawl::G3RsWorkspaceCrawl;
use guardrail3_check_types::G3CheckResult;

pub(crate) fn run(crawl: &G3RsWorkspaceCrawl) -> Result<Vec<G3CheckResult>, String> {
    let config_inputs = ingest_for_config_checks(crawl).map_err(|error| format!("{error:?}"))?;
    let file_tree_input =
        ingest_for_file_tree_checks(crawl).map_err(|error| format!("{error:?}"))?;
    let source_inputs = ingest_for_source_checks(crawl).map_err(|error| format!("{error:?}"))?;

    let mut results = Vec::new();
    for config_input in config_inputs {
        results.extend(check_config(&config_input));
    }
    results.extend(check_file_tree(&file_tree_input));
    for source_input in source_inputs {
        results.extend(check_source(&source_input));
    }
    Ok(results)
}
