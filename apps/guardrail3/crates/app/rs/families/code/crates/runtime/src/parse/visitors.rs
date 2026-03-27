use super::helpers::{
    attrs_have_allow_lint, is_cfg_test_attr, path_to_string, path_to_string_from_use_tree,
    span_line,
};
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

pub fn find_std_fs_import_lines(ast: &syn::File) -> Vec<usize> {
    let mut visitor = StdFsImportVisitor::default();
    visitor.visit_file(ast);
    visitor.out
}

pub fn find_inline_std_fs_call_lines(ast: &syn::File) -> Vec<usize> {
    let mut visitor = InlineStdFsVisitor {
        out: Vec::new(),
        in_cfg_test: false,
        in_allowed_scope: false,
    };
    visitor.visit_file(ast);
    visitor.out
}

pub fn find_std_fs_glob_import_lines(ast: &syn::File) -> Vec<usize> {
    let mut visitor = StdFsGlobImportVisitor::default();
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

fn use_tree_matches_std_fs(tree: &syn::UseTree) -> bool {
    match tree {
        syn::UseTree::Path(path) => {
            if path.ident != "std" {
                return false;
            }
            match &*path.tree {
                syn::UseTree::Path(inner) => inner.ident == "fs",
                syn::UseTree::Name(name) => name.ident == "fs",
                syn::UseTree::Rename(rename) => rename.ident == "fs",
                syn::UseTree::Group(group) => group
                    .items
                    .iter()
                    .any(use_tree_matches_std_fs_with_std_prefix),
                _ => false,
            }
        }
        syn::UseTree::Group(group) => group.items.iter().any(use_tree_matches_std_fs),
        _ => false,
    }
}

fn use_tree_matches_std_fs_with_std_prefix(tree: &syn::UseTree) -> bool {
    match tree {
        syn::UseTree::Path(path) => path.ident == "fs",
        syn::UseTree::Name(name) => name.ident == "fs",
        syn::UseTree::Rename(rename) => rename.ident == "fs",
        _ => false,
    }
}

fn use_tree_is_std_fs_glob(tree: &syn::UseTree) -> bool {
    match tree {
        syn::UseTree::Path(std_path) if std_path.ident == "std" => match &*std_path.tree {
            syn::UseTree::Path(fs_path) if fs_path.ident == "fs" => {
                fs_subtree_contains_glob(&fs_path.tree)
            }
            syn::UseTree::Group(group) => group
                .items
                .iter()
                .any(use_tree_is_std_fs_glob_with_std_prefix),
            _ => false,
        },
        syn::UseTree::Group(group) => group.items.iter().any(use_tree_is_std_fs_glob),
        _ => false,
    }
}

fn use_tree_is_std_fs_glob_with_std_prefix(tree: &syn::UseTree) -> bool {
    match tree {
        syn::UseTree::Path(fs_path) if fs_path.ident == "fs" => {
            fs_subtree_contains_glob(&fs_path.tree)
        }
        _ => false,
    }
}

fn fs_subtree_contains_glob(tree: &syn::UseTree) -> bool {
    match tree {
        syn::UseTree::Glob(_) => true,
        syn::UseTree::Group(group) => group.items.iter().any(fs_subtree_contains_glob),
        _ => false,
    }
}

struct ForbiddenMacroVisitor {
    out: Vec<(usize, String)>,
}

#[derive(Default)]
struct UnwrapExpectVisitor {
    out: Vec<(usize, String)>,
    in_cfg_test: bool,
    unwrap_allowed: bool,
    expect_allowed: bool,
}

#[derive(Default)]
struct StdFsImportVisitor {
    out: Vec<usize>,
    in_cfg_test: bool,
}

#[derive(Default)]
struct StdFsGlobImportVisitor {
    out: Vec<usize>,
    in_cfg_test: bool,
}

struct InlineStdFsVisitor {
    out: Vec<usize>,
    in_cfg_test: bool,
    in_allowed_scope: bool,
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
    fn save_and_apply(&mut self, attrs: &[syn::Attribute]) -> (bool, bool, bool) {
        let was = (self.in_cfg_test, self.unwrap_allowed, self.expect_allowed);
        self.in_cfg_test |= attrs.iter().any(is_cfg_test_attr);
        self.unwrap_allowed |= attrs_have_allow_lint(attrs, "unwrap_used");
        self.expect_allowed |= attrs_have_allow_lint(attrs, "expect_used");
        was
    }

    fn restore(&mut self, was: (bool, bool, bool)) {
        self.in_cfg_test = was.0;
        self.unwrap_allowed = was.1;
        self.expect_allowed = was.2;
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
            "unwrap" => self.in_cfg_test || self.unwrap_allowed,
            "expect" => self.in_cfg_test || self.expect_allowed,
            _ => true,
        };
        if !skip {
            self.out
                .push((span_line(method_call.method.span()), method));
        }
        syn::visit::visit_expr_method_call(self, method_call);
    }
}

impl InlineStdFsVisitor {
    fn path_is_std_fs_call(path: &syn::Path) -> bool {
        let mut segments = path
            .segments
            .iter()
            .map(|segment| segment.ident.to_string());
        matches!(
            (
                segments.next().as_deref(),
                segments.next().as_deref(),
                segments.next()
            ),
            (Some("std"), Some("fs"), Some(_))
        )
    }
}

impl<'ast> Visit<'ast> for InlineStdFsVisitor {
    fn visit_item_mod(&mut self, item_mod: &'ast syn::ItemMod) {
        let was = (self.in_cfg_test, self.in_allowed_scope);
        self.in_cfg_test |= item_mod.attrs.iter().any(is_cfg_test_attr);
        self.in_allowed_scope |= attrs_have_allow_lint(&item_mod.attrs, "disallowed_methods");
        syn::visit::visit_item_mod(self, item_mod);
        (self.in_cfg_test, self.in_allowed_scope) = was;
    }

    fn visit_item_fn(&mut self, item_fn: &'ast syn::ItemFn) {
        let was = (self.in_cfg_test, self.in_allowed_scope);
        self.in_cfg_test |= item_fn.attrs.iter().any(is_cfg_test_attr);
        self.in_allowed_scope |= attrs_have_allow_lint(&item_fn.attrs, "disallowed_methods");
        syn::visit::visit_item_fn(self, item_fn);
        (self.in_cfg_test, self.in_allowed_scope) = was;
    }

    fn visit_impl_item_fn(&mut self, item_fn: &'ast syn::ImplItemFn) {
        let was = (self.in_cfg_test, self.in_allowed_scope);
        self.in_cfg_test |= item_fn.attrs.iter().any(is_cfg_test_attr);
        self.in_allowed_scope |= attrs_have_allow_lint(&item_fn.attrs, "disallowed_methods");
        syn::visit::visit_impl_item_fn(self, item_fn);
        (self.in_cfg_test, self.in_allowed_scope) = was;
    }

    fn visit_local(&mut self, local: &'ast syn::Local) {
        let was = (self.in_cfg_test, self.in_allowed_scope);
        self.in_cfg_test |= local.attrs.iter().any(is_cfg_test_attr);
        self.in_allowed_scope |= attrs_have_allow_lint(&local.attrs, "disallowed_methods");
        syn::visit::visit_local(self, local);
        (self.in_cfg_test, self.in_allowed_scope) = was;
    }

    fn visit_expr_call(&mut self, expr_call: &'ast syn::ExprCall) {
        if !self.in_cfg_test && !self.in_allowed_scope {
            if let syn::Expr::Path(expr_path) = &*expr_call.func {
                if Self::path_is_std_fs_call(&expr_path.path) {
                    self.out.push(span_line(expr_path.path.span()));
                }
            }
        }
        syn::visit::visit_expr_call(self, expr_call);
    }
}

impl<'ast> Visit<'ast> for StdFsGlobImportVisitor {
    fn visit_item_mod(&mut self, item_mod: &'ast syn::ItemMod) {
        let was = self.in_cfg_test;
        self.in_cfg_test |= item_mod.attrs.iter().any(is_cfg_test_attr);
        syn::visit::visit_item_mod(self, item_mod);
        self.in_cfg_test = was;
    }

    fn visit_item_fn(&mut self, item_fn: &'ast syn::ItemFn) {
        let was = self.in_cfg_test;
        self.in_cfg_test |= item_fn.attrs.iter().any(is_cfg_test_attr);
        syn::visit::visit_item_fn(self, item_fn);
        self.in_cfg_test = was;
    }

    fn visit_impl_item_fn(&mut self, item_fn: &'ast syn::ImplItemFn) {
        let was = self.in_cfg_test;
        self.in_cfg_test |= item_fn.attrs.iter().any(is_cfg_test_attr);
        syn::visit::visit_impl_item_fn(self, item_fn);
        self.in_cfg_test = was;
    }

    fn visit_item_use(&mut self, use_item: &'ast syn::ItemUse) {
        if !self.in_cfg_test
            && !use_item.attrs.iter().any(is_cfg_test_attr)
            && use_tree_is_std_fs_glob(&use_item.tree)
        {
            self.out.push(span_line(use_item.span()));
        }
        syn::visit::visit_item_use(self, use_item);
    }
}

impl<'ast> Visit<'ast> for StdFsImportVisitor {
    fn visit_item_mod(&mut self, item_mod: &'ast syn::ItemMod) {
        let was = self.in_cfg_test;
        self.in_cfg_test |= item_mod.attrs.iter().any(is_cfg_test_attr);
        syn::visit::visit_item_mod(self, item_mod);
        self.in_cfg_test = was;
    }

    fn visit_item_fn(&mut self, item_fn: &'ast syn::ItemFn) {
        let was = self.in_cfg_test;
        self.in_cfg_test |= item_fn.attrs.iter().any(is_cfg_test_attr);
        syn::visit::visit_item_fn(self, item_fn);
        self.in_cfg_test = was;
    }

    fn visit_impl_item_fn(&mut self, item_fn: &'ast syn::ImplItemFn) {
        let was = self.in_cfg_test;
        self.in_cfg_test |= item_fn.attrs.iter().any(is_cfg_test_attr);
        syn::visit::visit_impl_item_fn(self, item_fn);
        self.in_cfg_test = was;
    }

    fn visit_item_use(&mut self, use_item: &'ast syn::ItemUse) {
        if !self.in_cfg_test
            && !use_item.attrs.iter().any(is_cfg_test_attr)
            && use_tree_matches_std_fs(&use_item.tree)
        {
            self.out.push(span_line(use_item.span()));
        }
        syn::visit::visit_item_use(self, use_item);
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
