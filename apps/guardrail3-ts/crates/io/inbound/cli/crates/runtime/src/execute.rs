//! CLI validate command adapter.

use std::path::Path;

use guardrail3_ts_app_types::{
    FamilyRunner, ReportRenderer, ValidateRepoRequest, ValidateWorkspaceRequest, WorkspaceCrawler,
};
use guardrail3_ts_validate_command::{execute, execute_repo};

use crate::cli::FamilyArg;
use crate::process::run_git;
use crate::run::CliOutput;

/// Runs the `validate workspace` subcommand.
#[expect(
    clippy::too_many_arguments,
    reason = "stable CLI surface threads path, family list, three flags, and three injected adapters"
)]
pub(crate) fn run_validate_workspace(
    path: &Path,
    family: &[FamilyArg],
    inventory: bool,
    staged: bool,
    rules_only: bool,
    crawler: &dyn WorkspaceCrawler,
    family_runner: &dyn FamilyRunner,
    renderer: &dyn ReportRenderer,
) -> CliOutput {
    let request = ValidateWorkspaceRequest {
        workspace_root: path.to_path_buf(),
        families: family.iter().copied().flat_map(FamilyArg::expand).collect(),
        include_inventory: inventory,
        staged,
        rules_only,
    };
    let outcome = match execute(&request, crawler, family_runner, renderer) {
        Ok(outcome) => outcome,
        Err(error) => {
            return CliOutput {
                stdout: String::new(),
                stderr: format!("{error}\n"),
                exit_code: 1,
            };
        }
    };

    CliOutput {
        stdout: outcome.stdout().to_owned(),
        stderr: outcome.stderr().to_owned(),
        exit_code: outcome.exit_code(),
    }
}

/// Runs the `validate repo` subcommand.
pub(crate) fn run_validate_repo(
    path: Option<&Path>,
    inventory: bool,
    crawler: &dyn WorkspaceCrawler,
    family_runner: &dyn FamilyRunner,
    renderer: &dyn ReportRenderer,
) -> CliOutput {
    let start = path.unwrap_or_else(|| Path::new("."));
    if !start.is_dir() {
        return CliOutput {
            stdout: String::new(),
            stderr: format!("path is not a directory: {}\n", start.display()),
            exit_code: 1,
        };
    }
    let Some(repo_root) = git_root(start) else {
        return CliOutput {
            stdout: String::new(),
            stderr: "validate repo: could not resolve git repo root\n".to_owned(),
            exit_code: 1,
        };
    };

    let request = ValidateRepoRequest {
        repo_root,
        include_inventory: inventory,
    };
    let outcome = match execute_repo(&request, crawler, family_runner, renderer) {
        Ok(outcome) => outcome,
        Err(error) => {
            return CliOutput {
                stdout: String::new(),
                stderr: format!("{error}\n"),
                exit_code: 1,
            };
        }
    };

    CliOutput {
        stdout: outcome.stdout().to_owned(),
        stderr: outcome.stderr().to_owned(),
        exit_code: outcome.exit_code(),
    }
}

/// Resolves the Git repository root from a user-supplied path.
fn git_root(start: &Path) -> Option<std::path::PathBuf> {
    let output = run_git(&["rev-parse", "--show-toplevel"], start).ok()?;
    if !output.status.success() {
        return None;
    }
    let text = String::from_utf8_lossy(&output.stdout).trim().to_owned();
    if text.is_empty() {
        return None;
    }
    Some(std::path::PathBuf::from(text))
}
