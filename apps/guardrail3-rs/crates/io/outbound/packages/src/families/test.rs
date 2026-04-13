use g3rs_test_config_checks::check as check_config;
use g3rs_test_file_tree_checks::check as check_filetree;
use g3rs_test_ingestion::{ingest_for_config_checks, ingest_for_file_tree_checks, ingest_for_source_checks};
use g3rs_test_source_checks::check as check_source;
use g3rs_workspace_crawl::G3RsWorkspaceCrawl;
use guardrail3_check_types::G3CheckResult;

pub(crate) fn run(crawl: &G3RsWorkspaceCrawl) -> Result<Vec<G3CheckResult>, String> {
    let config_inputs =
        ingest_for_config_checks(crawl).map_err(|error| format!("{error:?}"))?;
    let filetree_inputs =
        ingest_for_file_tree_checks(crawl).map_err(|error| format!("{error:?}"))?;
    let source_inputs =
        ingest_for_source_checks(crawl).map_err(|error| format!("{error:?}"))?;

    let mut results = Vec::new();

    for input in config_inputs {
        results.extend(check_config(&input));
    }
    for input in filetree_inputs {
        results.extend(check_filetree(&input));
    }
    for input in source_inputs {
        results.extend(check_source(&input));
    }

    Ok(results)
}
