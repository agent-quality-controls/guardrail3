use g3_workspace_crawl::G3RsWorkspaceCrawl as G3WorkspaceCrawl;
use guardrail3_check_types::G3CheckResult;

/// Runs all Astro check packages against the prepared crawl.
#[must_use]
pub fn check(crawl: &G3WorkspaceCrawl) -> Vec<G3CheckResult> {
    let config_input = g3ts_astro_ingestion::ingest_for_config_checks(crawl);
    let file_tree_input = g3ts_astro_ingestion::ingest_for_file_tree_checks(crawl);

    let mut results = Vec::new();
    results.extend(g3ts_astro_setup_config_checks::check(&config_input));
    results.extend(g3ts_astro_setup_file_tree_checks::check(&file_tree_input));
    results.extend(g3ts_astro_content_config_checks::check(&config_input));
    results.extend(g3ts_astro_content_file_tree_checks::check(&file_tree_input));
    results.extend(g3ts_astro_mdx_config_checks::check(&config_input));
    results.extend(g3ts_astro_seo_config_checks::check(&config_input));
    results.extend(g3ts_astro_state_file_tree_checks::check(&file_tree_input));
    results
}
