//! Real tool checker adapter implementing the `ToolChecker` port.

use std::path::Path;
use std::process::Command;

use guardrail3_outbound_traits::{CommandRunResult, ToolChecker};

/// Production tool checker that runs actual shell commands.
#[derive(Debug)]
pub struct RealToolChecker;

impl ToolChecker for RealToolChecker {
    #[allow(clippy::disallowed_methods)] // reason: adapter layer — this IS the centralized tool-checking implementation
    fn is_installed(&self, tool: &str) -> bool {
        Command::new("which")
            .arg(tool)
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    }

    #[allow(clippy::disallowed_methods)] // reason: adapter layer — runs cargo publish dry-run for release checks
    fn run_cargo_publish_dry_run_outcome(&self, path: &Path) -> Option<CommandRunResult> {
        let output = Command::new("cargo")
            .args(["publish", "--dry-run"])
            .current_dir(path)
            .output()
            .ok()?;
        Some(CommandRunResult {
            success: output.status.success(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
        })
    }
}
