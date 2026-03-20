use std::path::Path;

use crate::domain::report::{CheckResult, Severity};
use crate::ports::outbound::FileSystem;

/// Rule 12: `apps/{name}/src/` is banned — code must be in `crates/`.
/// Uses metadata() to detect src/ existence (catches empty dirs too).
/// Only fires for directories, not files named `src`.
pub fn check(
    fs: &dyn FileSystem,
    name: &str,
    app_dir: &Path,
    results: &mut Vec<CheckResult>,
) {
    let src_dir = app_dir.join("src");
    // metadata() detects existence of both empty and non-empty dirs.
    // list_dir() on the result distinguishes dir from file — if it's a file,
    // list_dir returns empty. But metadata on a file also returns Some.
    // We need to check it's actually a directory. Use list_dir first;
    // if empty, fall back to metadata + check the path is a dir on disk.
    let src_entries = fs.list_dir(&src_dir);
    let is_dir = !src_entries.is_empty() || src_dir.is_dir();
    if is_dir {
        results.push(CheckResult {
            id: "R-ARCH-01".to_owned(),
            severity: Severity::Error,
            title: format!("Service `{name}` has src/ directory"),
            message: format!(
                "Service `{name}` has an `src/` directory. Code must be in `crates/` \
                 following hex arch layout. Move code into \
                 `crates/{{adapters,app,domain,ports}}` subcrates."
            ),
            file: Some(src_dir.display().to_string()),
            line: None,
            inventory: false,
        });
    }
}
