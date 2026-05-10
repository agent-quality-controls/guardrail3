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

use std::collections::BTreeSet;

use crate::parse::analysis_helpers;
use crate::parse::helpers;
use crate::parse::types::{AnyhowTypeBindings, PublicResultErrorInfo, PublicStructFieldBagInfo};
use syn::visit::Visit;

/// Implements `find public result error types`.
///
/// # Panics
/// Panics on assertion failure or unexpected input.
pub(crate) fn find_public_result_error_types(source: &syn::File) -> Vec<PublicResultErrorInfo> {
    let reachable_types = collect_reachable_public_types(source);
    let anyhow_bindings = collect_anyhow_type_bindings(&source.items);
    let mut visitor = PublicResultErrorVisitor {
        out: Vec::new(),
        public_module_stack: vec![true],
        module_path: Vec::new(),
        reachable_types,
        anyhow_bindings_stack: vec![anyhow_bindings],
    };
    visitor.visit_file(source);
    visitor.out
}

/// Implements `find public struct field bags`.
///
/// # Panics
/// Panics on assertion failure or unexpected input.
pub(crate) fn find_public_struct_field_bags(source: &syn::File) -> Vec<PublicStructFieldBagInfo> {
    let mut visitor = PublicStructFieldBagVisitor {
        out: Vec::new(),
        public_module_stack: vec![true],
        module_path: Vec::new(),
    };
    visitor.visit_file(source);
    visitor.out
}

/// Struct `PublicResultErrorVisitor` used by this module.
struct PublicResultErrorVisitor {
    /// Field `out`.
    out: Vec<PublicResultErrorInfo>,
    /// Field `public_module_stack`.
    public_module_stack: Vec<bool>,
    /// Field `module_path`.
    module_path: Vec<String>,
    /// Field `reachable_types`.
    reachable_types: BTreeSet<String>,
    /// Field `anyhow_bindings_stack`.
    anyhow_bindings_stack: Vec<AnyhowTypeBindings>,
}

/// Struct `PublicStructFieldBagVisitor` used by this module.
struct PublicStructFieldBagVisitor {
    /// Field `out`.
    out: Vec<PublicStructFieldBagInfo>,
    /// Field `public_module_stack`.
    public_module_stack: Vec<bool>,
    /// Field `module_path`.
    module_path: Vec<String>,
}

impl PublicResultErrorVisitor {
    /// Implements `current module public`.
    ///
    /// # Panics
    /// Panics on assertion failure or unexpected input.
    fn current_module_public(&self) -> bool {
        self.public_module_stack.last().copied().unwrap_or(true)
    }

    /// Implements `with nested module`.
    ///
    /// # Panics
    /// Panics on assertion failure or unexpected input.
    fn with_nested_module(&mut self, item_mod: &syn::ItemMod, visit: impl FnOnce(&mut Self)) {
        let next =
            self.current_module_public() && matches!(item_mod.vis, syn::Visibility::Public(_));
        self.public_module_stack.push(next);
        self.module_path.push(item_mod.ident.to_string());
        let nested_bindings = item_mod
            .content
            .as_ref()
            .map(|(_, items)| {
                merge_anyhow_type_bindings(
                    self.current_anyhow_bindings(),
                    collect_anyhow_type_bindings(items),
                )
            })
            .unwrap_or_else(|| self.current_anyhow_bindings().clone());
        self.anyhow_bindings_stack.push(nested_bindings);
        visit(self);
        let _ = self.anyhow_bindings_stack.pop();
        let _ = self.module_path.pop();
        let _ = self.public_module_stack.pop();
    }

    /// Implements `current anyhow bindings`.
    ///
    /// # Panics
    /// Panics on assertion failure or unexpected input.
    fn current_anyhow_bindings(&self) -> &AnyhowTypeBindings {
        self.anyhow_bindings_stack
            .last()
            .expect("anyhow bindings stack is always seeded")
    }

    /// Implements `reachable type name`.
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
    /// Implements `current module public`.
    fn current_module_public(&self) -> bool {
        self.public_module_stack.last().copied().unwrap_or(true)
    }

    /// Implements `with nested module`.
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
            if let Some(kind) =
                analysis_helpers::result_error_kind(ty, self.current_anyhow_bindings())
            {
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
                if let Some(kind) =
                    analysis_helpers::result_error_kind(ty, self.current_anyhow_bindings())
                {
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
                if let Some(kind) =
                    analysis_helpers::result_error_kind(ty, self.current_anyhow_bindings())
                {
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
                    qualified_name: qualified_public_item_name(
                        &self.module_path,
                        &item_struct.ident.to_string(),
                    ),
                    public_field_count,
                    all_fields_public,
                });
            }
        }
        syn::visit::visit_item_struct(self, item_struct);
    }
}

/// Implements `qualified public item name`.
fn qualified_public_item_name(module_path: &[String], item_name: &str) -> String {
    if module_path.is_empty() {
        item_name.to_owned()
    } else {
        format!("{}::{item_name}", module_path.join("::"))
    }
}

/// Implements `collect reachable public types`.
fn collect_reachable_public_types(source: &syn::File) -> BTreeSet<String> {
    let mut visitor = ReachablePublicTypeVisitor {
        public_module_stack: vec![true],
        module_path: Vec::new(),
        names: BTreeSet::new(),
    };
    visitor.visit_file(source);
    visitor.names
}

/// Struct `ReachablePublicTypeVisitor` used by this module.
struct ReachablePublicTypeVisitor {
    /// Field `public_module_stack`.
    public_module_stack: Vec<bool>,
    /// Field `module_path`.
    module_path: Vec<String>,
    /// Field `names`.
    names: BTreeSet<String>,
}

impl ReachablePublicTypeVisitor {
    /// Implements `current module public`.
    fn current_module_public(&self) -> bool {
        self.public_module_stack.last().copied().unwrap_or(true)
    }

    /// Implements `maybe record`.
    fn maybe_record(&mut self, vis: &syn::Visibility, ident: &syn::Ident) {
        if self.current_module_public() && matches!(vis, syn::Visibility::Public(_)) {
            let mut path = self.module_path.clone();
            path.push(ident.to_string());
            let _ = self.names.insert(path.join("::"));
        }
    }

    /// Implements `with nested module`.
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

/// Implements `normalize type path`.
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
                let _popped = resolved.pop()?;
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

/// Implements `collect anyhow type bindings`.
fn collect_anyhow_type_bindings(items: &[syn::Item]) -> AnyhowTypeBindings {
    let mut bindings = AnyhowTypeBindings::default();
    for item in items {
        let syn::Item::Use(item_use) = item else {
            continue;
        };
        collect_anyhow_bindings_from_use_tree(&item_use.tree, &mut Vec::new(), &mut bindings);
    }
    bindings
}

/// Implements `merge anyhow type bindings`.
fn merge_anyhow_type_bindings(
    parent: &AnyhowTypeBindings,
    local: AnyhowTypeBindings,
) -> AnyhowTypeBindings {
    let mut merged = parent.clone();
    merged.error_type_names.extend(local.error_type_names);
    merged.module_aliases.extend(local.module_aliases);
    merged
}

/// Implements `collect anyhow bindings from use tree`.
fn collect_anyhow_bindings_from_use_tree(
    tree: &syn::UseTree,
    prefix: &mut Vec<String>,
    bindings: &mut AnyhowTypeBindings,
) {
    match tree {
        syn::UseTree::Path(path) => {
            prefix.push(path.ident.to_string());
            collect_anyhow_bindings_from_use_tree(&path.tree, prefix, bindings);
            let _ = prefix.pop();
        }
        syn::UseTree::Name(name) => {
            let mut full = prefix.clone();
            full.push(name.ident.to_string());
            match full.as_slice() {
                [module] if module == "anyhow" => {
                    let _ = bindings.module_aliases.insert("anyhow".to_owned());
                }
                [module, ident] if module == "anyhow" && ident == "self" => {
                    let _ = bindings.module_aliases.insert("anyhow".to_owned());
                }
                [module, err] if module == "anyhow" && err == "Error" => {
                    let _ = bindings.error_type_names.insert("Error".to_owned());
                }
                _ => {}
            }
        }
        syn::UseTree::Rename(rename) => {
            let mut full = prefix.clone();
            full.push(rename.ident.to_string());
            match full.as_slice() {
                [module] if module == "anyhow" => {
                    let _ = bindings.module_aliases.insert(rename.rename.to_string());
                }
                [module, ident] if module == "anyhow" && ident == "self" => {
                    let _ = bindings.module_aliases.insert(rename.rename.to_string());
                }
                [module, err] if module == "anyhow" && err == "Error" => {
                    let _ = bindings.error_type_names.insert(rename.rename.to_string());
                }
                _ => {}
            }
        }
        syn::UseTree::Group(group) => {
            for item in &group.items {
                collect_anyhow_bindings_from_use_tree(item, prefix, bindings);
            }
        }
        syn::UseTree::Glob(_) => {}
    }
}
