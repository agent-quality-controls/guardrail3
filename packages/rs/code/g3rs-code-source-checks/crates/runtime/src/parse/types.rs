#[derive(Debug, Clone, PartialEq, Eq)]
/// Struct `ForbiddenMacroInfo` used by this module.
pub(crate) struct ForbiddenMacroInfo {
    /// Field `line`.
    pub(crate) line: usize,
    /// Field `macro_name`.
    pub(crate) macro_name: String,
    /// Field `in_test_context`.
    pub(crate) in_test_context: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Struct `ImplAllowInfo` used by this module.
pub(crate) struct ImplAllowInfo {
    /// Field `line`.
    pub(crate) line: usize,
    /// Field `lint`.
    pub(crate) lint: String,
    /// Field `kind`.
    pub(crate) kind: LintPolicyKind,
    /// Field `method_count`.
    pub(crate) method_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Struct `ForeignModAllowInfo` used by this module.
pub(crate) struct ForeignModAllowInfo {
    /// Field `line`.
    pub(crate) line: usize,
    /// Field `lint`.
    pub(crate) lint: String,
    /// Field `kind`.
    pub(crate) kind: LintPolicyKind,
    /// Field `via_cfg_attr`.
    pub(crate) via_cfg_attr: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Struct `IncludeMacroInfo` used by this module.
pub(crate) struct IncludeMacroInfo {
    /// Field `line`.
    pub(crate) line: usize,
    /// Field `macro_name`.
    pub(crate) macro_name: String,
    /// Field `build_script_pattern`.
    pub(crate) build_script_pattern: bool,
    /// Field `path_traversal`.
    pub(crate) path_traversal: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Struct `GenericParameterCapInfo` used by this module.
pub(crate) struct GenericParameterCapInfo {
    /// Field `line`.
    pub(crate) line: usize,
    /// Field `item_kind`.
    pub(crate) item_kind: &'static str,
    /// Field `item_name`.
    pub(crate) item_name: String,
    /// Field `type_const_param_count`.
    pub(crate) type_const_param_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Struct `TestExpectCallInfo` used by this module.
pub(crate) struct TestExpectCallInfo {
    /// Field `line`.
    pub(crate) line: usize,
    /// Field `message`.
    pub(crate) message: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Struct `StringDispatchInfo` used by this module.
pub(crate) struct StringDispatchInfo {
    /// Field `line`.
    pub(crate) line: usize,
    /// Field `site_kind`.
    pub(crate) site_kind: &'static str,
    /// Field `string_literal_branch_count`.
    pub(crate) string_literal_branch_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Enum `LargeTypeItem` used by this module.
pub(crate) enum LargeTypeItem {
    /// Variant `Struct`.
    Struct {
        /// Field `line`.
        line: usize,
        /// Field `name`.
        name: String,
        /// Field `field_count`.
        field_count: usize,
    },
    /// Variant `Enum`.
    Enum {
        /// Field `line`.
        line: usize,
        /// Field `name`.
        name: String,
        /// Field `variant_count`.
        variant_count: usize,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Struct `PublicStructFieldBagInfo` used by this module.
pub(crate) struct PublicStructFieldBagInfo {
    /// Field `line`.
    pub(crate) line: usize,
    /// Field `struct_name`.
    pub(crate) struct_name: String,
    /// Field `qualified_name`.
    pub(crate) qualified_name: String,
    /// Field `public_field_count`.
    pub(crate) public_field_count: usize,
    /// Field `all_fields_public`.
    pub(crate) all_fields_public: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Enum `PublicResultErrorKind` used by this module.
pub(crate) enum PublicResultErrorKind {
    /// Variant `String`.
    String,
    /// Variant `StrRef`.
    StrRef,
    /// Variant `AnyhowError`.
    AnyhowError,
    /// Variant `BoxDynError`.
    BoxDynError,
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Struct `PublicResultErrorInfo` used by this module.
pub(crate) struct PublicResultErrorInfo {
    /// Field `line`.
    pub(crate) line: usize,
    /// Field `fn_name`.
    pub(crate) fn_name: String,
    /// Field `kind`.
    pub(crate) kind: PublicResultErrorKind,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
/// Struct `AnyhowTypeBindings` used by this module.
pub(crate) struct AnyhowTypeBindings {
    /// Field `error_type_names`.
    pub(crate) error_type_names: BTreeSet<String>,
    /// Field `module_aliases`.
    pub(crate) module_aliases: BTreeSet<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Struct `TraitMethodCountInfo` used by this module.
pub(crate) struct TraitMethodCountInfo {
    /// Field `line`.
    pub(crate) line: usize,
    /// Field `trait_name`.
    pub(crate) trait_name: String,
    /// Field `method_count`.
    pub(crate) method_count: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Enum `LintPolicyKind` used by this module.
pub(crate) enum LintPolicyKind {
    /// Variant `Allow`.
    Allow,
    /// Variant `Expect`.
    Expect,
}

impl LintPolicyKind {
    /// Implements `attr name`.
    pub(crate) const fn attr_name(self) -> &'static str {
        match self {
            Self::Allow => "allow",
            Self::Expect => "expect",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Enum `CfgPredicateTruth` used by this module.
pub(crate) enum CfgPredicateTruth {
    /// Variant `KnownTrue`.
    KnownTrue,
    /// Variant `KnownFalse`.
    KnownFalse,
    /// Variant `Unknown`.
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Struct `LintPolicyInfo` used by this module.
pub(crate) struct LintPolicyInfo {
    /// Field `line`.
    pub(crate) line: usize,
    /// Field `lint`.
    pub(crate) lint: String,
    /// Field `kind`.
    pub(crate) kind: LintPolicyKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Struct `CfgAttrLintInfo` used by this module.
pub(crate) struct CfgAttrLintInfo {
    /// Field `line`.
    pub(crate) line: usize,
    /// Field `lint`.
    pub(crate) lint: String,
    /// Field `kind`.
    pub(crate) kind: LintPolicyKind,
    /// Field `truth`.
    pub(crate) truth: CfgPredicateTruth,
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Struct `PathAttrInfo` used by this module.
pub(crate) struct PathAttrInfo {
    /// Field `line`.
    pub(crate) line: usize,
    /// Field `module_name`.
    pub(crate) module_name: String,
    /// Field `path_value`.
    pub(crate) path_value: String,
    /// Field `via_cfg_attr`.
    pub(crate) via_cfg_attr: bool,
    /// Field `cfg_truth`.
    pub(crate) cfg_truth: CfgPredicateTruth,
    /// Field `is_test_sidecar_exempt`.
    pub(crate) is_test_sidecar_exempt: bool,
    /// Field `escapes_parent`.
    pub(crate) escapes_parent: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Struct `DenyForbidInfo` used by this module.
pub(crate) struct DenyForbidInfo {
    /// Field `line`.
    pub(crate) line: usize,
    /// Field `lint`.
    pub(crate) lint: String,
    /// Field `level`.
    pub(crate) level: String,
    /// Field `crate_level_inner`.
    pub(crate) crate_level_inner: bool,
    /// Field `cfg_truth`.
    pub(crate) cfg_truth: CfgPredicateTruth,
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Struct `InlineModAllow` used by this module.
pub(crate) struct InlineModAllow {
    /// Field `line`.
    pub(crate) line: usize,
    /// Field `lint`.
    pub(crate) lint: String,
    /// Field `module_path`.
    pub(crate) module_path: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Struct `GardeSkipInfo` used by this module.
pub(crate) struct GardeSkipInfo {
    /// Field `line`.
    pub(crate) line: usize,
    /// Field `field_name`.
    pub(crate) field_name: String,
    /// Field `field_type`.
    pub(crate) field_type: String,
    /// Field `is_type_level`.
    pub(crate) is_type_level: bool,
    /// Field `is_exempt`.
    pub(crate) is_exempt: bool,
    /// Field `has_subcommand_attr`.
    pub(crate) has_subcommand_attr: bool,
}
use std::collections::BTreeSet;
