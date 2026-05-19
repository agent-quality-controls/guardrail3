//! Centralized process-spawn boundary for validate-command runtime.

use std::path::Path;
use std::process::Output;

/// Runs `git <args>` inside `cwd`.
#[expect(
    clippy::disallowed_methods,
    reason = "this module is the centralized process-spawn boundary for fixed-shape git invocations"
)]
pub(crate) fn run_git(args: &[&str], cwd: &Path) -> std::io::Result<Output> {
    std::process::Command::new("git")
        .args(args)
        .current_dir(cwd)
        .output()
}

/// Runs `/usr/bin/env which <command>`.
#[expect(
    clippy::disallowed_methods,
    reason = "this module is the centralized process-spawn boundary for fixed-shape PATH probes"
)]
pub(crate) fn run_env_which(command: &str) -> std::io::Result<Output> {
    std::process::Command::new("/usr/bin/env")
        .args(["which", command])
        .output()
}
