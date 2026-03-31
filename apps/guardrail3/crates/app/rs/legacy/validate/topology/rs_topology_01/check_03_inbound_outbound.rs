use std::path::Path;

use guardrail3_domain_report::CheckResult;
use guardrail3_outbound_traits::FileSystem;

use super::helpers;

/// Rule 3: `adapters/` and `ports/` must contain exactly `{inbound, outbound}`.
pub fn check(
    fs: &dyn FileSystem,
    name: &str,
    dir: &Path,
    label: &str,
    results: &mut Vec<CheckResult>,
) {
    let entries = fs.list_dir(dir);
    if entries.is_empty() {
        return; // missing dir already reported by rule 2
    }

    helpers::check_exact_subdirs(fs, name, dir, label, &["inbound", "outbound"], results);
}
