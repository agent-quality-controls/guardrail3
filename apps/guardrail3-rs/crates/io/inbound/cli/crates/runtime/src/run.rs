use g3_workspace_crawl::G3WorkspaceCrawl;
use guardrail3_rs_app_types::{
    FamilyResults, FamilyRunError, FamilyRunner, InitRepoRequest, InitWorkspaceRequest,
    ReportRenderer, SupportedFamily, ValidateRepoRequest, ValidateWorkspaceRequest,
    WorkspaceCrawler,
};
use guardrail3_rs_validate_command::{
    execute, execute_init_repo, execute_init_workspace, execute_repo, resolve_repo_root,
};

use crate::{Command, InitCommand, ValidateCommand};

/// CLI-local adapter that dispatches families into the bounded runner groups.
#[derive(Debug, Default)]
pub struct CliFamilyRunner;

impl FamilyRunner for CliFamilyRunner {
    fn run_family(
        &self,
        family: SupportedFamily,
        crawl: &G3WorkspaceCrawl,
    ) -> Result<FamilyResults, FamilyRunError> {
        match family {
            SupportedFamily::Toolchain | SupportedFamily::Fmt | SupportedFamily::Cargo => {
                guardrail3_rs_family_runner_style::run(family, crawl)
            }
            SupportedFamily::Clippy | SupportedFamily::Deny => {
                guardrail3_rs_family_runner_policy::run(family, crawl)
            }
            SupportedFamily::Code | SupportedFamily::Deps | SupportedFamily::Garde => {
                guardrail3_rs_family_runner_quality::run(family, crawl)
            }
            SupportedFamily::Hooks | SupportedFamily::Release => {
                guardrail3_rs_family_runner_process::run(family, crawl)
            }
            SupportedFamily::Test => guardrail3_rs_family_runner_test::run(family, crawl),
            SupportedFamily::Topology | SupportedFamily::Arch | SupportedFamily::Apparch => {
                guardrail3_rs_family_runner_structure::run(family, crawl)
            }
        }
    }
}

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
        Command::Init { command } => run_init(command, crawler, family_runner, renderer),
        Command::Validate { command } => run_validate(command, crawler, family_runner, renderer),
    }
}

/// Runs the selected init command and maps it into CLI output.
fn run_init(
    command: InitCommand,
    crawler: &dyn WorkspaceCrawler,
    family_runner: &dyn FamilyRunner,
    renderer: &dyn ReportRenderer,
) -> CliOutput {
    let outcome = match command {
        InitCommand::Repo { path, force } => execute_init_repo(
            &InitRepoRequest { path, force },
            crawler,
            family_runner,
            renderer,
        ),
        InitCommand::Workspace { path, force } => execute_init_workspace(
            &InitWorkspaceRequest {
                workspace_root: path,
                force,
            },
            crawler,
            family_runner,
            renderer,
        ),
    };
    CliOutput {
        stdout: outcome.stdout().to_owned(),
        stderr: outcome.stderr().to_owned(),
        exit_code: outcome.exit_code(),
    }
}

/// Runs the selected validate command and maps it into CLI output.
fn run_validate(
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
        } => {
            let request = ValidateWorkspaceRequest {
                workspace_root: path,
                families: family.into_iter().map(Into::into).collect(),
                include_inventory: inventory,
                staged,
                rules_only,
            };
            outcome_to_cli(execute(&request, crawler, family_runner, renderer))
        }
        ValidateCommand::Repo { path, inventory } => {
            let resolved_root = resolve_repo_root(&path);
            let request = ValidateRepoRequest {
                repo_root: resolved_root,
                include_inventory: inventory,
            };
            outcome_to_cli(execute_repo(&request, crawler, family_runner, renderer))
        }
    }
}

/// Converts command execution output into the CLI return type.
fn outcome_to_cli(
    outcome: Result<
        guardrail3_rs_validate_command::ExecutionOutcome,
        guardrail3_rs_app_types::WorkspaceCrawlError,
    >,
) -> CliOutput {
    match outcome {
        Ok(outcome) => CliOutput {
            stdout: outcome.stdout().to_owned(),
            stderr: outcome.stderr().to_owned(),
            exit_code: outcome.exit_code(),
        },
        Err(error) => CliOutput {
            stdout: String::new(),
            stderr: format!("{error}\n"),
            exit_code: 1,
        },
    }
}
