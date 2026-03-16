use std::path::Path;

use crate::domain::report::CheckResult;

use super::eslint_check;
use super::jscpd_check;
use super::npmrc_check;
use super::package_check;
use super::tsconfig_check;
use crate::ports::outbound::FileSystem;
pub fn check(fs: &dyn FileSystem, path: &Path) -> Vec<CheckResult> {
    let mut results = Vec::new();

    eslint_check::check_eslint_config(fs, path, &mut results);
    tsconfig_check::check_tsconfig(fs, path, &mut results);
    npmrc_check::check_npmrc(fs, path, &mut results);
    package_check::check_package_json(fs, path, &mut results);
    jscpd_check::check_jscpd(fs, path, &mut results);
    jscpd_check::check_content_import_restriction(fs, path, &mut results);
    jscpd_check::check_velite_config(path, &mut results);

    results
}
