use super::helpers::{is_cfg_test_attr, path_to_string, path_to_string_from_use_tree, span_line};
use super::types::{FacadeBodyItemInfo, LargeTypeItem as LargeTypeFact, TraitMethodCountInfo};
use syn::spanned::Spanned;
use syn::visit::Visit;

pub fn find_forbidden_macros(ast: &syn::File) -> Vec<(usize, String)> {
    let mut visitor = ForbiddenMacroVisitor { out: Vec::new() };
    visitor.visit_file(ast);
    visitor.out
}

pub fn find_unwrap_expect(ast: &syn::File) -> Vec<(usize, String)> {
    let mut visitor = UnwrapExpectVisitor::default();
    visitor.visit_file(ast);
    visitor.out
}

pub fn find_pub_use_glob_reexports(ast: &syn::File) -> Vec<(usize, String)> {
    ast.items
        .iter()
        .filter_map(|item| {
            let syn::Item::Use(use_item) = item else {
                return None;
            };
            if !matches!(use_item.vis, syn::Visibility::Public(_)) {
                return None;
            }
            glob_reexport_target(&use_item.tree).map(|target| (span_line(use_item.span()), target))
        })
        .collect()
}

fn glob_reexport_target(tree: &syn::UseTree) -> Option<String> {
    match tree {
        syn::UseTree::Path(path) => {
            glob_reexport_target(&path.tree).map(|target| format!("{}::{target}", path.ident))
        }
        syn::UseTree::Glob(_) => Some("*".to_owned()),
        _ => None,
    }
}

pub fn find_facade_body_items(ast: &syn::File) -> Vec<FacadeBodyItemInfo> {
    ast.items
        .iter()
        .filter_map(|item| match item {
            syn::Item::Fn(item_fn) => Some(FacadeBodyItemInfo {
                line: span_line(item_fn.sig.ident.span()),
                kind: "function",
                name: item_fn.sig.ident.to_string(),
            }),
            syn::Item::Impl(item_impl) => Some(FacadeBodyItemInfo {
                line: span_line(item_impl.impl_token.span()),
                kind: "impl",
                name: "impl".to_owned(),
            }),
            syn::Item::Use(item_use) if !matches!(item_use.vis, syn::Visibility::Public(_)) => {
                Some(FacadeBodyItemInfo {
                    line: span_line(item_use.span()),
                    kind: "private use",
                    name: path_to_string_from_use_tree(&item_use.tree),
                })
            }
            syn::Item::ExternCrate(item) => Some(FacadeBodyItemInfo {
                line: span_line(item.span()),
                kind: "extern crate",
                name: item.ident.to_string(),
            }),
            syn::Item::Static(item) => Some(FacadeBodyItemInfo {
                line: span_line(item.ident.span()),
                kind: "static",
                name: item.ident.to_string(),
            }),
            syn::Item::ForeignMod(item) => Some(FacadeBodyItemInfo {
                line: span_line(item.abi.extern_token.span()),
                kind: "extern block",
                name: "extern".to_owned(),
            }),
            syn::Item::Macro(item) => Some(FacadeBodyItemInfo {
                line: span_line(item.mac.path.span()),
                kind: "macro item",
                name: path_to_string(&item.mac.path),
            }),
            _ => None,
        })
        .collect()
}

pub fn find_inline_public_modules(ast: &syn::File) -> Vec<(usize, String)> {
    ast.items
        .iter()
        .filter_map(|item| {
            let syn::Item::Mod(item_mod) = item else {
                return None;
            };
            if item_mod.content.is_none() || !matches!(item_mod.vis, syn::Visibility::Public(_)) {
                return None;
            }
            Some((span_line(item_mod.ident.span()), item_mod.ident.to_string()))
        })
        .collect()
}

pub fn find_large_type_items(ast: &syn::File) -> Vec<LargeTypeFact> {
    let mut visitor = LargeTypeVisitor { out: Vec::new() };
    visitor.visit_file(ast);
    visitor.out
}

pub fn find_large_traits(ast: &syn::File) -> Vec<TraitMethodCountInfo> {
    let mut visitor = LargeTraitVisitor { out: Vec::new() };
    visitor.visit_file(ast);
    visitor.out
}

struct ForbiddenMacroVisitor {
    out: Vec<(usize, String)>,
}

#[derive(Default)]
struct UnwrapExpectVisitor {
    out: Vec<(usize, String)>,
    in_cfg_test: bool,
}

struct LargeTypeVisitor {
    out: Vec<LargeTypeFact>,
}

struct LargeTraitVisitor {
    out: Vec<TraitMethodCountInfo>,
}

impl<'ast> Visit<'ast> for ForbiddenMacroVisitor {
    fn visit_macro(&mut self, macro_call: &'ast syn::Macro) {
        let name = path_to_string(&macro_call.path);
        let base = name.rsplit("::").next().unwrap_or(&name);
        if matches!(base, "todo" | "unimplemented" | "unreachable" | "panic") {
            self.out.push((span_line(macro_call.path.span()), name));
        }
        syn::visit::visit_macro(self, macro_call);
    }
}

impl UnwrapExpectVisitor {
    fn save_and_apply(&mut self, attrs: &[syn::Attribute]) -> bool {
        let was = self.in_cfg_test;
        self.in_cfg_test |= attrs.iter().any(is_cfg_test_attr);
        was
    }

    fn restore(&mut self, was: bool) {
        self.in_cfg_test = was;
    }
}

impl<'ast> Visit<'ast> for UnwrapExpectVisitor {
    fn visit_item_fn(&mut self, item_fn: &'ast syn::ItemFn) {
        let was = self.save_and_apply(&item_fn.attrs);
        syn::visit::visit_item_fn(self, item_fn);
        self.restore(was);
    }

    fn visit_impl_item_fn(&mut self, item_fn: &'ast syn::ImplItemFn) {
        let was = self.save_and_apply(&item_fn.attrs);
        syn::visit::visit_impl_item_fn(self, item_fn);
        self.restore(was);
    }

    fn visit_item_mod(&mut self, item_mod: &'ast syn::ItemMod) {
        let was = self.save_and_apply(&item_mod.attrs);
        syn::visit::visit_item_mod(self, item_mod);
        self.restore(was);
    }

    fn visit_local(&mut self, local: &'ast syn::Local) {
        let was = self.save_and_apply(&local.attrs);
        syn::visit::visit_local(self, local);
        self.restore(was);
    }

    fn visit_expr_method_call(&mut self, method_call: &'ast syn::ExprMethodCall) {
        let method = method_call.method.to_string();
        let skip = match method.as_str() {
            "unwrap" | "expect" => self.in_cfg_test,
            _ => true,
        };
        if !skip {
            self.out
                .push((span_line(method_call.method.span()), method));
        }
        syn::visit::visit_expr_method_call(self, method_call);
    }

    fn visit_expr_call(&mut self, expr_call: &'ast syn::ExprCall) {
        if !self.in_cfg_test {
            if let syn::Expr::Path(expr_path) = &*expr_call.func {
                let mut segments = expr_path.path.segments.iter();
                let first = segments.next();
                let second = segments.next();
                if first.is_some() && second.is_some() {
                    let method = expr_path
                        .path
                        .segments
                        .last()
                        .map(|segment| segment.ident.to_string());
                    if let Some(method) = method {
                        if matches!(method.as_str(), "unwrap" | "expect") {
                            self.out.push((span_line(expr_path.path.span()), method));
                        }
                    }
                }
            }
        }
        syn::visit::visit_expr_call(self, expr_call);
    }
}


impl<'ast> Visit<'ast> for LargeTypeVisitor {
    fn visit_item_struct(&mut self, item_struct: &'ast syn::ItemStruct) {
        let field_count = match &item_struct.fields {
            syn::Fields::Named(fields) => fields.named.len(),
            syn::Fields::Unnamed(fields) => fields.unnamed.len(),
            syn::Fields::Unit => 0,
        };
        if field_count > 15 {
            self.out.push(LargeTypeFact::Struct {
                line: span_line(item_struct.ident.span()),
                name: item_struct.ident.to_string(),
                field_count,
            });
        }
        syn::visit::visit_item_struct(self, item_struct);
    }

    fn visit_item_enum(&mut self, item_enum: &'ast syn::ItemEnum) {
        let variant_count = item_enum.variants.len();
        if variant_count > 20 {
            self.out.push(LargeTypeFact::Enum {
                line: span_line(item_enum.ident.span()),
                name: item_enum.ident.to_string(),
                variant_count,
            });
        }
        syn::visit::visit_item_enum(self, item_enum);
    }
}

impl<'ast> Visit<'ast> for LargeTraitVisitor {
    fn visit_item_trait(&mut self, item_trait: &'ast syn::ItemTrait) {
        let method_count = item_trait
            .items
            .iter()
            .filter(|item| matches!(item, syn::TraitItem::Fn(_)))
            .count();
        if method_count > 8 {
            self.out.push(TraitMethodCountInfo {
                line: span_line(item_trait.ident.span()),
                trait_name: item_trait.ident.to_string(),
                method_count,
            });
        }
        syn::visit::visit_item_trait(self, item_trait);
    }
}
