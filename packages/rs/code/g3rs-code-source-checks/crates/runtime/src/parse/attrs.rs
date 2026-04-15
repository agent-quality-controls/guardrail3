use std::collections::BTreeSet;

use super::analysis_helpers;
use super::helpers;
use super::types::{
    CfgAttrLintInfo, CfgPredicateTruth, DenyForbidInfo, ForeignModAllowInfo, ImplAllowInfo,
    IncludeMacroInfo, InlineModAllow, LintPolicyInfo, PathAttrInfo, PublicResultErrorInfo,
    PublicStructFieldBagInfo,
};
use syn::spanned::Spanned;
use syn::visit::Visit;

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

pub(crate) fn find_inline_mod_allows(source: &syn::File) -> Vec<InlineModAllow> {
    let mut out = Vec::new();
    for item in &source.items {
        if let syn::Item::Mod(item_mod) = item {
            collect_mod_inner_allows(item_mod, &item_mod.ident.to_string(), &mut out);
        }
    }
    out
}

pub(crate) fn find_item_lint_policies(source: &syn::File) -> Vec<LintPolicyInfo> {
    let mut visitor = ItemOnlyPolicyVisitor { out: Vec::new() };
    visitor.visit_file(source);
    visitor.out
}

pub(crate) fn find_impl_block_allows(source: &syn::File) -> Vec<ImplAllowInfo> {
    let mut visitor = ImplAllowVisitor { out: Vec::new() };
    visitor.visit_file(source);
    visitor.out
}

pub(crate) fn find_deny_forbid_attrs(source: &syn::File) -> Vec<DenyForbidInfo> {
    let mut out = Vec::new();
    helpers::collect_deny_forbid_attrs(&source.attrs, true, &mut out);
    helpers::collect_cfg_attr_deny_forbid_attrs(&source.attrs, true, &mut out);
    let mut visitor = DenyForbidVisitor { out: &mut out };
    visitor.visit_file(source);
    out
}

pub(crate) fn find_foreign_mod_allows(source: &syn::File) -> Vec<ForeignModAllowInfo> {
    let mut visitor = ForeignModAllowVisitor { out: Vec::new() };
    visitor.visit_file(source);
    visitor.out
}

pub(crate) fn find_include_macros(source: &syn::File) -> Vec<IncludeMacroInfo> {
    let mut visitor = IncludeMacroVisitor { out: Vec::new() };
    visitor.visit_file(source);
    visitor.out
}

pub(crate) fn find_cfg_attr_lint_policies(source: &syn::File) -> Vec<CfgAttrLintInfo> {
    let mut out = Vec::new();
    helpers::collect_cfg_attr_lint_policies(&source.attrs, &mut out);
    let mut visitor = CfgAttrPolicyVisitor { out: &mut out };
    visitor.visit_file(source);
    out
}

pub(crate) fn find_path_attrs(source: &syn::File) -> Vec<PathAttrInfo> {
    let mut visitor = PathAttrVisitor { out: Vec::new() };
    visitor.visit_file(source);
    visitor.out
}

pub(crate) fn find_public_result_error_types(source: &syn::File) -> Vec<PublicResultErrorInfo> {
    let reachable_types = collect_reachable_public_types(source);
    let mut visitor = PublicResultErrorVisitor {
        out: Vec::new(),
        public_module_stack: vec![true],
        module_path: Vec::new(),
        reachable_types,
    };
    visitor.visit_file(source);
    visitor.out
}

pub(crate) fn find_public_struct_field_bags(source: &syn::File) -> Vec<PublicStructFieldBagInfo> {
    let mut visitor = PublicStructFieldBagVisitor {
        out: Vec::new(),
        public_module_stack: vec![true],
    };
    visitor.visit_file(source);
    visitor.out
}

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

struct PathAttrVisitor {
    out: Vec<PathAttrInfo>,
}

struct PublicResultErrorVisitor {
    out: Vec<PublicResultErrorInfo>,
    public_module_stack: Vec<bool>,
    module_path: Vec<String>,
    reachable_types: BTreeSet<String>,
}

struct PublicStructFieldBagVisitor {
    out: Vec<PublicStructFieldBagInfo>,
    public_module_stack: Vec<bool>,
}

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

fn path_string_has_parent_segment(path: &str) -> bool {
    path.split('/').any(|segment| segment == "..")
        || path.split('\\').any(|segment| segment == "..")
}

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

impl PublicStructFieldBagVisitor {
    fn current_module_public(&self) -> bool {
        self.public_module_stack.last().copied().unwrap_or(true)
    }

    fn with_nested_module(&mut self, item_mod: &syn::ItemMod, visit: impl FnOnce(&mut Self)) {
        let next =
            self.current_module_public() && matches!(item_mod.vis, syn::Visibility::Public(_));
        self.public_module_stack.push(next);
        visit(self);
        let _ = self.public_module_stack.pop();
    }
}

impl<'source> Visit<'source> for PublicResultErrorVisitor {
    fn visit_item_mod(&mut self, item_mod: &'source syn::ItemMod) {
        self.with_nested_module(item_mod, |visitor| {
            syn::visit::visit_item_mod(visitor, item_mod);
        });
    }

    fn visit_item_fn(&mut self, item_fn: &'source syn::ItemFn) {
        if self.current_module_public() && matches!(item_fn.vis, syn::Visibility::Public(_)) {
            let syn::ReturnType::Type(_, ty) = &item_fn.sig.output else {
                return;
            };
            if let Some(kind) = analysis_helpers::result_error_kind(ty) {
                self.out.push(PublicResultErrorInfo {
                    line: helpers::span_line(item_fn.sig.ident.span()),
                    fn_name: item_fn.sig.ident.to_string(),
                    kind,
                });
            }
        }
        syn::visit::visit_item_fn(self, item_fn);
    }

    fn visit_item_impl(&mut self, item_impl: &'source syn::ItemImpl) {
        let impl_is_public = self.current_module_public()
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

    fn visit_item_trait(&mut self, item_trait: &'source syn::ItemTrait) {
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

impl<'source> Visit<'source> for PublicStructFieldBagVisitor {
    fn visit_item_mod(&mut self, item_mod: &'source syn::ItemMod) {
        self.with_nested_module(item_mod, |visitor| {
            syn::visit::visit_item_mod(visitor, item_mod);
        });
    }

    fn visit_item_struct(&mut self, item_struct: &'source syn::ItemStruct) {
        if self.current_module_public()
            && matches!(item_struct.vis, syn::Visibility::Public(_))
            && matches!(item_struct.fields, syn::Fields::Named(_))
        {
            let public_field_count = match &item_struct.fields {
                syn::Fields::Named(fields) => fields
                    .named
                    .iter()
                    .filter(|field| matches!(field.vis, syn::Visibility::Public(_)))
                    .count(),
                _ => 0,
            };
            if public_field_count > 0 {
                let all_fields_public = match &item_struct.fields {
                    syn::Fields::Named(fields) => fields
                        .named
                        .iter()
                        .all(|field| matches!(field.vis, syn::Visibility::Public(_))),
                    _ => false,
                };
                self.out.push(PublicStructFieldBagInfo {
                    line: helpers::span_line(item_struct.ident.span()),
                    struct_name: item_struct.ident.to_string(),
                    public_field_count,
                    all_fields_public,
                });
            }
        }
        syn::visit::visit_item_struct(self, item_struct);
    }
}

fn collect_reachable_public_types(source: &syn::File) -> BTreeSet<String> {
    let mut visitor = ReachablePublicTypeVisitor {
        public_module_stack: vec![true],
        module_path: Vec::new(),
        names: BTreeSet::new(),
    };
    visitor.visit_file(source);
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

impl<'source> Visit<'source> for ReachablePublicTypeVisitor {
    fn visit_item_mod(&mut self, item_mod: &'source syn::ItemMod) {
        self.with_nested_module(item_mod, |visitor| {
            syn::visit::visit_item_mod(visitor, item_mod);
        });
    }

    fn visit_item_struct(&mut self, item: &'source syn::ItemStruct) {
        self.maybe_record(&item.vis, &item.ident);
        syn::visit::visit_item_struct(self, item);
    }

    fn visit_item_enum(&mut self, item: &'source syn::ItemEnum) {
        self.maybe_record(&item.vis, &item.ident);
        syn::visit::visit_item_enum(self, item);
    }

    fn visit_item_union(&mut self, item: &'source syn::ItemUnion) {
        self.maybe_record(&item.vis, &item.ident);
        syn::visit::visit_item_union(self, item);
    }

    fn visit_item_type(&mut self, item: &'source syn::ItemType) {
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
            _ if segments.len() == 1 => segments.to_vec(),
            _ => {
                let mut resolved = current_module_path.to_vec();
                resolved.extend_from_slice(segments);
                resolved
            }
        }
    };

    Some(normalized.join("::"))
}
