use g3_workspace_crawl::G3RsWorkspaceCrawl as G3WorkspaceCrawl;
use g3ts_astro_types::G3TsAstroStateFileTreeChecksInput;

#[must_use]
pub fn ingest_for_file_tree_checks(crawl: &G3WorkspaceCrawl) -> G3TsAstroStateFileTreeChecksInput {
    let app_roots = g3ts_astro_check_support::surfaces::app_root_inputs(crawl);
    G3TsAstroStateFileTreeChecksInput {
        build_collection_roots: g3ts_astro_check_support::surfaces::build_collection_roots(
            crawl,
            &app_roots,
        ),
        live_collection_roots: g3ts_astro_check_support::surfaces::live_collection_roots(
            crawl,
            &app_roots,
        ),
    }
}
