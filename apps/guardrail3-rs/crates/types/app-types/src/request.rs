use std::path::PathBuf;

use crate::SupportedFamily;

/// Top-level command payload for one app command.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AppCommand {
    /// Initialize repo-level or workspace-level setup.
    Init(InitCommand),
    /// Validate repo-level or workspace-level state.
    Validate(ValidateCommand),
}

/// Init command payload.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InitCommand {
    /// Repo-level hook bootstrap.
    Repo(InitRepoRequest),
    /// Workspace-level Rust policy bootstrap.
    Workspace(InitWorkspaceRequest),
}

/// Validate command payload.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValidateCommand {
    /// Repo-level validation.
    Repo(ValidateRepoRequest),
    /// Workspace-level validation.
    Workspace(ValidateWorkspaceRequest),
}

/// Full validated input for one repo init command.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InitRepoRequest {
    /// Path inside the repository to initialize.
    pub path: PathBuf,
    /// Whether bounded generated rewrites and managed-block insertion are allowed.
    pub force: bool,
}

/// Full validated input for one workspace init command.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InitWorkspaceRequest {
    /// Rust workspace or package root to initialize.
    pub workspace_root: PathBuf,
    /// Whether bounded generated rewrites are allowed.
    pub force: bool,
}

/// Full validated input for one CLI validate command.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidateWorkspaceRequest {
    /// Workspace root requested by the user.
    pub workspace_root: PathBuf,
    /// Families selected for this run.
    pub families: Vec<SupportedFamily>,
    /// Whether inventory findings should be included in the output.
    pub include_inventory: bool,
    /// When true, filter cargo gates by staged files; skip if no Rust-relevant staged paths inside the workspace root.
    pub staged: bool,
    /// When true, skip cargo gates entirely; only run static rule families.
    pub rules_only: bool,
}

/// Full validated input for one repo-level validate command.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidateRepoRequest {
    /// Repository root.
    pub repo_root: PathBuf,
    /// Whether inventory findings should be included in the output.
    pub include_inventory: bool,
}
