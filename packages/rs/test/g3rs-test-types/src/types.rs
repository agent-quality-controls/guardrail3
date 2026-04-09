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
pub struct G3RsTestAstChecksInput {
    pub root_rel_dir: String,
    pub cargo_rel_path: String,
    pub files: Vec<G3RsTestSourceFile>,
    pub components: Vec<G3RsTestComponentAstFacts>,
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
    Other,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3RsTestComponentAstFacts {
    pub rel_dir: String,
    pub runtime_rel_dir: String,
    pub runtime_package_name: Option<String>,
    pub assertions_rel_dir: String,
    pub assertions_exists: bool,
    pub assertions_package_name: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct G3RsTestFileTreeChecksInput;
