//! Reusable assertion helpers for the topology file-tree checks crate.

use g3rs_topology_file_tree_checks_runtime as _;

/// Shared helpers for matching emitted findings against expected snapshots.
mod common;
#[cfg(feature = "checks")]
pub mod run;

#[cfg(feature = "checks")]
pub mod declared_workspace_members_only;
#[cfg(feature = "checks")]
pub mod member_paths_must_not_escape_root;
#[cfg(feature = "checks")]
pub mod no_nested_guardrail3_rs_toml;
#[cfg(feature = "checks")]
pub mod no_nested_workspaces;
#[cfg(feature = "checks")]
pub mod required_inputs_fail_closed;
#[cfg(feature = "checks")]
pub mod workspace_local_file_placement;
