use super::helpers::{path_to_string, path_to_string_from_use_tree, span_line};
use super::types::{
    FacadeBodyItemInfo, LargeTypeItem as LargeTypeFact, TestExpectCallInfo, TraitMethodCountInfo,
};
use syn::parse::Parser;
use syn::spanned::Spanned;
use syn::visit::Visit;

pub fn find_forbidden_macros(ast: &syn::File) -> Vec<(usize, String)> {
    let mut visitor = ForbiddenMacroVisitor { out: Vec::new() };
    visitor.visit_file(ast);
    visitor.out
}

pub fn find_test_expect_calls(ast: &syn::File, file_is_test: bool) -> Vec<TestExpectCallInfo> {
    let mut visitor = TestExpectVisitor {
        out: Vec::new(),
        in_test_context: file_is_test,
    };
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
            let targets = glob_reexport_targets(&use_item.tree);
            if targets.is_empty() {
                None
            } else {
                Some(
                    targets
                        .into_iter()
                        .map(|target| (span_line(use_item.span()), target))
                        .collect::<Vec<_>>(),
                )
            }
        })
        .flatten()
        .collect()
}

fn glob_reexport_targets(tree: &syn::UseTree) -> Vec<String> {
    match tree {
        syn::UseTree::Path(path) => glob_reexport_targets(&path.tree)
            .into_iter()
            .map(|target| format!("{}::{target}", path.ident))
            .collect(),
        syn::UseTree::Glob(_) => vec!["*".to_owned()],
        syn::UseTree::Group(group) => group.items.iter().flat_map(glob_reexport_targets).collect(),
        _ => Vec::new(),
    }
}

pub fn find_facade_body_items(ast: &syn::File) -> Vec<FacadeBodyItemInfo> {
    ast.items
        .iter()
        .filter_map(|item| match item {
            syn::Item::Mod(item_mod) if item_mod.content.is_some() => Some(FacadeBodyItemInfo {
                line: span_line(item_mod.ident.span()),
                kind: "inline module",
                name: item_mod.ident.to_string(),
            }),
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

struct TestExpectVisitor {
    out: Vec<TestExpectCallInfo>,
    in_test_context: bool,
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

impl TestExpectVisitor {
    fn save_and_apply(&mut self, attrs: &[syn::Attribute]) -> bool {
        let was = self.in_test_context;
        self.in_test_context |= attrs.iter().any(super::helpers::is_cfg_test_attr);
        was
    }

    fn restore(&mut self, was: bool) {
        self.in_test_context = was;
    }

    fn push_expect_call(
        &mut self,
        line: usize,
        args: &syn::punctuated::Punctuated<syn::Expr, syn::Token![,]>,
    ) {
        if !self.in_test_context {
            return;
        }
        let message = args.first().and_then(extract_expect_message);
        self.out.push(TestExpectCallInfo { line, message });
    }
}

fn extract_expect_message(expr: &syn::Expr) -> Option<String> {
    match expr {
        syn::Expr::Lit(expr_lit) => match &expr_lit.lit {
            syn::Lit::Str(lit) => Some(lit.value()),
            _ => None,
        },
        syn::Expr::Macro(expr_macro) if expr_macro.mac.path.is_ident("concat") => {
            let parser = syn::punctuated::Punctuated::<syn::Expr, syn::Token![,]>::parse_terminated;
            let args = parser.parse2(expr_macro.mac.tokens.clone()).ok()?;
            let mut out = String::new();
            for arg in args {
                let syn::Expr::Lit(expr_lit) = arg else {
                    return None;
                };
                let syn::Lit::Str(lit) = expr_lit.lit else {
                    return None;
                };
                out.push_str(&lit.value());
            }
            Some(out)
        }
        _ => None,
    }
}

impl<'ast> Visit<'ast> for TestExpectVisitor {
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

    fn visit_trait_item_fn(&mut self, item_fn: &'ast syn::TraitItemFn) {
        let was = self.save_and_apply(&item_fn.attrs);
        syn::visit::visit_trait_item_fn(self, item_fn);
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
        if method_call.method == "expect" {
            self.push_expect_call(span_line(method_call.method.span()), &method_call.args);
        }
        syn::visit::visit_expr_method_call(self, method_call);
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
