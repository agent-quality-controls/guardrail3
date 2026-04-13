use g3rs_toolchain_config_checks::check as check_config;
use g3rs_toolchain_filetree_checks::check as check_filetree;
use g3rs_toolchain_ingestion::{
    G3RsToolchainIngestionError, ingest_for_config_checks, ingest_for_file_tree_checks,
};
use g3rs_workspace_crawl::G3RsWorkspaceCrawl;
use guardrail3_check_types::G3CheckResult;

pub(crate) fn run(crawl: &G3RsWorkspaceCrawl) -> Result<Vec<G3CheckResult>, String> {
    let mut results = match ingest_for_config_checks(crawl) {
        Ok(config_input) => check_config(&config_input),
        Err(G3RsToolchainIngestionError::ToolchainTomlNotFound) => Vec::new(),
        Err(error) => return Err(format!("{error:?}")),
    };

    let filetree_input =
        ingest_for_file_tree_checks(crawl).map_err(|error| format!("{error:?}"))?;
    results.extend(check_filetree(&filetree_input));

    Ok(results)
}
