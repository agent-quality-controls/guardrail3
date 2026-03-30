use std::collections::BTreeSet;

#[derive(Debug, Clone, Default)]
pub struct ParsedTestFile {
    pub(crate) ignore_without_reason_lines: Vec<usize>,
    pub(crate) modules: Vec<ModuleInfo>,
    pub(crate) cfg_test_modules: Vec<CfgTestModuleInfo>,
    pub(crate) test_functions: Vec<TestFunctionInfo>,
    pub(crate) functions: Vec<FunctionInfo>,
    pub(crate) public_values: Vec<PublicValueInfo>,
    pub(crate) file_value_names: BTreeSet<String>,
    pub(crate) file_function_names: BTreeSet<String>,
    pub(crate) file_call_paths: Vec<Vec<String>>,
    pub(crate) imports: Vec<UseBinding>,
    pub(crate) macro_defined_proof_functions: BTreeSet<String>,
}

#[derive(Debug, Clone)]
pub struct ModuleInfo {
    pub(crate) line: usize,
    pub(crate) path_attr: Option<String>,
}

#[derive(Debug, Clone)]
pub struct CfgTestModuleInfo {
    pub(crate) line: usize,
    pub(crate) name: String,
    pub(crate) has_body: bool,
    pub(crate) path_attr: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct TestFunctionInfo {
    pub(crate) line: usize,
    pub(crate) name: String,
    pub(crate) uses_tokio_test_attr: bool,
    pub(crate) has_assertion_macro: bool,
    pub(crate) has_failure_enforcement: bool,
    pub(crate) call_paths: Vec<Vec<String>>,
    pub(crate) path_uses: Vec<Vec<String>>,
    pub(crate) method_receiver_paths: Vec<Vec<String>>,
    pub(crate) field_accesses: Vec<FieldAccessInfo>,
    pub(crate) string_literals: Vec<String>,
    pub(crate) shadowed_idents: BTreeSet<String>,
    pub(crate) should_panic_line: Option<usize>,
    pub(crate) should_panic_has_expected: bool,
    pub(crate) tautological_assert_lines: Vec<usize>,
    pub(crate) weak_matches_lines: Vec<usize>,
}

#[derive(Debug, Clone, Default)]
pub struct FunctionInfo {
    pub(crate) line: usize,
    pub(crate) name: String,
    pub(crate) is_public: bool,
    pub(crate) is_test: bool,
    pub(crate) arg_count: usize,
    pub(crate) arg_names: BTreeSet<String>,
    pub(crate) has_check_result_arg: bool,
    pub(crate) return_kind: ReturnKind,
    pub(crate) has_assertion_macro: bool,
    pub(crate) has_failure_enforcement: bool,
    pub(crate) call_paths: Vec<Vec<String>>,
    pub(crate) path_uses: Vec<Vec<String>>,
    pub(crate) field_accesses: Vec<FieldAccessInfo>,
    pub(crate) string_literals: Vec<String>,
    pub(crate) shadowed_idents: BTreeSet<String>,
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
    pub(crate) line: usize,
    pub(crate) name: String,
    pub(crate) kind: PublicValueKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PublicValueKind {
    Const,
    Static,
}

#[derive(Debug, Clone)]
pub struct FieldAccessInfo {
    pub(crate) name: String,
}

#[derive(Debug, Clone)]
pub struct UseBinding {
    pub(crate) line: usize,
    pub(crate) path_segments: Vec<String>,
    pub(crate) local_name: Option<String>,
}
