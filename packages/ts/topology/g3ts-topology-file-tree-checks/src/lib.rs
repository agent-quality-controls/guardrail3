//! Facade crate re-exporting the g3ts topology file-tree checks API.

#[cfg(feature = "api")]
pub use g3ts_topology_file_tree_checks_runtime::check;
#[cfg(feature = "api")]
pub use g3ts_topology_file_tree_checks_types as types;
