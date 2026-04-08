use super::analysis_helpers;
use super::helpers;
use super::types::{
    CfgAttrLintInfo, CfgPredicateTruth, ForeignModAllowInfo, ImplAllowInfo, IncludeMacroInfo,
};
use syn::spanned::Spanned;
use syn::visit::Visit;

pub(crate) fn find_impl_block_allows(ast: &syn::File) -> Vec<ImplAllowInfo> {
    let mut visitor = ImplAllowVisitor { out: Vec::new() };
    visitor.visit_file(ast);
    visitor.out
}

pub(crate) fn find_foreign_mod_allows(ast: &syn::File) -> Vec<ForeignModAllowInfo> {
    let mut visitor = ForeignModAllowVisitor { out: Vec::new() };
    visitor.visit_file(ast);
    visitor.out
}

pub(crate) fn find_include_macros(ast: &syn::File) -> Vec<IncludeMacroInfo> {
    let mut visitor = IncludeMacroVisitor { out: Vec::new() };
    visitor.visit_file(ast);
    visitor.out
}

pub(crate) fn find_cfg_attr_lint_policies(ast: &syn::File) -> Vec<CfgAttrLintInfo> {
    let mut out = Vec::new();
    helpers::collect_cfg_attr_lint_policies(&ast.attrs, &mut out);
    let mut visitor = CfgAttrPolicyVisitor { out: &mut out };
    visitor.visit_file(ast);
    out
}

struct ImplAllowVisitor {
    out: Vec<ImplAllowInfo>,
}

struct ForeignModAllowVisitor {
    out: Vec<ForeignModAllowInfo>,
}

struct IncludeMacroVisitor {
    out: Vec<IncludeMacroInfo>,
}

struct CfgAttrPolicyVisitor<'a> {
    out: &'a mut Vec<CfgAttrLintInfo>,
}

impl<'ast> Visit<'ast> for ImplAllowVisitor {
    fn visit_item_impl(&mut self, item_impl: &'ast syn::ItemImpl) {
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

impl<'ast> Visit<'ast> for ForeignModAllowVisitor {
    fn visit_item_foreign_mod(&mut self, item: &'ast syn::ItemForeignMod) {
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

impl<'ast> Visit<'ast> for IncludeMacroVisitor {
    fn visit_macro(&mut self, macro_call: &'ast syn::Macro) {
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

impl<'ast> Visit<'ast> for CfgAttrPolicyVisitor<'ast> {
    fn visit_item(&mut self, item: &'ast syn::Item) {
        if !matches!(item, syn::Item::ForeignMod(_)) {
            helpers::collect_cfg_attr_lint_policies(helpers::item_attrs(item), self.out);
        }
        syn::visit::visit_item(self, item);
    }

    fn visit_impl_item(&mut self, item: &'ast syn::ImplItem) {
        helpers::collect_cfg_attr_lint_policies(helpers::impl_item_attrs(item), self.out);
        syn::visit::visit_impl_item(self, item);
    }

    fn visit_trait_item(&mut self, item: &'ast syn::TraitItem) {
        helpers::collect_cfg_attr_lint_policies(helpers::trait_item_attrs(item), self.out);
        syn::visit::visit_trait_item(self, item);
    }
}
