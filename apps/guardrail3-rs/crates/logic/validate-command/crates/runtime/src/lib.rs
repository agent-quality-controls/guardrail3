/// Cargo gate execution: replicates the verifier's command sequence inside the binary.
mod cargo_gates;
/// Command execution flow for validate requests.
mod execute;
/// Per-workspace family opt-out via `guardrail3-rs.toml`.
mod family_opt_out;
/// Centralized filesystem boundary.
mod fs;
/// Repo-wide marker-pair completeness check.
mod marker_pairs;
/// Final CLI outcome payload.
mod outcome;
/// Family-selection helpers shared by command execution.
mod selection;
/// Staged-file collection from git.
mod staged;

#[cfg(feature = "api")]
pub use cargo_gates::{
    CargoGateOutcome, any_rust_relevant, any_rust_source, cargo_gate_commands,
    is_rust_relevant_path, paths_under_workspace, run_cargo_gates,
};
#[cfg(feature = "api")]
pub use execute::{execute, execute_repo};
#[cfg(feature = "api")]
pub use family_opt_out::disabled_families;
#[cfg(feature = "api")]
pub use marker_pairs::check_repo as check_marker_pairs;
#[cfg(feature = "api")]
pub use outcome::ExecutionOutcome;
#[cfg(feature = "api")]
pub use selection::{
    REPO_LEVEL_FAMILIES, family_cli_name, selected_families, selected_families_with_opt_out,
};
#[cfg(feature = "api")]
pub use staged::{read_staged_files, resolve_repo_root};
