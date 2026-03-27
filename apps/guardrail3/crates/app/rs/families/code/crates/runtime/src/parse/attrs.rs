use super::helpers::{
    collect_allow_lints, collect_always_true_cfg_attr_allows, collect_cfg_attr_allow_lints,
    collect_cfg_attr_path_attrs, collect_deny_forbid_attrs, collect_path_attrs,
    expr_contains_out_dir, expr_has_path_traversal, impl_item_attrs, item_attrs,
    macro_token_exprs, path_to_string, result_error_kind, span_line, trait_item_attrs,
};
use super::types::{
    DenyForbidInfo, ForeignModAllowInfo, ImplAllowInfo, IncludeMacroInfo, PathAttrInfo,
    PublicResultErrorInfo,
};
use guardrail3_app_rs_ast::ast_helpers;
use syn::spanned::Spanned;
use syn::visit::Visit;

pub fn find_item_allows(ast: &syn::File) -> Vec<(usize, String)> {
    let mut visitor = ItemOnlyAllowVisitor { out: Vec::new() };
    visitor.visit_file(ast);
    visitor.out
}

pub fn find_impl_block_allows(ast: &syn::File) -> Vec<ImplAllowInfo> {
    let mut visitor = ImplAllowVisitor { out: Vec::new() };
    visitor.visit_file(ast);
    visitor.out
}

pub fn find_deny_forbid_attrs(ast: &syn::File) -> Vec<DenyForbidInfo> {
    let mut out = Vec::new();
    collect_deny_forbid_attrs(&ast.attrs, true, &mut out);
    let mut visitor = DenyForbidVisitor { out: &mut out };
    visitor.visit_file(ast);
    out
}

pub fn find_foreign_mod_allows(ast: &syn::File) -> Vec<ForeignModAllowInfo> {
    let mut visitor = ForeignModAllowVisitor { out: Vec::new() };
    visitor.visit_file(ast);
    visitor.out
}

pub fn find_include_macros(ast: &syn::File) -> Vec<IncludeMacroInfo> {
    let mut visitor = IncludeMacroVisitor { out: Vec::new() };
    visitor.visit_file(ast);
    visitor.out
}

pub fn find_always_true_cfg_attr_allows(ast: &syn::File) -> Vec<ast_helpers::CfgAttrAllowInfo> {
    let mut out = Vec::new();
    collect_always_true_cfg_attr_allows(&ast.attrs, &mut out);
    let mut visitor = AlwaysTrueCfgAttrVisitor { out: &mut out };
    visitor.visit_file(ast);
    out
}

pub fn find_cfg_attr_allows(ast: &syn::File) -> Vec<ast_helpers::CfgAttrAllowInfo> {
    let always_true = find_always_true_cfg_attr_allows(ast)
        .into_iter()
        .map(|info| (info.line, info.lint))
        .collect::<std::collections::BTreeSet<_>>();
    ast_helpers::find_cfg_attr_allows(ast)
        .into_iter()
        .map(|mut info| {
            if always_true.contains(&(info.line, info.lint.clone())) {
                info.is_always_true = true;
            }
            info
        })
        .collect()
}

pub fn find_path_attrs(ast: &syn::File) -> Vec<PathAttrInfo> {
    let mut out = Vec::new();
    collect_path_attrs(&ast.attrs, &mut out);
    collect_cfg_attr_path_attrs(&ast.attrs, &mut out);
    let mut visitor = PathAttrVisitor { out: &mut out };
    visitor.visit_file(ast);
    out
}

pub fn find_public_result_error_types(ast: &syn::File) -> Vec<PublicResultErrorInfo> {
    let mut visitor = PublicResultErrorVisitor { out: Vec::new() };
    visitor.visit_file(ast);
    visitor.out
}

struct ItemOnlyAllowVisitor {
    out: Vec<(usize, String)>,
}

struct ImplAllowVisitor {
    out: Vec<ImplAllowInfo>,
}

struct DenyForbidVisitor<'a> {
    out: &'a mut Vec<DenyForbidInfo>,
}

struct ForeignModAllowVisitor {
    out: Vec<ForeignModAllowInfo>,
}

struct IncludeMacroVisitor {
    out: Vec<IncludeMacroInfo>,
}

struct AlwaysTrueCfgAttrVisitor<'a> {
    out: &'a mut Vec<ast_helpers::CfgAttrAllowInfo>,
}

struct PathAttrVisitor<'a> {
    out: &'a mut Vec<PathAttrInfo>,
}

struct PublicResultErrorVisitor {
    out: Vec<PublicResultErrorInfo>,
}

impl<'ast> Visit<'ast> for ItemOnlyAllowVisitor {
    fn visit_item(&mut self, item: &'ast syn::Item) {
        if !matches!(item, syn::Item::ForeignMod(_)) {
            self.out.extend(collect_allow_lints(item_attrs(item)));
        }
        syn::visit::visit_item(self, item);
    }

    fn visit_impl_item(&mut self, item: &'ast syn::ImplItem) {
        self.out.extend(collect_allow_lints(impl_item_attrs(item)));
        syn::visit::visit_impl_item(self, item);
    }

    fn visit_trait_item(&mut self, item: &'ast syn::TraitItem) {
        self.out.extend(collect_allow_lints(trait_item_attrs(item)));
        syn::visit::visit_trait_item(self, item);
    }
}

impl<'ast> Visit<'ast> for ImplAllowVisitor {
    fn visit_item_impl(&mut self, item_impl: &'ast syn::ItemImpl) {
        let method_count = item_impl
            .items
            .iter()
            .filter(|item| matches!(item, syn::ImplItem::Fn(_)))
            .count();
        if method_count > 3 {
            for (line, lint) in collect_allow_lints(&item_impl.attrs) {
                self.out.push(ImplAllowInfo {
                    line,
                    lint,
                    method_count,
                });
            }
        }
        syn::visit::visit_item_impl(self, item_impl);
    }
}

impl<'ast> Visit<'ast> for DenyForbidVisitor<'ast> {
    fn visit_item(&mut self, item: &'ast syn::Item) {
        collect_deny_forbid_attrs(item_attrs(item), false, self.out);
        syn::visit::visit_item(self, item);
    }

    fn visit_impl_item(&mut self, item: &'ast syn::ImplItem) {
        collect_deny_forbid_attrs(impl_item_attrs(item), false, self.out);
        syn::visit::visit_impl_item(self, item);
    }

    fn visit_trait_item(&mut self, item: &'ast syn::TraitItem) {
        collect_deny_forbid_attrs(trait_item_attrs(item), false, self.out);
        syn::visit::visit_trait_item(self, item);
    }
}

impl<'ast> Visit<'ast> for ForeignModAllowVisitor {
    fn visit_item_foreign_mod(&mut self, item: &'ast syn::ItemForeignMod) {
        for (line, lint) in collect_allow_lints(&item.attrs) {
            self.out.push(ForeignModAllowInfo {
                line,
                lint,
                via_cfg_attr: false,
            });
        }
        for (line, lint) in collect_cfg_attr_allow_lints(&item.attrs) {
            self.out.push(ForeignModAllowInfo {
                line,
                lint,
                via_cfg_attr: true,
            });
        }
        syn::visit::visit_item_foreign_mod(self, item);
    }
}

impl<'ast> Visit<'ast> for IncludeMacroVisitor {
    fn visit_macro(&mut self, macro_call: &'ast syn::Macro) {
        let name = path_to_string(&macro_call.path);
        let base = name.rsplit("::").next().unwrap_or(&name);
        if matches!(base, "include" | "include_str" | "include_bytes") {
            let exprs = macro_token_exprs(macro_call);
            let build_script_pattern = exprs.iter().any(expr_contains_out_dir);
            let path_traversal = exprs.iter().any(expr_has_path_traversal);
            self.out.push(IncludeMacroInfo {
                line: span_line(macro_call.path.span()),
                macro_name: base.to_owned(),
                build_script_pattern,
                path_traversal,
            });
        }
        syn::visit::visit_macro(self, macro_call);
    }
}

impl<'ast> Visit<'ast> for AlwaysTrueCfgAttrVisitor<'ast> {
    fn visit_item(&mut self, item: &'ast syn::Item) {
        if !matches!(item, syn::Item::ForeignMod(_)) {
            collect_always_true_cfg_attr_allows(item_attrs(item), self.out);
        }
        syn::visit::visit_item(self, item);
    }

    fn visit_impl_item(&mut self, item: &'ast syn::ImplItem) {
        collect_always_true_cfg_attr_allows(impl_item_attrs(item), self.out);
        syn::visit::visit_impl_item(self, item);
    }
}

impl<'ast> Visit<'ast> for PathAttrVisitor<'ast> {
    fn visit_item(&mut self, item: &'ast syn::Item) {
        collect_path_attrs(item_attrs(item), self.out);
        collect_cfg_attr_path_attrs(item_attrs(item), self.out);
        syn::visit::visit_item(self, item);
    }
}

impl<'ast> Visit<'ast> for PublicResultErrorVisitor {
    fn visit_item_fn(&mut self, item_fn: &'ast syn::ItemFn) {
        if !matches!(item_fn.vis, syn::Visibility::Public(_)) {
            return syn::visit::visit_item_fn(self, item_fn);
        }
        if let syn::ReturnType::Type(_, ty) = &item_fn.sig.output {
            if let Some(kind) = result_error_kind(ty) {
                self.out.push(PublicResultErrorInfo {
                    line: span_line(item_fn.sig.ident.span()),
                    fn_name: item_fn.sig.ident.to_string(),
                    kind,
                });
            }
        }
        syn::visit::visit_item_fn(self, item_fn);
    }

    fn visit_impl_item_fn(&mut self, item_fn: &'ast syn::ImplItemFn) {
        if !matches!(item_fn.vis, syn::Visibility::Public(_)) {
            return syn::visit::visit_impl_item_fn(self, item_fn);
        }
        if let syn::ReturnType::Type(_, ty) = &item_fn.sig.output {
            if let Some(kind) = result_error_kind(ty) {
                self.out.push(PublicResultErrorInfo {
                    line: span_line(item_fn.sig.ident.span()),
                    fn_name: item_fn.sig.ident.to_string(),
                    kind,
                });
            }
        }
        syn::visit::visit_impl_item_fn(self, item_fn);
    }
}
