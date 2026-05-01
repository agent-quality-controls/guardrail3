use g3_workspace_crawl::G3RsWorkspaceCrawl as G3WorkspaceCrawl;
use g3ts_fmt_types::{G3TsFmtConfigChecksInput, G3TsFmtContractInput};

#[must_use]
pub fn ingest_for_config_checks(crawl: &G3WorkspaceCrawl) -> G3TsFmtConfigChecksInput {
    G3TsFmtConfigChecksInput {
        contracts: crate::roots::fmt_roots(crawl)
            .into_iter()
            .map(|app_root_rel_path| G3TsFmtContractInput {
                package: crate::package::ingest_package_surface(crawl, &app_root_rel_path),
                prettier_config: crate::config::ingest_prettier_config(crawl, &app_root_rel_path),
                syncpack_config: crate::syncpack::ingest_syncpack_config(crawl, &app_root_rel_path),
                app_root_rel_path,
            })
            .collect(),
    }
}

#[cfg(test)]
#[path = "run_tests/mod.rs"]
mod run_tests;
