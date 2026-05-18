//! CLI command dispatch glue. The validate repo / validate workspace pipeline lives
//! in dedicated sibling modules (`execute`, `marker_pairs`, `toolchain_gates`,
//! `tool_presence`, `topology`); this file holds the top-level entry points
//! and the shared `CliOutput` payload.

use guardrail3_ts_app_types::{FamilyRunner, ReportRenderer, WorkspaceCrawler};

pub(crate) use crate::cli::Command;
use crate::cli::{InitCommand, ValidateCommand};
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
        Command::Init { command } => run_init_command(command),
        Command::Validate { command } => {
            run_validate_command(command, crawler, family_runner, renderer)
        }
    }
}

/// Runs the selected init subcommand and maps it into CLI output.
fn run_init_command(command: InitCommand) -> CliOutput {
    let output = match command {
        InitCommand::Repo { path, force } => guardrail3_ts_validate_command::execute_init_repo(
            &guardrail3_ts_app_types::InitRepoRequest { path, force },
        ),
        InitCommand::Workspace { path, force } => {
            guardrail3_ts_validate_command::execute_init_workspace(
                &guardrail3_ts_app_types::InitWorkspaceRequest {
                    workspace_root: path,
                    force,
                },
            )
        }
    };
    CliOutput {
        stdout: output.stdout().to_owned(),
        stderr: output.stderr().to_owned(),
        exit_code: output.exit_code(),
    }
}

/// Runs the selected validate subcommand and maps it into CLI output.
fn run_validate_command(
    command: ValidateCommand,
    crawler: &dyn WorkspaceCrawler,
    family_runner: &dyn FamilyRunner,
    renderer: &dyn ReportRenderer,
) -> CliOutput {
    match command {
        ValidateCommand::Workspace {
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
        ValidateCommand::Repo { path } => {
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
