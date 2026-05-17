use g3_workspace_crawl::G3WorkspaceCrawl;
use g3ts_typecov_types::{G3TsTypecovConfigChecksInput, G3TsTypecovContractInput};

#[must_use]
pub fn ingest_for_config_checks(crawl: &G3WorkspaceCrawl) -> G3TsTypecovConfigChecksInput {
    G3TsTypecovConfigChecksInput {
        contracts: crate::roots::typecov_roots(crawl)
            .into_iter()
            .map(|app_root_rel_path| G3TsTypecovContractInput {
                package: crate::package::ingest_package_surface(crawl, &app_root_rel_path),
                syncpack_config: crate::syncpack::ingest_syncpack_config(crawl, &app_root_rel_path),
                app_root_rel_path,
            })
            .collect(),
    }
}
