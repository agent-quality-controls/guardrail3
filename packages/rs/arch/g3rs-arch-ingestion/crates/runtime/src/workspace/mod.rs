//! Workspace traversal helpers shared by the source, file-tree, and config tiers.

/// Implementation of the workspace traversal primitives re-exported from this module.
mod run;

pub(crate) use run::{
    collect_crate_nodes, collect_dirs_recursive, collect_rs_files_recursive, is_inside,
    is_test_or_example_path, is_under_crate_src, normalize_path, parent_of,
    should_stop_at_nested_crate,
};
