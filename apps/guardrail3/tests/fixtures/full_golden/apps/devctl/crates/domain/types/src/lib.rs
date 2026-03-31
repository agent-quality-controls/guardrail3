use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceConfig {
    pub root: String,
    pub rust_checks_enabled: bool,
    pub hook_mode: HookMode,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum HookMode {
    ValidateOnly,
    ValidateAndFix,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DoctorReport {
    pub workspace_root: String,
    pub missing_paths: Vec<String>,
    pub stale_generated_files: Vec<String>,
    pub hook_status: HookStatus,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum HookStatus {
    Installed,
    Missing,
    Stale,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum DevctlError {
    #[error("workspace root must not be empty")]
    MissingWorkspaceRoot,
}

impl WorkspaceConfig {
    pub fn new(root: impl Into<String>, hook_mode: HookMode) -> Result<Self, DevctlError> {
        let root = root.into();
        if root.trim().is_empty() {
            return Err(DevctlError::MissingWorkspaceRoot);
        }

        Ok(Self {
            root,
            rust_checks_enabled: true,
            hook_mode,
        })
    }
}
