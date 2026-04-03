mod analysis_helpers;
mod attrs;
mod comments;
mod core;
mod fs_visitors;
mod helpers;
mod types;
mod visitors;

pub type GardeSkipInfo = guardrail3_app_rs_ast::ast_helpers::GardeSkipInfo;
pub type InlineModAllow = guardrail3_app_rs_ast::ast_helpers::InlineModAllow;

pub type CfgAttrLintInfo = types::CfgAttrLintInfo;
pub type CfgPredicateTruth = types::CfgPredicateTruth;
pub type DenyForbidInfo = types::DenyForbidInfo;
#[allow(dead_code)] // reason: CODE-27 removed (redundant with ARCH-02) but type retained for potential future use
pub type FacadeBodyItemInfo = types::FacadeBodyItemInfo;
pub type ForbiddenMacroInfo = types::ForbiddenMacroInfo;
pub type ForeignModAllowInfo = types::ForeignModAllowInfo;
pub type GenericParameterCapInfo = types::GenericParameterCapInfo;
pub type ImplAllowInfo = types::ImplAllowInfo;
pub type IncludeMacroInfo = types::IncludeMacroInfo;
pub type LargeTypeItem = types::LargeTypeItem;
pub type LintPolicyInfo = types::LintPolicyInfo;
// PathAttrInfo removed: RS-CODE-24 moved to RS-ARCH-09.
pub type PublicStructFieldBagInfo = types::PublicStructFieldBagInfo;
pub type PublicResultErrorInfo = types::PublicResultErrorInfo;
pub type PublicResultErrorKind = types::PublicResultErrorKind;
pub type StringDispatchInfo = types::StringDispatchInfo;
pub type TestExpectCallInfo = types::TestExpectCallInfo;
pub type TraitMethodCountInfo = types::TraitMethodCountInfo;

pub fn parse_rust_file(content: &str) -> Result<syn::File, syn::Error> {
    core::parse_rust_file(content)
}

pub fn effective_non_comment_line_count(content: &str) -> usize {
    comments::effective_non_comment_line_count(content)
}

pub fn count_top_level_use_imports(ast: &syn::File) -> usize {
    core::count_top_level_use_imports(ast)
}

pub fn line_text(content: &str, line: usize) -> &str {
    comments::line_text(content, line)
}

pub fn same_line_reason(content: &str, line: usize) -> Option<String> {
    comments::same_line_reason(content, line)
}

pub fn same_line_has_comment(content: &str, line: usize) -> bool {
    comments::same_line_has_comment(content, line)
}

pub fn find_crate_level_allows(ast: &syn::File) -> Vec<(usize, String)> {
    guardrail3_app_rs_ast::ast_helpers::find_crate_level_allows(ast)
}

pub fn find_inline_mod_allows(ast: &syn::File) -> Vec<InlineModAllow> {
    guardrail3_app_rs_ast::ast_helpers::find_inline_mod_allows(ast)
}

pub fn find_garde_skips_with_types(ast: &syn::File) -> Vec<GardeSkipInfo> {
    guardrail3_app_rs_ast::ast_helpers::find_garde_skips_with_types(ast)
}

pub fn find_item_lint_policies(ast: &syn::File) -> Vec<LintPolicyInfo> {
    attrs::find_item_lint_policies(ast)
}

pub fn find_impl_block_allows(ast: &syn::File) -> Vec<ImplAllowInfo> {
    attrs::find_impl_block_allows(ast)
}

pub fn find_deny_forbid_attrs(ast: &syn::File) -> Vec<DenyForbidInfo> {
    attrs::find_deny_forbid_attrs(ast)
}

pub fn find_foreign_mod_allows(ast: &syn::File) -> Vec<ForeignModAllowInfo> {
    attrs::find_foreign_mod_allows(ast)
}

pub fn find_include_macros(ast: &syn::File) -> Vec<IncludeMacroInfo> {
    attrs::find_include_macros(ast)
}

pub fn find_cfg_attr_lint_policies(ast: &syn::File) -> Vec<CfgAttrLintInfo> {
    attrs::find_cfg_attr_lint_policies(ast)
}

// find_path_attrs and path_string_has_parent_segment removed: RS-CODE-24 moved to RS-ARCH-09.

pub fn find_public_result_error_types(ast: &syn::File) -> Vec<PublicResultErrorInfo> {
    attrs::find_public_result_error_types(ast)
}

pub fn find_public_struct_field_bags(ast: &syn::File) -> Vec<PublicStructFieldBagInfo> {
    attrs::find_public_struct_field_bags(ast)
}

pub fn find_forbidden_macros(ast: &syn::File, file_is_test_root: bool) -> Vec<ForbiddenMacroInfo> {
    visitors::find_forbidden_macros(ast, file_is_test_root)
}

pub fn find_test_expect_calls(ast: &syn::File, file_is_test_root: bool) -> Vec<TestExpectCallInfo> {
    visitors::find_test_expect_calls(ast, file_is_test_root)
}

pub fn find_std_fs_import_lines(ast: &syn::File) -> Vec<usize> {
    fs_visitors::find_std_fs_import_lines(ast)
}

pub fn find_inline_std_fs_call_lines(ast: &syn::File) -> Vec<usize> {
    fs_visitors::find_inline_std_fs_call_lines(ast)
}

pub fn find_std_fs_glob_import_lines(ast: &syn::File) -> Vec<usize> {
    fs_visitors::find_std_fs_glob_import_lines(ast)
}

#[allow(dead_code)] // reason: CODE-26 removed (redundant with ARCH-02)
pub fn find_pub_use_glob_reexports(ast: &syn::File) -> Vec<(usize, String)> {
    visitors::find_pub_use_glob_reexports(ast)
}

#[allow(dead_code)] // reason: CODE-27 removed (redundant with ARCH-02)
pub fn find_facade_body_items(ast: &syn::File) -> Vec<FacadeBodyItemInfo> {
    visitors::find_facade_body_items(ast)
}

pub fn find_large_type_items(ast: &syn::File) -> Vec<LargeTypeItem> {
    visitors::find_large_type_items(ast)
}

pub fn find_large_traits(ast: &syn::File) -> Vec<TraitMethodCountInfo> {
    visitors::find_large_traits(ast)
}

pub fn find_generic_parameter_caps(ast: &syn::File) -> Vec<GenericParameterCapInfo> {
    visitors::find_generic_parameter_caps(ast)
}

pub fn find_string_dispatch_sites(
    ast: &syn::File,
    file_is_test_root: bool,
) -> Vec<StringDispatchInfo> {
    visitors::find_string_dispatch_sites(ast, file_is_test_root)
}
