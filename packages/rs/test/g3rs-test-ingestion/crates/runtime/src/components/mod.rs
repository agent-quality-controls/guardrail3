use std::collections::BTreeSet;

/// `classify` module.
mod classify;
/// `collect` module.
mod collect;
/// `facts` module.
mod facts;
/// `support` module.
mod support;

pub(crate) use collect::{
    collect_ast_files, collect_components, collect_file_tree_components, collect_file_tree_files,
    collect_local_package_names,
};
pub(crate) use facts::{public_component_facts, public_file_tree_component_facts};

/// `OwnedTestComponent` struct.
#[derive(Debug, Clone)]
pub(crate) struct OwnedTestComponent {
    /// `rel_dir` item.
    pub(crate) rel_dir: String,
    /// `runtime_rel_dir` item.
    pub(crate) runtime_rel_dir: String,
    /// `runtime_cargo_rel_path` item.
    pub(crate) runtime_cargo_rel_path: String,
    /// `runtime_package_name` item.
    pub(crate) runtime_package_name: Option<String>,
    /// `runtime_normal_dependencies` item.
    pub(crate) runtime_normal_dependencies: BTreeSet<String>,
    /// `runtime_dev_dependencies` item.
    pub(crate) runtime_dev_dependencies: BTreeSet<String>,
    /// `assertions_rel_dir` item.
    pub(crate) assertions_rel_dir: String,
    /// `assertions_cargo_rel_path` item.
    pub(crate) assertions_cargo_rel_path: String,
    /// `assertions_exists` item.
    pub(crate) assertions_exists: bool,
    /// `nested_assertions_cargo_rel_path` item.
    pub(crate) nested_assertions_cargo_rel_path: Option<String>,
    /// `assertions_package_name` item.
    pub(crate) assertions_package_name: Option<String>,
    /// `assertions_dependencies` item.
    pub(crate) assertions_dependencies: BTreeSet<String>,
    /// `sidecars` item.
    pub(crate) sidecars: Vec<g3rs_test_types::G3RsTestOwnedSidecarFacts>,
    /// `external_harnesses` item.
    pub(crate) external_harnesses: Vec<String>,
}
