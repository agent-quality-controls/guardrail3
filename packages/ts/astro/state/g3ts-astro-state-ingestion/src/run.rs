use g3_workspace_crawl::G3RsWorkspaceCrawl as G3WorkspaceCrawl;
use g3ts_astro_types::G3TsAstroFileTreeChecksInput;

#[must_use]
pub fn ingest_for_file_tree_checks(crawl: &G3WorkspaceCrawl) -> G3TsAstroFileTreeChecksInput {
    g3ts_astro_check_support::ingestion::ingest_for_file_tree_checks(crawl)
}
