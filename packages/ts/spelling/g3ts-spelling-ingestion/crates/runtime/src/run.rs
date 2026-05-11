use g3_workspace_crawl::G3WorkspaceCrawl;
use g3ts_spelling_types::{G3TsSpellingConfigChecksInput, G3TsSpellingContractInput};

#[must_use]
pub fn ingest_for_config_checks(crawl: &G3WorkspaceCrawl) -> G3TsSpellingConfigChecksInput {
    G3TsSpellingConfigChecksInput {
        contracts: crate::roots::spelling_roots(crawl)
            .into_iter()
            .map(|app_root_rel_path| G3TsSpellingContractInput {
                package: crate::package::ingest_package_surface(crawl, &app_root_rel_path),
                cspell_config: crate::config::ingest_cspell_config(crawl, &app_root_rel_path),
                syncpack_config: crate::syncpack::ingest_syncpack_config(crawl, &app_root_rel_path),
                app_root_rel_path,
            })
            .collect(),
    }
}

#[cfg(test)]
#[path = "run_tests/mod.rs"]
mod run_tests;
