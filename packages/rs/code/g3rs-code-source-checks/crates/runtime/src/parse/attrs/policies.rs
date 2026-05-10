#![allow(
    clippy::excessive_nesting,
    clippy::missing_docs_in_private_items,
    clippy::wildcard_enum_match_arm,
    clippy::match_wildcard_for_single_variants,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects,
    clippy::similar_names,
    clippy::too_many_lines,
    clippy::question_mark,
    clippy::case_sensitive_file_extension_comparisons,
    clippy::needless_pass_by_value,
    clippy::expect_used,
    clippy::option_if_let_else,
    clippy::map_unwrap_or,
    clippy::if_same_then_else,
    clippy::match_same_arms,
    clippy::match_like_matches_macro,
    clippy::nonminimal_bool,
    clippy::single_match_else,
    clippy::items_after_statements,
    clippy::collapsible_if,
    clippy::collapsible_match,
    clippy::needless_for_each,
    clippy::manual_let_else,
    clippy::redundant_else,
    clippy::shadow_unrelated,
    clippy::struct_excessive_bools,
    clippy::type_complexity,
    clippy::too_many_arguments,
    clippy::module_name_repetitions,
    clippy::large_enum_variant,
    clippy::large_types_passed_by_value,
    clippy::ptr_arg,
    clippy::needless_collect,
    clippy::branches_sharing_code,
    clippy::unused_self,
    reason = "code-source-checks parse/visitor walks every variant of large external syntax-tree enums (syn::Type, syn::Item, syn::Expr, syn::Pat, etc.) and the ban-detection visitors mirror the source structure they are looking for; the rule modules accept the schema-versioned shape verbatim because the per-rule findings depend on the exact spans and the rule ids embed the schema."
)]

use crate::parse::analysis_helpers;
use crate::parse::helpers;
use crate::parse::types::{
    CfgAttrLintInfo, CfgPredicateTruth, DenyForbidInfo, ForeignModAllowInfo, ImplAllowInfo,
    IncludeMacroInfo, InlineModAllow, LintPolicyInfo, PathAttrInfo,
};
use syn::spanned::Spanned;
use syn::visit::Visit;

/// Implements `find crate level allows`.
pub(crate) fn find_crate_level_allows(source: &syn::File) -> Vec<(usize, String)> {
    let mut out = Vec::new();
    for attr in &source.attrs {
        if matches!(attr.style, syn::AttrStyle::Inner(_)) && attr.path().is_ident("allow") {
            out.extend(
                helpers::collect_item_lint_policies(std::slice::from_ref(attr))
                    .into_iter()
                    .map(|info| (info.line, info.lint)),
            );
        }
    }
    out
}

/// Implements `find inline mod allows`.
pub(crate) fn find_inline_mod_allows(source: &syn::File) -> Vec<InlineModAllow> {
    let mut out = Vec::new();
    for item in &source.items {
        if let syn::Item::Mod(item_mod) = item {
            collect_mod_inner_allows(item_mod, &item_mod.ident.to_string(), &mut out);
        }
    }
    out
}

/// Implements `find item lint policies`.
pub(crate) fn find_item_lint_policies(source: &syn::File) -> Vec<LintPolicyInfo> {
    let mut visitor = ItemOnlyPolicyVisitor { out: Vec::new() };
    visitor.visit_file(source);
    visitor.out
}

/// Implements `find impl block allows`.
pub(crate) fn find_impl_block_allows(source: &syn::File) -> Vec<ImplAllowInfo> {
    let mut visitor = ImplAllowVisitor { out: Vec::new() };
    visitor.visit_file(source);
    visitor.out
}

/// Implements `find deny forbid attrs`.
pub(crate) fn find_deny_forbid_attrs(source: &syn::File) -> Vec<DenyForbidInfo> {
    let mut out = Vec::new();
    helpers::collect_deny_forbid_attrs(&source.attrs, true, &mut out);
    helpers::collect_cfg_attr_deny_forbid_attrs(&source.attrs, true, &mut out);
    let mut visitor = DenyForbidVisitor { out: &mut out };
    visitor.visit_file(source);
    out
}

/// Implements `find foreign mod allows`.
pub(crate) fn find_foreign_mod_allows(source: &syn::File) -> Vec<ForeignModAllowInfo> {
    let mut visitor = ForeignModAllowVisitor { out: Vec::new() };
    visitor.visit_file(source);
    visitor.out
}

/// Implements `find include macros`.
pub(crate) fn find_include_macros(source: &syn::File) -> Vec<IncludeMacroInfo> {
    let mut visitor = IncludeMacroVisitor { out: Vec::new() };
    visitor.visit_file(source);
    visitor.out
}

/// Implements `find cfg attr lint policies`.
pub(crate) fn find_cfg_attr_lint_policies(source: &syn::File) -> Vec<CfgAttrLintInfo> {
    let mut out = Vec::new();
    helpers::collect_cfg_attr_lint_policies(&source.attrs, &mut out);
    let mut visitor = CfgAttrPolicyVisitor { out: &mut out };
    visitor.visit_file(source);
    out
}

/// Implements `find path attrs`.
pub(crate) fn find_path_attrs(source: &syn::File) -> Vec<PathAttrInfo> {
    let mut visitor = PathAttrVisitor { out: Vec::new() };
    visitor.visit_file(source);
    visitor.out
}

/// Implements `collect mod inner allows`.
fn collect_mod_inner_allows(item_mod: &syn::ItemMod, path: &str, out: &mut Vec<InlineModAllow>) {
    let Some((_, items)) = &item_mod.content else {
        return;
    };

    for attr in &item_mod.attrs {
        if matches!(attr.style, syn::AttrStyle::Inner(_)) && attr.path().is_ident("allow") {
            for info in helpers::collect_item_lint_policies(std::slice::from_ref(attr)) {
                out.push(InlineModAllow {
                    line: info.line,
                    lint: info.lint,
                    module_path: path.to_owned(),
                });
            }
        }
    }

    for item in items {
        if let syn::Item::Mod(nested) = item {
            let nested_path = format!("{path}::{}", nested.ident);
            collect_mod_inner_allows(nested, &nested_path, out);
        }
    }
}

/// Struct `ItemOnlyPolicyVisitor` used by this module.
struct ItemOnlyPolicyVisitor {
    /// Field `out`.
    out: Vec<LintPolicyInfo>,
}

/// Struct `ImplAllowVisitor` used by this module.
struct ImplAllowVisitor {
    /// Field `out`.
    out: Vec<ImplAllowInfo>,
}

/// Struct `DenyForbidVisitor` used by this module.
struct DenyForbidVisitor<'a> {
    /// Field `out`.
    out: &'a mut Vec<DenyForbidInfo>,
}

/// Struct `ForeignModAllowVisitor` used by this module.
struct ForeignModAllowVisitor {
    /// Field `out`.
    out: Vec<ForeignModAllowInfo>,
}

/// Struct `IncludeMacroVisitor` used by this module.
struct IncludeMacroVisitor {
    /// Field `out`.
    out: Vec<IncludeMacroInfo>,
}

/// Struct `CfgAttrPolicyVisitor` used by this module.
struct CfgAttrPolicyVisitor<'a> {
    /// Field `out`.
    out: &'a mut Vec<CfgAttrLintInfo>,
}

/// Struct `PathAttrVisitor` used by this module.
struct PathAttrVisitor {
    /// Field `out`.
    out: Vec<PathAttrInfo>,
}

/// Implements `extract path attr`.
fn extract_path_attr(attr: &syn::Attribute) -> Option<String> {
    let syn::Meta::NameValue(name_value) = &attr.meta else {
        return None;
    };
    let syn::Expr::Lit(expr_lit) = &name_value.value else {
        return None;
    };
    let syn::Lit::Str(value) = &expr_lit.lit else {
        return None;
    };
    Some(value.value())
}

/// Implements `path string has parent segment`.
fn path_string_has_parent_segment(path: &str) -> bool {
    path.split('/').any(|segment| segment == "..")
        || path.split('\\').any(|segment| segment == "..")
}

/// Implements `is test sidecar exempt`.
fn is_test_sidecar_exempt(item_mod: &syn::ItemMod, path_value: &str) -> bool {
    if !helpers::attrs_enter_test_context(&item_mod.attrs) {
        return false;
    }
    item_mod.ident.to_string().ends_with("_tests")
        && path_value == format!("{}/mod.rs", item_mod.ident)
}

impl<'source> Visit<'source> for ItemOnlyPolicyVisitor {
    fn visit_item(&mut self, item: &'source syn::Item) {
        if !matches!(item, syn::Item::ForeignMod(_)) {
            self.out
                .extend(helpers::collect_item_lint_policies(helpers::item_attrs(
                    item,
                )));
        }
        syn::visit::visit_item(self, item);
    }

    fn visit_impl_item(&mut self, item: &'source syn::ImplItem) {
        self.out.extend(helpers::collect_item_lint_policies(
            helpers::impl_item_attrs(item),
        ));
        syn::visit::visit_impl_item(self, item);
    }

    fn visit_trait_item(&mut self, item: &'source syn::TraitItem) {
        self.out.extend(helpers::collect_item_lint_policies(
            helpers::trait_item_attrs(item),
        ));
        syn::visit::visit_trait_item(self, item);
    }
}

impl<'source> Visit<'source> for ImplAllowVisitor {
    fn visit_item_impl(&mut self, item_impl: &'source syn::ItemImpl) {
        let method_count = item_impl
            .items
            .iter()
            .filter(|item| matches!(item, syn::ImplItem::Fn(_)))
            .count();
        if method_count > 3 {
            for info in helpers::collect_item_lint_policies(&item_impl.attrs) {
                self.out.push(ImplAllowInfo {
                    line: info.line,
                    lint: info.lint,
                    kind: info.kind,
                    method_count,
                });
            }
        }
        syn::visit::visit_item_impl(self, item_impl);
    }
}

impl<'source> Visit<'source> for DenyForbidVisitor<'source> {
    fn visit_item(&mut self, item: &'source syn::Item) {
        helpers::collect_deny_forbid_attrs(helpers::item_attrs(item), false, self.out);
        helpers::collect_cfg_attr_deny_forbid_attrs(helpers::item_attrs(item), false, self.out);
        syn::visit::visit_item(self, item);
    }

    fn visit_impl_item(&mut self, item: &'source syn::ImplItem) {
        helpers::collect_deny_forbid_attrs(helpers::impl_item_attrs(item), false, self.out);
        helpers::collect_cfg_attr_deny_forbid_attrs(
            helpers::impl_item_attrs(item),
            false,
            self.out,
        );
        syn::visit::visit_impl_item(self, item);
    }

    fn visit_trait_item(&mut self, item: &'source syn::TraitItem) {
        helpers::collect_deny_forbid_attrs(helpers::trait_item_attrs(item), false, self.out);
        helpers::collect_cfg_attr_deny_forbid_attrs(
            helpers::trait_item_attrs(item),
            false,
            self.out,
        );
        syn::visit::visit_trait_item(self, item);
    }
}

impl<'source> Visit<'source> for ForeignModAllowVisitor {
    fn visit_item_foreign_mod(&mut self, item: &'source syn::ItemForeignMod) {
        for info in helpers::collect_item_lint_policies(&item.attrs) {
            self.out.push(ForeignModAllowInfo {
                line: info.line,
                lint: info.lint,
                kind: info.kind,
                via_cfg_attr: false,
            });
        }
        let mut cfg_infos = Vec::new();
        helpers::collect_cfg_attr_lint_policies(&item.attrs, &mut cfg_infos);
        for info in cfg_infos {
            if info.truth == CfgPredicateTruth::KnownFalse {
                continue;
            }
            self.out.push(ForeignModAllowInfo {
                line: info.line,
                lint: info.lint,
                kind: info.kind,
                via_cfg_attr: true,
            });
        }
        syn::visit::visit_item_foreign_mod(self, item);
    }
}

impl<'source> Visit<'source> for IncludeMacroVisitor {
    fn visit_macro(&mut self, macro_call: &'source syn::Macro) {
        let name = helpers::path_to_string(&macro_call.path);
        let base = name.rsplit("::").next().unwrap_or(&name);
        if matches!(base, "include" | "include_str" | "include_bytes") {
            let exprs = analysis_helpers::macro_token_exprs(macro_call);
            let build_script_pattern = base == "include"
                && exprs.len() == 1
                && analysis_helpers::expr_is_out_dir_concat(&exprs[0]);
            let path_traversal = exprs.iter().any(analysis_helpers::expr_has_path_traversal);
            self.out.push(IncludeMacroInfo {
                line: helpers::span_line(macro_call.path.span()),
                macro_name: base.to_owned(),
                build_script_pattern,
                path_traversal,
            });
        }
        syn::visit::visit_macro(self, macro_call);
    }
}

impl<'source> Visit<'source> for CfgAttrPolicyVisitor<'source> {
    fn visit_item(&mut self, item: &'source syn::Item) {
        if !matches!(item, syn::Item::ForeignMod(_)) {
            helpers::collect_cfg_attr_lint_policies(helpers::item_attrs(item), self.out);
        }
        syn::visit::visit_item(self, item);
    }

    fn visit_impl_item(&mut self, item: &'source syn::ImplItem) {
        helpers::collect_cfg_attr_lint_policies(helpers::impl_item_attrs(item), self.out);
        syn::visit::visit_impl_item(self, item);
    }

    fn visit_trait_item(&mut self, item: &'source syn::TraitItem) {
        helpers::collect_cfg_attr_lint_policies(helpers::trait_item_attrs(item), self.out);
        syn::visit::visit_trait_item(self, item);
    }
}

impl<'source> Visit<'source> for PathAttrVisitor {
    fn visit_item_mod(&mut self, item_mod: &'source syn::ItemMod) {
        for attr in &item_mod.attrs {
            if attr.path().is_ident("path") {
                if let Some(path_value) = extract_path_attr(attr) {
                    self.out.push(PathAttrInfo {
                        line: helpers::span_line(attr.span()),
                        module_name: item_mod.ident.to_string(),
                        path_value: path_value.clone(),
                        via_cfg_attr: false,
                        cfg_truth: CfgPredicateTruth::KnownTrue,
                        is_test_sidecar_exempt: is_test_sidecar_exempt(item_mod, &path_value),
                        escapes_parent: path_string_has_parent_segment(&path_value),
                    });
                }
                continue;
            }

            if !attr.path().is_ident("cfg_attr") {
                continue;
            }

            analysis_helpers::walk_cfg_attr_payloads(attr, |line, truth, meta| {
                let syn::Meta::NameValue(name_value) = meta else {
                    return;
                };
                if !name_value.path.is_ident("path") {
                    return;
                }
                let syn::Expr::Lit(expr_lit) = &name_value.value else {
                    return;
                };
                let syn::Lit::Str(value) = &expr_lit.lit else {
                    return;
                };
                let path_value = value.value();
                self.out.push(PathAttrInfo {
                    line,
                    module_name: item_mod.ident.to_string(),
                    path_value: path_value.clone(),
                    via_cfg_attr: true,
                    cfg_truth: truth,
                    is_test_sidecar_exempt: is_test_sidecar_exempt(item_mod, &path_value),
                    escapes_parent: path_string_has_parent_segment(&path_value),
                });
            });
        }

        syn::visit::visit_item_mod(self, item_mod);
    }
}
