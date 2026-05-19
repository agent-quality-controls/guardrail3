/// Command execution flow for validate requests.
mod execute;
/// Per-package family opt-out via `guardrail3-ts.toml`.
mod family_opt_out;
/// Centralized filesystem boundary for init and validate command code.
mod fs;
/// Command execution flow for init requests.
mod init;
/// Package manifest mutations for init workspace.
mod init_package_json;
/// Repo marker-pair checks for TypeScript adoption.
mod marker_pairs;
/// Final CLI outcome payload.
mod outcome;
/// Process-spawn boundary for validate-command runtime.
mod process;
/// Family-selection helpers shared by command execution.
mod selection;
/// Staged-file relevance filter for TypeScript workspace validation.
mod staged;
/// Repo required-tool checks.
mod tool_presence;
/// Toolchain-gate construction and execution after the static rule pipeline.
mod toolchain_gates;
/// Repo package-root adoption checks.
mod workspace_adoption;

#[cfg(feature = "api")]
pub use execute::{execute, execute_repo};
#[cfg(feature = "api")]
pub use family_opt_out::{GuardrailConfigError, disabled_families};
#[cfg(feature = "api")]
pub use init::{execute_init_repo, execute_init_workspace};
#[cfg(feature = "api")]
pub use outcome::ExecutionOutcome;
#[cfg(feature = "api")]
pub use selection::{
    PER_WORKSPACE_DEFAULT_FAMILIES, REPO_LEVEL_FAMILIES, family_cli_name, selected_families,
    selected_families_with_opt_out,
};
