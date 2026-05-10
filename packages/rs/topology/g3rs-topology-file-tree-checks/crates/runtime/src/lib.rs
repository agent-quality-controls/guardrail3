//! File-tree checks for the g3rs topology family.

#[cfg(test)]
use cargo_toml_parser as _;
#[cfg(test)]
use g3rs_topology_file_tree_checks_assertions as _;

/// Rule: every Cargo workspace member must be declared in the root `Cargo.toml`.
mod declared_workspace_members_only;
/// Rule: declared workspace member paths must not escape the workspace root.
mod member_paths_must_not_escape_root;
/// Rule: no `guardrail3-rs.toml` may exist nested under another workspace root.
mod no_nested_guardrail3_rs_toml;
/// Rule: no Cargo workspaces may exist nested under another workspace root.
mod no_nested_workspaces;
/// Rule: required inputs must be present so the family fails closed when missing.
mod required_inputs_fail_closed;
/// Public dispatch entry point combining all file-tree rules.
mod run;
/// Support utilities shared between rules in this crate.
mod support;
/// Rule: workspace-local files must live at the documented placement paths.
mod workspace_local_file_placement;

#[cfg(feature = "checks")]
pub use run::check;
