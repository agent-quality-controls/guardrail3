use std::collections::BTreeSet;

#[derive(Debug, Clone)]
pub struct TestRootFacts {
    pub(crate) rel_dir: String,
    pub(crate) cargo_rel_path: String,
    pub(crate) mutants_rel_path: String,
    pub(crate) mutants_exists: bool,
    pub(crate) mutants_parsed: Option<toml::Value>,
    pub(crate) nextest_rel_path: String,
    pub(crate) nextest_exists: bool,
    pub(crate) nextest_parsed: Option<toml::Value>,
    pub(crate) tokio_dependency_present: bool,
    pub(crate) has_mutants_profile: bool,
    pub(crate) mutation_hook_files: Vec<String>,
    pub(crate) components: Vec<TestComponentFacts>,
}

#[derive(Debug, Clone)]
pub struct DiscoveredTestFile {
    pub(crate) rel_path: String,
    pub(crate) root_rel_dir: String,
    pub(crate) kind: TestFileKind,
    pub(crate) owner_module_name: Option<String>,
    pub(crate) component_rel_dir: Option<String>,
    pub(crate) assertions_package_name: Option<String>,
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
    pub(crate) root_rel_dir: String,
    pub(crate) rel_path: String,
    pub(crate) message: String,
}

#[derive(Debug, Clone)]
pub struct TestComponentFacts {
    pub(crate) rel_dir: String,
    pub(crate) runtime_rel_dir: String,
    pub(crate) runtime_cargo_rel_path: String,
    pub(crate) runtime_package_name: Option<String>,
    pub(crate) runtime_normal_dependencies: BTreeSet<String>,
    pub(crate) runtime_dev_dependencies: BTreeSet<String>,
    pub(crate) assertions_rel_dir: String,
    pub(crate) assertions_cargo_rel_path: String,
    pub(crate) assertions_exists: bool,
    pub(crate) assertions_package_name: Option<String>,
    pub(crate) assertions_dependencies: BTreeSet<String>,
    pub(crate) sidecars: Vec<SidecarFacts>,
    pub(crate) external_harnesses: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct SidecarFacts {
    pub(crate) mod_rel_path: String,
    pub(crate) assertions_module_rel_path: String,
}

#[derive(Debug, Clone)]
pub struct SidecarViolation {
    pub(crate) rel_path: String,
    pub(crate) line: Option<usize>,
    pub(crate) title: String,
    pub(crate) message: String,
}

#[derive(Debug, Clone)]
pub struct RuntimeAssertionsViolation {
    pub(crate) rel_path: String,
    pub(crate) line: Option<usize>,
    pub(crate) title: String,
    pub(crate) message: String,
}

#[derive(Debug, Clone)]
pub struct TestFacts {
    pub(crate) cargo_mutants_installed: bool,
    pub(crate) local_package_names: BTreeSet<String>,
    pub(crate) roots: Vec<TestRootFacts>,
    pub(crate) files: Vec<DiscoveredTestFile>,
    pub(crate) input_failures: Vec<InputFailureFacts>,
}
