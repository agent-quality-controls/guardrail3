use g3_workspace_crawl::G3WorkspaceCrawl;
use g3ts_style_types::{G3TsStyleConfigChecksInput, G3TsStyleContractInput};

#[must_use]
pub fn ingest_for_config_checks(crawl: &G3WorkspaceCrawl) -> G3TsStyleConfigChecksInput {
    let style_roots = crate::roots::style_roots(crawl);
    G3TsStyleConfigChecksInput {
        contracts: style_roots
            .iter()
            .cloned()
            .map(|app_root_rel_path| {
                let policy = crate::policy::ingest_policy(crawl, &app_root_rel_path);
                let package = crate::package::ingest_package_surface(crawl, &app_root_rel_path);
                G3TsStyleContractInput {
                    syncpack_config: crate::syncpack::ingest_syncpack_config(
                        crawl,
                        &app_root_rel_path,
                    ),
                    package,
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
        eslint_directives: style_roots
            .iter()
            .flat_map(|app_root_rel_path| {
                let policy = crate::policy::ingest_policy(crawl, app_root_rel_path);
                crate::eslint_directives::eslint_directives(crawl, app_root_rel_path, &policy)
            })
            .collect(),
    }
}
