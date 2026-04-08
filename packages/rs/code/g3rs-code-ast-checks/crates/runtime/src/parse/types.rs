#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ForbiddenMacroInfo {
    pub(crate) line: usize,
    pub(crate) macro_name: String,
    pub(crate) in_test_context: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ImplAllowInfo {
    pub(crate) line: usize,
    pub(crate) lint: String,
    pub(crate) kind: LintPolicyKind,
    pub(crate) method_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ForeignModAllowInfo {
    pub(crate) line: usize,
    pub(crate) lint: String,
    pub(crate) kind: LintPolicyKind,
    pub(crate) via_cfg_attr: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct IncludeMacroInfo {
    pub(crate) line: usize,
    pub(crate) macro_name: String,
    pub(crate) build_script_pattern: bool,
    pub(crate) path_traversal: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct GenericParameterCapInfo {
    pub(crate) line: usize,
    pub(crate) item_kind: &'static str,
    pub(crate) item_name: String,
    pub(crate) type_const_param_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct TestExpectCallInfo {
    pub(crate) line: usize,
    pub(crate) message: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct StringDispatchInfo {
    pub(crate) line: usize,
    pub(crate) site_kind: &'static str,
    pub(crate) string_literal_branch_count: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum LintPolicyKind {
    Allow,
    Expect,
}

impl LintPolicyKind {
    pub(crate) fn attr_name(self) -> &'static str {
        match self {
            Self::Allow => "allow",
            Self::Expect => "expect",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CfgPredicateTruth {
    KnownTrue,
    KnownFalse,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct LintPolicyInfo {
    pub(crate) line: usize,
    pub(crate) lint: String,
    pub(crate) kind: LintPolicyKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CfgAttrLintInfo {
    pub(crate) line: usize,
    pub(crate) lint: String,
    pub(crate) kind: LintPolicyKind,
    pub(crate) truth: CfgPredicateTruth,
}
