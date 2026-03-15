use std::path::Path;

use crate::report::types::CheckResult;

use super::eslint_check;
use super::jscpd_check;
use super::npmrc_check;
use super::package_check;
use super::tsconfig_check;

pub fn check(path: &Path) -> Vec<CheckResult> {
    let mut results = Vec::new();

    eslint_check::check_eslint_config(path, &mut results);
    tsconfig_check::check_tsconfig(path, &mut results);
    npmrc_check::check_npmrc(path, &mut results);
    package_check::check_package_json(path, &mut results);
    jscpd_check::check_jscpd(path, &mut results);
    jscpd_check::check_content_import_restriction(path, &mut results);
    jscpd_check::check_velite_config(path, &mut results);

    results
}
