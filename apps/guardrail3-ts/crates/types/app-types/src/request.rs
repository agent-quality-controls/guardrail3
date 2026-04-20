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
}
