//! CLI command dispatch glue. The validate / validate-repo pipeline lives
//! in dedicated sibling modules (`execute`, `marker_pairs`, `toolchain_gates`,
//! `tool_presence`, `topology`); this file holds the top-level entry points
//! and the shared `CliOutput` payload.

use guardrail3_ts_app_types::{FamilyRunner, ReportRenderer, WorkspaceCrawler};

pub(crate) use crate::cli::Command;
use crate::execute;
use crate::topology::CliFamilyRunner;

/// Final stdout, stderr, and exit code returned by one CLI command.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CliOutput {
    /// Text written to stdout.
    pub stdout: String,
    /// Text written to stderr.
    pub stderr: String,
    /// Process exit code.
    pub exit_code: i32,
}

/// Executes one parsed command against the injected runtime adapters.
pub fn run_command(
    command: Command,
    crawler: &dyn WorkspaceCrawler,
    family_runner: &dyn FamilyRunner,
    renderer: &dyn ReportRenderer,
) -> CliOutput {
    match command {
        Command::Validate {
            path,
            family,
            inventory,
            staged,
            rules_only,
        } => execute::run_validate(
            &path,
            &family,
            inventory,
            staged,
            rules_only,
            crawler,
            family_runner,
            renderer,
        ),
        Command::ValidateRepo { path } => {
            execute::run_validate_repo(path.as_deref(), crawler, family_runner, renderer)
        }
    }
}

/// Executes one parsed command through the app's default runtime wiring.
#[must_use]
pub fn run_command_with_defaults(command: Command) -> CliOutput {
    run_command(
        command,
        &crate::PackageRuntime,
        &CliFamilyRunner,
        &crate::PlainTextReportRenderer,
    )
}

#[cfg(test)]
#[path = "run_tests/mod.rs"] // reason: owned sidecar tests for run module.
mod run_tests;
