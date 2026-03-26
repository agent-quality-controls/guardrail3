use std::collections::BTreeSet;

#[derive(Debug, Clone)]
pub struct TestRootFacts {
    pub rel_dir: String,
    pub cargo_rel_path: String,
    pub mutants_rel_path: String,
    pub mutants_exists: bool,
    pub mutants_parsed: Option<toml::Value>,
    pub nextest_rel_path: String,
    pub nextest_exists: bool,
    pub nextest_parsed: Option<toml::Value>,
    pub tokio_dependency_present: bool,
    pub has_mutants_profile: bool,
    pub mutation_hook_files: Vec<String>,
    pub components: Vec<TestComponentFacts>,
}

#[derive(Debug, Clone)]
pub struct DiscoveredTestFile {
    pub rel_path: String,
    pub root_rel_dir: String,
    pub kind: TestFileKind,
    pub owner_module_name: Option<String>,
    pub component_rel_dir: Option<String>,
    pub assertions_package_name: Option<String>,
}

#[derive(Debug, Clone)]
pub enum TestFileKind {
    Source,
    InternalSidecarMod,
    InternalSidecarSupport,
    ExternalHarness,
    AssertionsModule,
    Other,
}

#[derive(Debug, Clone)]
pub struct InputFailureFacts {
    pub root_rel_dir: String,
    pub rel_path: String,
    pub message: String,
}

#[derive(Debug, Clone)]
pub struct TestComponentFacts {
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
    pub sidecars: Vec<SidecarFacts>,
    pub external_harnesses: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct SidecarFacts {
    pub mod_rel_path: String,
    pub assertions_module_rel_path: String,
}

#[derive(Debug, Clone)]
pub struct SidecarViolation {
    pub rel_path: String,
    pub line: Option<usize>,
    pub title: String,
    pub message: String,
}

#[derive(Debug, Clone)]
pub struct RuntimeAssertionsViolation {
    pub rel_path: String,
    pub line: Option<usize>,
    pub title: String,
    pub message: String,
}

#[derive(Debug, Clone)]
pub struct TestFacts {
    pub cargo_mutants_installed: bool,
    pub local_package_names: BTreeSet<String>,
    pub roots: Vec<TestRootFacts>,
    pub files: Vec<DiscoveredTestFile>,
    pub input_failures: Vec<InputFailureFacts>,
}