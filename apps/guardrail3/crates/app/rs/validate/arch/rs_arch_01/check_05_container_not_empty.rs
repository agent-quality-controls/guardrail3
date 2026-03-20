use std::path::Path;

use crate::domain::report::CheckResult;
use crate::ports::outbound::FileSystem;

use super::helpers;

/// Rule 5: container dirs must have `.gitkeep` or at least one subdir.
/// Also checks for loose files when container has subdirs.
pub fn check(
    fs: &dyn FileSystem,
    name: &str,
    dir: &Path,
    label: &str,
    results: &mut Vec<CheckResult>,
) {
    helpers::check_container_not_empty(fs, name, dir, label, results);
}
