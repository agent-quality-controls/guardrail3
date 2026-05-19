//! Centralized process-spawn boundary for the CLI runtime.
//!
//! Every other module in this crate must spawn external processes through
//! these helpers instead of calling `std::process::Command::new` directly.
//! This keeps shell-exec touchpoints in one auditable place.

use std::path::Path;
use std::process::Output;

/// Runs `git <args>` inside `cwd` and returns its `Output`. Returns the raw
/// `std::io::Error` when the spawn itself fails (binary missing, etc.).
#[expect(
    clippy::disallowed_methods,
    reason = "this module IS the centralized process-spawn boundary for the CLI runtime; argv is a fixed-shape git invocation, not user-supplied shell input"
)]
pub(crate) fn run_git(args: &[&str], cwd: &Path) -> std::io::Result<Output> {
    std::process::Command::new("git")
        .args(args)
        .current_dir(cwd)
        .output()
}
