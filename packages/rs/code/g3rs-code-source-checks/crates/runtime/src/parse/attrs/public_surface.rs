use std::collections::BTreeSet;

use crate::parse::analysis_helpers;
use crate::parse::helpers;
use crate::parse::types::{PublicResultErrorInfo, PublicStructFieldBagInfo};
use syn::visit::Visit;

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
