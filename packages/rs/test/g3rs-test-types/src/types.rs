use std::collections::BTreeSet;

use cargo_toml_parser::CargoToml;
use mutants_toml_parser::MutantsToml;
use nextest_toml_parser::NextestToml;

#[derive(Debug, Clone, PartialEq)]
pub struct G3RsTestConfigChecksInput {
    pub root_rel_dir: String,
    pub cargo_rel_path: String,
    pub mutants_rel_path: String,
    pub nextest_rel_path: String,
    pub cargo: CargoToml,
    pub nextest: Option<NextestToml>,
    pub mutants: Option<MutantsToml>,
    pub has_tests: bool,
    pub has_tokio_tests: bool,
    pub tokio_dependency_present: bool,
    pub cargo_mutants_installed: bool,
    pub mutation_hook_active: bool,
    pub mutation_hook_files: Vec<String>,
    pub mutants_exists: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3RsTestSourceChecksInput {
    pub root_rel_dir: String,
    pub cargo_rel_path: String,
    pub files: Vec<G3RsTestSourceFile>,
    pub components: Vec<G3RsTestComponentSourceFacts>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3RsTestSourceFile {
    pub rel_path: String,
    pub kind: G3RsTestFileKind,
    pub owner_module_name: Option<String>,
    pub component_rel_dir: Option<String>,
    pub assertions_package_name: Option<String>,
    pub content: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum G3RsTestFileKind {
    Source,
    InternalSidecarMod,
    InternalSidecarSupport,
    ExternalHarness,
    AssertionsModule,
    TestSupport,
    Other,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3RsTestComponentSourceFacts {
    pub rel_dir: String,
    pub runtime_rel_dir: String,
    pub runtime_package_name: Option<String>,
    pub assertions_rel_dir: String,
    pub assertions_exists: bool,
    pub assertions_package_name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3RsTestFileTreeChecksInput {
    pub root_rel_dir: String,
    pub cargo_rel_path: String,
    pub files: Vec<G3RsTestSourceFile>,
    pub components: Vec<G3RsTestComponentFileTreeFacts>,
    pub local_package_names: BTreeSet<String>,
    pub input_failures: Vec<G3RsTestFileTreeInputFailure>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3RsTestComponentFileTreeFacts {
    pub rel_dir: String,
    pub runtime_rel_dir: String,
    pub runtime_cargo_rel_path: String,
    pub runtime_package_name: Option<String>,
    pub runtime_normal_dependencies: BTreeSet<String>,
    pub runtime_dev_dependencies: BTreeSet<String>,
    pub assertions_rel_dir: String,
    pub assertions_cargo_rel_path: String,
    pub assertions_exists: bool,
    pub assertions_package_name: Option<String>,
    pub assertions_dependencies: BTreeSet<String>,
    pub sidecars: Vec<G3RsTestOwnedSidecarFacts>,
    pub external_harnesses: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3RsTestOwnedSidecarFacts {
    pub mod_rel_path: String,
    pub assertions_module_rel_path: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3RsTestFileTreeInputFailure {
    pub rel_path: String,
    pub message: String,
}
