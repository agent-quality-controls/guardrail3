use std::collections::BTreeSet;

mod classify;
mod collect;
mod facts;
mod support;

pub(crate) use collect::{
    collect_ast_files, collect_components, collect_file_tree_components, collect_file_tree_files,
    collect_local_package_names,
};
pub(crate) use facts::{public_component_facts, public_file_tree_component_facts};

#[derive(Debug, Clone)]
pub(crate) struct OwnedTestComponent {
    pub(crate) rel_dir: String,
    pub(crate) runtime_rel_dir: String,
    pub(crate) runtime_cargo_rel_path: String,
    pub(crate) runtime_package_name: Option<String>,
    pub(crate) runtime_normal_dependencies: BTreeSet<String>,
    pub(crate) runtime_dev_dependencies: BTreeSet<String>,
    pub(crate) assertions_rel_dir: String,
    pub(crate) assertions_cargo_rel_path: String,
    pub(crate) assertions_exists: bool,
    pub(crate) nested_assertions_cargo_rel_path: Option<String>,
    pub(crate) assertions_package_name: Option<String>,
    pub(crate) assertions_dependencies: BTreeSet<String>,
    pub(crate) sidecars: Vec<g3rs_test_types::G3RsTestOwnedSidecarFacts>,
    pub(crate) external_harnesses: Vec<String>,
}
