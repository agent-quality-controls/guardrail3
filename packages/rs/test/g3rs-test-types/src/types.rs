use std::collections::BTreeMap;
use std::collections::BTreeSet;

use cargo_toml_parser::types::CargoToml;
use mutants_toml_parser::types::MutantsToml;
use nextest_toml_parser::types::NextestToml;

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
    pub files: Vec<G3RsTestAnalyzedSourceFile>,
    pub components: Vec<G3RsTestComponentSourceFacts>,
    pub input_failures: Vec<G3RsTestSourceInputFailure>,
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
pub struct G3RsTestAnalyzedSourceFile {
    pub rel_path: String,
    pub kind: G3RsTestFileKind,
    pub owner_module_name: Option<String>,
    pub component_rel_dir: Option<String>,
    pub assertions_package_name: Option<String>,
    pub parsed: ParsedTestFile,
    pub local_proof_helper_functions: BTreeSet<String>,
    pub proof_bearing_exported_functions: BTreeSet<String>,
    pub proof_bearing_assertion_functions: BTreeSet<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3RsTestSourceInputFailure {
    pub rel_path: String,
    pub message: String,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ParsedTestFile {
    pub ignore_reasons: Vec<IgnoreReasonInfo>,
    pub modules: Vec<ModuleInfo>,
    pub cfg_test_modules: Vec<CfgTestModuleInfo>,
    pub test_functions: Vec<TestFunctionInfo>,
    pub functions: Vec<FunctionInfo>,
    pub public_values: Vec<PublicValueInfo>,
    pub file_value_names: BTreeSet<String>,
    pub file_function_names: BTreeSet<String>,
    pub check_result_aliases: BTreeSet<String>,
    pub file_call_paths: Vec<Vec<String>>,
    pub imports: Vec<UseBinding>,
    pub macro_defined_proof_functions: BTreeSet<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModuleInfo {
    pub line: usize,
    pub path_attr: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CfgTestModuleInfo {
    pub line: usize,
    pub name: String,
    pub has_body: bool,
    pub path_attr: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TestFunctionInfo {
    pub line: usize,
    pub name: String,
    pub assertions: AssertionBodyInfo,
    pub body: FunctionBodyFacts,
    pub harness: TestHarnessFacts,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct FunctionInfo {
    pub line: usize,
    pub name: String,
    pub is_public: bool,
    pub is_test: bool,
    pub signature: FunctionSignatureInfo,
    pub assertions: AssertionBodyInfo,
    pub body: FunctionBodyFacts,
    pub string_literals: Vec<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct FunctionSignatureInfo {
    pub arg_count: usize,
    pub arg_names: BTreeSet<String>,
    pub has_check_result_arg: bool,
    pub return_kind: ReturnKind,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct AssertionBodyInfo {
    pub has_assertion_macro: bool,
    pub has_failure_enforcement: bool,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct FunctionBodyFacts {
    pub call_paths: Vec<Vec<String>>,
    pub path_uses: Vec<Vec<String>>,
    pub method_names: Vec<String>,
    pub local_call_aliases: BTreeMap<String, Vec<String>>,
    pub field_accesses: Vec<FieldAccessInfo>,
    pub shadowed_idents: BTreeSet<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TestHarnessFacts {
    pub uses_tokio_test_attr: bool,
    pub method_receiver_paths: Vec<Vec<String>>,
    pub should_panic_line: Option<usize>,
    pub should_panic_has_expected: bool,
    pub tautological_assert_lines: Vec<usize>,
    pub weak_matches_lines: Vec<usize>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ReturnKind {
    #[default]
    None,
    Other,
    StringLike,
    PathLike,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublicValueInfo {
    pub line: usize,
    pub name: String,
    pub kind: PublicValueKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PublicValueKind {
    Const,
    Static,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldAccessInfo {
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UseBinding {
    pub line: usize,
    pub path_segments: Vec<String>,
    pub local_name: Option<String>,
    pub is_public: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IgnoreReasonInfo {
    pub line: usize,
    pub reason: Option<String>,
}

pub mod ast {
    pub use super::{
        AssertionBodyInfo, CfgTestModuleInfo, FieldAccessInfo, FunctionBodyFacts, FunctionInfo,
        FunctionSignatureInfo, IgnoreReasonInfo, ModuleInfo, ParsedTestFile, PublicValueInfo,
        PublicValueKind, ReturnKind, TestFunctionInfo, TestHarnessFacts, UseBinding,
    };
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3RsTestFileTreeChecksInput {
    pub root_rel_dir: String,
    pub cargo_rel_path: String,
    pub files: Vec<G3RsTestAnalyzedSourceFile>,
    pub existing_file_paths: BTreeSet<String>,
    pub components: Vec<G3RsTestComponentFileTreeFacts>,
    pub has_tests: bool,
    pub local_package_names: BTreeSet<String>,
    pub local_runtime_packages: BTreeSet<String>,
    pub local_assertions_packages: BTreeSet<String>,
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
    pub nested_assertions_cargo_rel_path: Option<String>,
    pub assertions_package_name: Option<String>,
    pub assertions_dependencies: BTreeSet<String>,
    pub source_module_names: BTreeSet<String>,
    pub sidecars: Vec<G3RsTestOwnedSidecarFacts>,
    pub external_harnesses: Vec<String>,
    pub sidecar_files: Vec<G3RsTestAnalyzedSourceFile>,
    pub external_harness_files: Vec<G3RsTestAnalyzedSourceFile>,
    pub assertions_module_files: Vec<G3RsTestAnalyzedSourceFile>,
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
