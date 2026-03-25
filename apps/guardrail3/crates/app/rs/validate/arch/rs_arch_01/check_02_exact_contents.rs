use std::path::Path;

use crate::domain::report::CheckResult;
use guardrail3_outbound_traits::FileSystem;

use super::helpers;

/// Rule 2: `crates/` must contain exactly `{adapters, app, domain, ports}`.
/// No unexpected dirs, no loose files.
pub fn check(
    fs: &dyn FileSystem,
    name: &str,
    crates_dir: &Path,
    label_prefix: &str,
    results: &mut Vec<CheckResult>,
) {
    let expected = ["adapters", "app", "domain", "ports"];
    helpers::check_exact_subdirs(fs, name, crates_dir, label_prefix, &expected, results);
}
