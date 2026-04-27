#[cfg(test)]
use cargo_toml_parser as _;
#[cfg(test)]
use g3rs_topology_file_tree_checks_assertions as _;

mod declared_workspace_members_only;
mod member_paths_must_not_escape_root;
mod no_nested_workspaces;
mod required_inputs_fail_closed;
mod run;
mod support;
mod workspace_local_file_placement;

#[cfg(feature = "checks")]
pub use run::check;
