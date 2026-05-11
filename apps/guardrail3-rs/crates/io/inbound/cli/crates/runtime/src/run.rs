use g3_workspace_crawl::G3WorkspaceCrawl;
use guardrail3_rs_app_types::{
    FamilyResults, FamilyRunError, FamilyRunner, ReportRenderer, SupportedFamily,
    ValidateRepoRequest, ValidateRequest, WorkspaceCrawler,
};
use guardrail3_rs_validate_command::{execute, execute_repo, resolve_repo_root};

use crate::Command;

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
        Command::Validate {
            path,
            family,
            inventory,
            staged,
            rules_only,
        } => {
            let request = ValidateRequest {
                workspace_root: path,
                families: family.into_iter().map(Into::into).collect(),
                include_inventory: inventory,
                staged,
                rules_only,
            };
            match execute(&request, crawler, family_runner, renderer) {
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
        Command::ValidateRepo {
            repo_root,
            inventory,
        } => {
            let resolved_root =
                repo_root.unwrap_or_else(|| resolve_repo_root(&std::path::PathBuf::from(".")));
            let request = ValidateRepoRequest {
                repo_root: resolved_root,
                include_inventory: inventory,
            };
            match execute_repo(&request, crawler, family_runner, renderer) {
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
    }
}

#[cfg(test)]
#[path = "run_tests/mod.rs"] // reason: owned sidecar tests for file module.
mod run_tests;
