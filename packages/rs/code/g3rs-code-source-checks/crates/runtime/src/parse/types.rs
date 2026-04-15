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

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum LargeTypeItem {
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
pub(crate) struct PublicStructFieldBagInfo {
    pub(crate) line: usize,
    pub(crate) struct_name: String,
    pub(crate) public_field_count: usize,
    pub(crate) all_fields_public: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum PublicResultErrorKind {
    String,
    StrRef,
    AnyhowError,
    BoxDynError,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PublicResultErrorInfo {
    pub(crate) line: usize,
    pub(crate) fn_name: String,
    pub(crate) kind: PublicResultErrorKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct TraitMethodCountInfo {
    pub(crate) line: usize,
    pub(crate) trait_name: String,
    pub(crate) method_count: usize,
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PathAttrInfo {
    pub(crate) line: usize,
    pub(crate) module_name: String,
    pub(crate) path_value: String,
    pub(crate) via_cfg_attr: bool,
    pub(crate) cfg_truth: CfgPredicateTruth,
    pub(crate) is_test_sidecar_exempt: bool,
    pub(crate) escapes_parent: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct DenyForbidInfo {
    pub(crate) line: usize,
    pub(crate) lint: String,
    pub(crate) level: String,
    pub(crate) crate_level_inner: bool,
    pub(crate) cfg_truth: CfgPredicateTruth,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct InlineModAllow {
    pub(crate) line: usize,
    pub(crate) lint: String,
    pub(crate) module_path: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct GardeSkipInfo {
    pub(crate) line: usize,
    pub(crate) field_name: String,
    pub(crate) field_type: String,
    pub(crate) is_type_level: bool,
    pub(crate) is_exempt: bool,
    pub(crate) has_subcommand_attr: bool,
}
