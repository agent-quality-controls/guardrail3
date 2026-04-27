use g3_workspace_crawl::G3RsWorkspaceCrawl as G3WorkspaceCrawl;
use g3ts_astro_types::G3TsAstroConfigChecksInput;

#[must_use]
pub fn ingest_for_config_checks(crawl: &G3WorkspaceCrawl) -> G3TsAstroConfigChecksInput {
    g3ts_astro_check_support::ingestion::ingest_for_config_checks(crawl)
}
