use std::path::Path;

use guardrail3_app_core::crawl::CrawlResult;
use guardrail3_domain_report::CheckResult;

use crate::validate::eslint::eslint_check;
use super::jscpd_check;
use super::npmrc_check;
use super::package_check;
use super::tsconfig_check;
use guardrail3_outbound_traits::FileSystem;
pub fn check(fs: &dyn FileSystem, path: &Path, crawl: &CrawlResult) -> Vec<CheckResult> {
    let mut results = Vec::new();

    eslint_check::check_eslint_config(fs, &crawl.eslint_configs, path, &mut results);
    tsconfig_check::check_tsconfig(
        fs,
        &crawl.tsconfigs,
        &crawl.tsconfig_bases,
        path,
        &mut results,
    );
    npmrc_check::check_npmrc(fs, &crawl.npmrcs, path, &mut results);
    package_check::check_package_json(fs, &crawl.package_jsons, path, &mut results);
    jscpd_check::check_jscpd(fs, &crawl.jscpd_configs, path, &mut results);
    jscpd_check::check_content_import_restriction(fs, &crawl.eslint_configs, path, &mut results);
    jscpd_check::check_velite_config(&crawl.velite_configs, &mut results);

    results
}
