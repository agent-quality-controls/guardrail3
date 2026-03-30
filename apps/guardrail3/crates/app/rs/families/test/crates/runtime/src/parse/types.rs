use std::collections::BTreeSet;

#[derive(Debug, Clone, Default)]
pub struct ParsedTestFile {
    pub ignore_without_reason_lines: Vec<usize>,
    pub modules: Vec<ModuleInfo>,
    pub cfg_test_modules: Vec<CfgTestModuleInfo>,
    pub test_functions: Vec<TestFunctionInfo>,
    pub functions: Vec<FunctionInfo>,
    pub public_values: Vec<PublicValueInfo>,
    pub file_value_names: BTreeSet<String>,
    pub file_function_names: BTreeSet<String>,
    pub file_call_paths: Vec<Vec<String>>,
    pub imports: Vec<UseBinding>,
    pub macro_defined_proof_functions: BTreeSet<String>,
}

#[derive(Debug, Clone)]
pub struct ModuleInfo {
    pub line: usize,
    pub path_attr: Option<String>,
}

#[derive(Debug, Clone)]
pub struct CfgTestModuleInfo {
    pub line: usize,
    pub name: String,
    pub has_body: bool,
    pub path_attr: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct TestFunctionInfo {
    pub line: usize,
    pub name: String,
    pub uses_tokio_test_attr: bool,
    pub has_assertion_macro: bool,
    pub has_failure_enforcement: bool,
    pub call_paths: Vec<Vec<String>>,
    pub path_uses: Vec<Vec<String>>,
    pub method_receiver_paths: Vec<Vec<String>>,
    pub field_accesses: Vec<FieldAccessInfo>,
    pub string_literals: Vec<String>,
    pub shadowed_idents: BTreeSet<String>,
    pub should_panic_line: Option<usize>,
    pub should_panic_has_expected: bool,
    pub tautological_assert_lines: Vec<usize>,
    pub weak_matches_lines: Vec<usize>,
}

#[derive(Debug, Clone, Default)]
pub struct FunctionInfo {
    pub line: usize,
    pub name: String,
    pub is_public: bool,
    pub is_test: bool,
    pub arg_count: usize,
    pub arg_names: BTreeSet<String>,
    pub has_check_result_arg: bool,
    pub return_kind: ReturnKind,
    pub has_assertion_macro: bool,
    pub has_failure_enforcement: bool,
    pub call_paths: Vec<Vec<String>>,
    pub path_uses: Vec<Vec<String>>,
    pub field_accesses: Vec<FieldAccessInfo>,
    pub string_literals: Vec<String>,
    pub shadowed_idents: BTreeSet<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReturnKind {
    None,
    Other,
    StringLike,
    PathLike,
}

impl Default for ReturnKind {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct FieldAccessInfo {
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct UseBinding {
    pub line: usize,
    pub path_segments: Vec<String>,
    pub local_name: Option<String>,
}
