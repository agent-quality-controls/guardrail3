use std::path::Path;

use crate::domain::report::{CheckResult, Severity};
use crate::ports::outbound::FileSystem;

/// Rule 12: `apps/{name}/src/` is banned — code must be in `crates/`.
pub fn check(
    fs: &dyn FileSystem,
    name: &str,
    app_dir: &Path,
    results: &mut Vec<CheckResult>,
) {
    let src_entries = fs.list_dir(&app_dir.join("src"));
    if !src_entries.is_empty() {
        results.push(CheckResult {
            id: "R-ARCH-01".to_owned(),
            severity: Severity::Error,
            title: format!("Service `{name}` has src/ directory"),
            message: format!(
                "Service `{name}` has an `src/` directory. Code must be in `crates/` \
                 following hex arch layout. Move code into \
                 `crates/{{adapters,app,domain,ports}}` subcrates."
            ),
            file: Some(app_dir.join("src").display().to_string()),
            line: None,
            inventory: false,
        });
    }
}
