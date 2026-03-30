#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LargeTypeItem {
    Struct {
        line: usize,
        name: String,
        field_count: usize,
    },
    Enum {
        line: usize,
        name: String,
        variant_count: usize,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublicStructFieldBagInfo {
    pub(crate) line: usize,
    pub(crate) struct_name: String,
    pub(crate) public_field_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImplAllowInfo {
    pub(crate) line: usize,
    pub(crate) lint: String,
    pub(crate) kind: LintPolicyKind,
    pub(crate) method_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DenyForbidInfo {
    pub(crate) line: usize,
    pub(crate) lint: String,
    pub(crate) level: String,
    pub(crate) crate_level_inner: bool,
    pub(crate) cfg_truth: CfgPredicateTruth,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IncludeMacroInfo {
    pub(crate) line: usize,
    pub(crate) macro_name: String,
    pub(crate) build_script_pattern: bool,
    pub(crate) path_traversal: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PathAttrInfo {
    pub(crate) line: usize,
    pub(crate) path: String,
    pub(crate) via_cfg_attr: bool,
    pub(crate) cfg_truth: CfgPredicateTruth,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PublicResultErrorKind {
    String,
    StrRef,
    AnyhowError,
    BoxDynError,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublicResultErrorInfo {
    pub(crate) line: usize,
    pub(crate) fn_name: String,
    pub(crate) kind: PublicResultErrorKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GenericParameterCapInfo {
    pub(crate) line: usize,
    pub(crate) item_kind: &'static str,
    pub(crate) item_name: String,
    pub(crate) type_const_param_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FacadeBodyItemInfo {
    pub(crate) line: usize,
    pub(crate) kind: &'static str,
    pub(crate) name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TraitMethodCountInfo {
    pub(crate) line: usize,
    pub(crate) trait_name: String,
    pub(crate) method_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StringDispatchInfo {
    pub(crate) line: usize,
    pub(crate) site_kind: &'static str,
    pub(crate) string_literal_branch_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForeignModAllowInfo {
    pub(crate) line: usize,
    pub(crate) lint: String,
    pub(crate) kind: LintPolicyKind,
    pub(crate) via_cfg_attr: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TestExpectCallInfo {
    pub(crate) line: usize,
    pub(crate) message: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LintPolicyKind {
    Allow,
    Expect,
}

impl LintPolicyKind {
    pub fn attr_name(self) -> &'static str {
        match self {
            Self::Allow => "allow",
            Self::Expect => "expect",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CfgPredicateTruth {
    KnownTrue,
    KnownFalse,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LintPolicyInfo {
    pub(crate) line: usize,
    pub(crate) lint: String,
    pub(crate) kind: LintPolicyKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CfgAttrLintInfo {
    pub(crate) line: usize,
    pub(crate) lint: String,
    pub(crate) kind: LintPolicyKind,
    pub(crate) truth: CfgPredicateTruth,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForbiddenMacroInfo {
    pub(crate) line: usize,
    pub(crate) macro_name: String,
    pub(crate) in_test_context: bool,
}
