use std::collections::BTreeSet;

use super::analysis_helpers;
use super::helpers;
use super::types::{self, CfgPredicateTruth};
use syn::spanned::Spanned;
use syn::visit::Visit;

type CfgAttrLintInfo = types::CfgAttrLintInfo;
type DenyForbidInfo = types::DenyForbidInfo;
type ForeignModAllowInfo = types::ForeignModAllowInfo;
type ImplAllowInfo = types::ImplAllowInfo;
type IncludeMacroInfo = types::IncludeMacroInfo;
type LintPolicyInfo = types::LintPolicyInfo;
type PathAttrInfo = types::PathAttrInfo;
type PublicResultErrorInfo = types::PublicResultErrorInfo;

pub fn find_item_lint_policies(ast: &syn::File) -> Vec<LintPolicyInfo> {
    let mut visitor = ItemOnlyPolicyVisitor { out: Vec::new() };
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
    helpers::collect_deny_forbid_attrs(&ast.attrs, true, &mut out);
    helpers::collect_cfg_attr_deny_forbid_attrs(&ast.attrs, true, &mut out);
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

pub fn find_cfg_attr_lint_policies(ast: &syn::File) -> Vec<CfgAttrLintInfo> {
    let mut out = Vec::new();
    helpers::collect_cfg_attr_lint_policies(&ast.attrs, &mut out);
    let mut visitor = CfgAttrPolicyVisitor { out: &mut out };
    visitor.visit_file(ast);
    out
}

pub fn find_path_attrs(ast: &syn::File) -> Vec<PathAttrInfo> {
    let mut out = Vec::new();
    helpers::collect_path_attrs(&ast.attrs, &mut out);
    helpers::collect_cfg_attr_path_attrs(&ast.attrs, &mut out);
    let mut visitor = PathAttrVisitor { out: &mut out };
    visitor.visit_file(ast);
    out
}

pub fn find_public_result_error_types(ast: &syn::File) -> Vec<PublicResultErrorInfo> {
    let reachable_types = collect_reachable_public_types(ast);
    let mut visitor = PublicResultErrorVisitor {
        out: Vec::new(),
        public_module_stack: vec![true],
        module_path: Vec::new(),
        reachable_types,
    };
    visitor.visit_file(ast);
    visitor.out
}

struct ItemOnlyPolicyVisitor {
    out: Vec<LintPolicyInfo>,
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

struct CfgAttrPolicyVisitor<'a> {
    out: &'a mut Vec<CfgAttrLintInfo>,
}

struct PathAttrVisitor<'a> {
    out: &'a mut Vec<PathAttrInfo>,
}

struct PublicResultErrorVisitor {
    out: Vec<PublicResultErrorInfo>,
    public_module_stack: Vec<bool>,
    module_path: Vec<String>,
    reachable_types: BTreeSet<String>,
}

impl<'ast> Visit<'ast> for ItemOnlyPolicyVisitor {
    fn visit_item(&mut self, item: &'ast syn::Item) {
        if !matches!(item, syn::Item::ForeignMod(_)) {
            self.out
                .extend(helpers::collect_item_lint_policies(helpers::item_attrs(
                    item,
                )));
        }
        syn::visit::visit_item(self, item);
    }

    fn visit_impl_item(&mut self, item: &'ast syn::ImplItem) {
        self.out.extend(helpers::collect_item_lint_policies(
            helpers::impl_item_attrs(item),
        ));
        syn::visit::visit_impl_item(self, item);
    }

    fn visit_trait_item(&mut self, item: &'ast syn::TraitItem) {
        self.out.extend(helpers::collect_item_lint_policies(
            helpers::trait_item_attrs(item),
        ));
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

impl<'ast> Visit<'ast> for DenyForbidVisitor<'ast> {
    fn visit_item(&mut self, item: &'ast syn::Item) {
        helpers::collect_deny_forbid_attrs(helpers::item_attrs(item), false, self.out);
        helpers::collect_cfg_attr_deny_forbid_attrs(helpers::item_attrs(item), false, self.out);
        syn::visit::visit_item(self, item);
    }

    fn visit_impl_item(&mut self, item: &'ast syn::ImplItem) {
        helpers::collect_deny_forbid_attrs(helpers::impl_item_attrs(item), false, self.out);
        helpers::collect_cfg_attr_deny_forbid_attrs(
            helpers::impl_item_attrs(item),
            false,
            self.out,
        );
        syn::visit::visit_impl_item(self, item);
    }

    fn visit_trait_item(&mut self, item: &'ast syn::TraitItem) {
        helpers::collect_deny_forbid_attrs(helpers::trait_item_attrs(item), false, self.out);
        helpers::collect_cfg_attr_deny_forbid_attrs(
            helpers::trait_item_attrs(item),
            false,
            self.out,
        );
        syn::visit::visit_trait_item(self, item);
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

impl<'ast> Visit<'ast> for PathAttrVisitor<'ast> {
    fn visit_item(&mut self, item: &'ast syn::Item) {
        helpers::collect_path_attrs(helpers::item_attrs(item), self.out);
        helpers::collect_cfg_attr_path_attrs(helpers::item_attrs(item), self.out);
        syn::visit::visit_item(self, item);
    }
}

impl PublicResultErrorVisitor {
    fn current_module_public(&self) -> bool {
        self.public_module_stack.last().copied().unwrap_or(true)
    }

    fn with_nested_module(&mut self, item_mod: &syn::ItemMod, visit: impl FnOnce(&mut Self)) {
        let next =
            self.current_module_public() && matches!(item_mod.vis, syn::Visibility::Public(_));
        self.public_module_stack.push(next);
        self.module_path.push(item_mod.ident.to_string());
        visit(self);
        let _ = self.module_path.pop();
        let _ = self.public_module_stack.pop();
    }
}

impl<'ast> Visit<'ast> for PublicResultErrorVisitor {
    fn visit_item_mod(&mut self, item_mod: &'ast syn::ItemMod) {
        self.with_nested_module(item_mod, |visitor| {
            syn::visit::visit_item_mod(visitor, item_mod);
        });
    }

    fn visit_item_fn(&mut self, item_fn: &'ast syn::ItemFn) {
        if self.current_module_public() && matches!(item_fn.vis, syn::Visibility::Public(_)) {
            if let syn::ReturnType::Type(_, ty) = &item_fn.sig.output {
                if let Some(kind) = analysis_helpers::result_error_kind(ty) {
                    self.out.push(PublicResultErrorInfo {
                        line: helpers::span_line(item_fn.sig.ident.span()),
                        fn_name: item_fn.sig.ident.to_string(),
                        kind,
                    });
                }
            }
        }
        syn::visit::visit_item_fn(self, item_fn);
    }

    fn visit_item_impl(&mut self, item_impl: &'ast syn::ItemImpl) {
        let impl_is_public = self.current_module_public()
            && item_impl.trait_.is_none()
            && self
                .reachable_type_name(&item_impl.self_ty)
                .is_some_and(|name| self.reachable_types.contains(&name));

        if impl_is_public {
            for item in &item_impl.items {
                let syn::ImplItem::Fn(item_fn) = item else {
                    continue;
                };
                if !matches!(item_fn.vis, syn::Visibility::Public(_)) {
                    continue;
                }
                let syn::ReturnType::Type(_, ty) = &item_fn.sig.output else {
                    continue;
                };
                if let Some(kind) = analysis_helpers::result_error_kind(ty) {
                    self.out.push(PublicResultErrorInfo {
                        line: helpers::span_line(item_fn.sig.ident.span()),
                        fn_name: item_fn.sig.ident.to_string(),
                        kind,
                    });
                }
            }
        }
        syn::visit::visit_item_impl(self, item_impl);
    }

    fn visit_item_trait(&mut self, item_trait: &'ast syn::ItemTrait) {
        if self.current_module_public() && matches!(item_trait.vis, syn::Visibility::Public(_)) {
            for item in &item_trait.items {
                let syn::TraitItem::Fn(item_fn) = item else {
                    continue;
                };
                let syn::ReturnType::Type(_, ty) = &item_fn.sig.output else {
                    continue;
                };
                if let Some(kind) = analysis_helpers::result_error_kind(ty) {
                    self.out.push(PublicResultErrorInfo {
                        line: helpers::span_line(item_fn.sig.ident.span()),
                        fn_name: format!("{}::{}", item_trait.ident, item_fn.sig.ident),
                        kind,
                    });
                }
            }
        }
        syn::visit::visit_item_trait(self, item_trait);
    }
}

impl PublicResultErrorVisitor {
    fn reachable_type_name(&self, ty: &syn::Type) -> Option<String> {
        let syn::Type::Path(type_path) = ty else {
            return None;
        };
        if type_path.qself.is_some() {
            return None;
        }

        let segments = type_path
            .path
            .segments
            .iter()
            .map(|segment| segment.ident.to_string())
            .collect::<Vec<_>>();
        normalize_type_path(
            &self.module_path,
            type_path.path.leading_colon.is_some(),
            &segments,
        )
    }
}

fn collect_reachable_public_types(ast: &syn::File) -> BTreeSet<String> {
    let mut visitor = ReachablePublicTypeVisitor {
        public_module_stack: vec![true],
        module_path: Vec::new(),
        names: BTreeSet::new(),
    };
    visitor.visit_file(ast);
    visitor.names
}

struct ReachablePublicTypeVisitor {
    public_module_stack: Vec<bool>,
    module_path: Vec<String>,
    names: BTreeSet<String>,
}

impl ReachablePublicTypeVisitor {
    fn current_module_public(&self) -> bool {
        self.public_module_stack.last().copied().unwrap_or(true)
    }

    fn maybe_record(&mut self, vis: &syn::Visibility, ident: &syn::Ident) {
        if self.current_module_public() && matches!(vis, syn::Visibility::Public(_)) {
            let mut path = self.module_path.clone();
            path.push(ident.to_string());
            let _ = self.names.insert(path.join("::"));
        }
    }

    fn with_nested_module(&mut self, item_mod: &syn::ItemMod, visit: impl FnOnce(&mut Self)) {
        let next =
            self.current_module_public() && matches!(item_mod.vis, syn::Visibility::Public(_));
        self.public_module_stack.push(next);
        self.module_path.push(item_mod.ident.to_string());
        visit(self);
        let _ = self.module_path.pop();
        let _ = self.public_module_stack.pop();
    }
}

impl<'ast> Visit<'ast> for ReachablePublicTypeVisitor {
    fn visit_item_mod(&mut self, item_mod: &'ast syn::ItemMod) {
        self.with_nested_module(item_mod, |visitor| {
            syn::visit::visit_item_mod(visitor, item_mod);
        });
    }

    fn visit_item_struct(&mut self, item: &'ast syn::ItemStruct) {
        self.maybe_record(&item.vis, &item.ident);
        syn::visit::visit_item_struct(self, item);
    }

    fn visit_item_enum(&mut self, item: &'ast syn::ItemEnum) {
        self.maybe_record(&item.vis, &item.ident);
        syn::visit::visit_item_enum(self, item);
    }

    fn visit_item_union(&mut self, item: &'ast syn::ItemUnion) {
        self.maybe_record(&item.vis, &item.ident);
        syn::visit::visit_item_union(self, item);
    }

    fn visit_item_type(&mut self, item: &'ast syn::ItemType) {
        self.maybe_record(&item.vis, &item.ident);
        syn::visit::visit_item_type(self, item);
    }
}

fn normalize_type_path(
    current_module_path: &[String],
    leading_colon: bool,
    segments: &[String],
) -> Option<String> {
    let first = segments.first()?;

    let normalized = if leading_colon {
        if first == "crate" {
            segments.get(1..).unwrap_or(&[]).to_vec()
        } else {
            segments.to_vec()
        }
    } else {
        match first.as_str() {
            "crate" => segments.get(1..).unwrap_or(&[]).to_vec(),
            "self" => {
                let mut resolved = current_module_path.to_vec();
                resolved.extend_from_slice(segments.get(1..).unwrap_or(&[]));
                resolved
            }
            "super" => {
                let mut resolved = current_module_path.to_vec();
                if resolved.pop().is_none() {
                    return None;
                }
                resolved.extend_from_slice(segments.get(1..).unwrap_or(&[]));
                resolved
            }
            _ => {
                let mut resolved = current_module_path.to_vec();
                resolved.extend_from_slice(segments);
                resolved
            }
        }
    };

    if normalized.is_empty() {
        None
    } else {
        Some(normalized.join("::"))
    }
}
