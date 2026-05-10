use std::path::PathBuf;

use crate::SupportedFamily;

/// Full validated input for one CLI validate command.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidateRequest {
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
