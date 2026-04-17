use g3rs_topology_file_tree_checks_runtime as _;

mod common;

#[cfg(feature = "checks")]
pub mod rs_topology_07_required_inputs_fail_closed;
#[cfg(feature = "checks")]
pub mod rs_topology_11_no_nested_workspaces;
#[cfg(feature = "checks")]
pub mod rs_topology_12_declared_workspace_members_only;
#[cfg(feature = "checks")]
pub mod rs_topology_13_member_paths_must_not_escape_root;
#[cfg(feature = "checks")]
pub mod rs_topology_16_workspace_local_file_placement;
