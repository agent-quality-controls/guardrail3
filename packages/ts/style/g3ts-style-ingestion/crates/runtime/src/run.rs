use g3_workspace_crawl::G3RsWorkspaceCrawl as G3WorkspaceCrawl;
use g3ts_style_types::{G3TsStyleConfigChecksInput, G3TsStyleContractInput};

#[must_use]
pub fn ingest_for_config_checks(crawl: &G3WorkspaceCrawl) -> G3TsStyleConfigChecksInput {
    G3TsStyleConfigChecksInput {
        contracts: crate::roots::style_roots(crawl)
            .into_iter()
            .map(|app_root_rel_path| {
                let policy = crate::policy::ingest_policy(crawl, &app_root_rel_path);
                G3TsStyleContractInput {
                    package: crate::package::ingest_package_surface(crawl, &app_root_rel_path),
                    stylelint_config: crate::stylelint::ingest_stylelint_config(
                        crawl,
                        &app_root_rel_path,
                        &policy,
                    ),
                    eslint_config: crate::eslint::ingest_eslint_config(
                        crawl,
                        &app_root_rel_path,
                        &policy,
                    ),
                    app_root_rel_path,
                    policy,
                }
            })
            .collect(),
    }
}
