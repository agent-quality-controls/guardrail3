#[cfg(test)]
use cargo_toml_parser as _;
#[cfg(test)]
use g3rs_topology_file_tree_checks_assertions as _;

mod rs_topology_07_required_inputs_fail_closed;
mod rs_topology_11_no_nested_workspaces;
mod rs_topology_12_declared_workspace_members_only;
mod rs_topology_13_member_paths_must_not_escape_root;
mod rs_topology_16_workspace_local_file_placement;
mod run;
mod support;

#[cfg(feature = "checks")]
pub use run::check;
