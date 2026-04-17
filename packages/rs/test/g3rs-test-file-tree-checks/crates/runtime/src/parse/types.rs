#![allow(
    dead_code,
    reason = "parser preserves source facts for planned test rules"
)]

use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, Default)]
pub(crate) struct ParsedTestFile {
    pub(crate) ignore_reasons: Vec<IgnoreReasonInfo>,
    pub(crate) modules: Vec<ModuleInfo>,
    pub(crate) cfg_test_modules: Vec<CfgTestModuleInfo>,
    pub(crate) test_functions: Vec<TestFunctionInfo>,
    pub(crate) functions: Vec<FunctionInfo>,
    pub(crate) public_values: Vec<PublicValueInfo>,
    pub(crate) file_value_names: BTreeSet<String>,
    pub(crate) file_function_names: BTreeSet<String>,
    pub(crate) check_result_aliases: BTreeSet<String>,
    pub(crate) file_call_paths: Vec<Vec<String>>,
    pub(crate) imports: Vec<UseBinding>,
    pub(crate) macro_defined_proof_functions: BTreeSet<String>,
}

#[derive(Debug, Clone)]
pub(crate) struct ModuleInfo {
    pub(crate) line: usize,
    pub(crate) path_attr: Option<String>,
}

#[derive(Debug, Clone)]
pub(crate) struct CfgTestModuleInfo {
    pub(crate) line: usize,
    pub(crate) name: String,
    pub(crate) has_body: bool,
    pub(crate) path_attr: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct TestFunctionInfo {
    pub(crate) line: usize,
    pub(crate) name: String,
    pub(crate) assertions: AssertionBodyInfo,
    pub(crate) body: FunctionBodyFacts,
    pub(crate) harness: TestHarnessFacts,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct FunctionInfo {
    pub(crate) line: usize,
    pub(crate) name: String,
    pub(crate) is_public: bool,
    pub(crate) is_test: bool,
    pub(crate) signature: FunctionSignatureInfo,
    pub(crate) assertions: AssertionBodyInfo,
    pub(crate) body: FunctionBodyFacts,
    pub(crate) string_literals: Vec<String>,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct FunctionSignatureInfo {
    pub(crate) arg_count: usize,
    pub(crate) arg_names: BTreeSet<String>,
    pub(crate) has_check_result_arg: bool,
    pub(crate) return_kind: ReturnKind,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct AssertionBodyInfo {
    pub(crate) has_assertion_macro: bool,
    pub(crate) has_failure_enforcement: bool,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct FunctionBodyFacts {
    pub(crate) call_paths: Vec<Vec<String>>,
    pub(crate) path_uses: Vec<Vec<String>>,
    pub(crate) method_names: Vec<String>,
    pub(crate) local_call_aliases: BTreeMap<String, Vec<String>>,
    pub(crate) field_accesses: Vec<FieldAccessInfo>,
    pub(crate) shadowed_idents: BTreeSet<String>,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct TestHarnessFacts {
    pub(crate) uses_tokio_test_attr: bool,
    pub(crate) method_receiver_paths: Vec<Vec<String>>,
    pub(crate) should_panic_line: Option<usize>,
    pub(crate) should_panic_has_expected: bool,
    pub(crate) tautological_assert_lines: Vec<usize>,
    pub(crate) weak_matches_lines: Vec<usize>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ReturnKind {
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
pub(crate) struct PublicValueInfo {
    pub(crate) line: usize,
    pub(crate) name: String,
    pub(crate) kind: PublicValueKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PublicValueKind {
    Const,
    Static,
}

#[derive(Debug, Clone)]
pub(crate) struct FieldAccessInfo {
    pub(crate) name: String,
}

#[derive(Debug, Clone)]
pub(crate) struct UseBinding {
    pub(crate) line: usize,
    pub(crate) path_segments: Vec<String>,
    pub(crate) local_name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct IgnoreReasonInfo {
    pub(crate) line: usize,
    pub(crate) reason: Option<String>,
}
