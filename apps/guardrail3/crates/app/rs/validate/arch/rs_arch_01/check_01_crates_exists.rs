use std::path::Path;

use guardrail3_domain_report::{CheckResult, Severity};
use guardrail3_outbound_traits::FileSystem;

/// Rule 1: `crates/` must exist.
/// Returns the crates dir path if it exists, None otherwise.
pub fn check(
    fs: &dyn FileSystem,
    name: &str,
    parent_dir: &Path,
    label_prefix: &str,
    results: &mut Vec<CheckResult>,
) -> bool {
    let crates_dir = parent_dir.join("crates");
    let crates_entries = fs.list_dir(&crates_dir);
    if crates_entries.is_empty() {
        results.push(CheckResult::from_parts(
    "R-ARCH-01".to_owned(),
    Severity::Error,
    format!("Service `{name}` missing {label_prefix}/ directory"),
    format!(
                "Service `{name}` has no `{label_prefix}/` directory. Create it with the hex arch \
                 template: `{label_prefix}/{{adapters/{{inbound,outbound}}, app, domain, \
                 ports/{{inbound,outbound}}}}`."
            ),
    Some(parent_dir.display().to_string()),
    None,
    false,
        ));
        return false;
    }
    true,
)
