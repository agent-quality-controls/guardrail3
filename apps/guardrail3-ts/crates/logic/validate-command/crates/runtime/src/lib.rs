/// Command execution flow for validate requests.
mod execute;
/// Per-package family opt-out via `guardrail3-ts.toml`.
mod family_opt_out;
/// Centralized filesystem boundary for init and validate command code.
mod fs;
/// Command execution flow for init requests.
mod init;
/// Final CLI outcome payload.
mod outcome;
/// Family-selection helpers shared by command execution.
mod selection;

#[cfg(feature = "api")]
pub use execute::execute;
#[cfg(feature = "api")]
pub use family_opt_out::disabled_families;
#[cfg(feature = "api")]
pub use init::{execute_init_repo, execute_init_workspace};
#[cfg(feature = "api")]
pub use outcome::ExecutionOutcome;
#[cfg(feature = "api")]
pub use selection::{family_cli_name, selected_families, selected_families_with_opt_out};
