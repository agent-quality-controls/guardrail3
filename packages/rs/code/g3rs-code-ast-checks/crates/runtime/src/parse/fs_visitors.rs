use std::collections::BTreeSet;

use syn::spanned::Spanned;
use syn::visit::Visit;

use super::helpers::{attrs_enter_test_context, span_line};

pub(crate) fn find_std_fs_import_lines(ast: &syn::File) -> Vec<usize> {
    let mut visitor = StdFsImportVisitor::default();
    visitor.visit_file(ast);
    visitor.out
}

pub(crate) fn find_inline_std_fs_call_lines(ast: &syn::File) -> Vec<usize> {
    let mut visitor = InlineStdFsVisitor::default();
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

trait TestContextAware {
    fn in_test_context_mut(&mut self) -> &mut bool;

    fn save_and_apply_test_context(&mut self, attrs: &[syn::Attribute]) -> bool {
        let was = *self.in_test_context_mut();
        *self.in_test_context_mut() |= attrs_enter_test_context(attrs);
        was
    }

    fn restore_test_context(&mut self, was: bool) {
        *self.in_test_context_mut() = was;
    }
}

#[derive(Default)]
struct StdFsImportVisitor {
    out: Vec<usize>,
    in_test_context: bool,
}

struct InlineStdFsVisitor {
    out: Vec<usize>,
    in_test_context: bool,
    std_aliases: BTreeSet<String>,
}

impl Default for InlineStdFsVisitor {
    fn default() -> Self {
        Self {
            out: Vec::new(),
            in_test_context: false,
            std_aliases: BTreeSet::from([String::from("std")]),
        }
    }
}

impl TestContextAware for StdFsImportVisitor {
    fn in_test_context_mut(&mut self) -> &mut bool {
        &mut self.in_test_context
    }
}

impl TestContextAware for InlineStdFsVisitor {
    fn in_test_context_mut(&mut self) -> &mut bool {
        &mut self.in_test_context
    }
}

impl InlineStdFsVisitor {
    fn path_is_std_fs_call(path: &syn::Path, std_aliases: &BTreeSet<String>) -> bool {
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
            (Some(first), Some("fs"), Some(_)) if std_aliases.contains(first)
        )
    }
}

fn collect_std_aliases(tree: &syn::UseTree, aliases: &mut BTreeSet<String>) {
    match tree {
        syn::UseTree::Rename(rename) if rename.ident == "std" => {
            let _ = aliases.insert(rename.rename.to_string());
        }
        syn::UseTree::Path(path) if path.ident == "std" => {
            collect_std_aliases_under_std(&path.tree, aliases);
        }
        syn::UseTree::Group(group) => {
            for item in &group.items {
                collect_std_aliases(item, aliases);
            }
        }
        _ => {}
    }
}

fn collect_std_aliases_under_std(tree: &syn::UseTree, aliases: &mut BTreeSet<String>) {
    match tree {
        syn::UseTree::Rename(rename) if rename.ident == "self" => {
            let _ = aliases.insert(rename.rename.to_string());
        }
        syn::UseTree::Group(group) => {
            for item in &group.items {
                collect_std_aliases_under_std(item, aliases);
            }
        }
        _ => {}
    }
}

fn collect_std_extern_crate_alias(item: &syn::ItemExternCrate, aliases: &mut BTreeSet<String>) {
    if item.ident != "std" {
        return;
    }
    if let Some((_, rename)) = &item.rename {
        let _ = aliases.insert(rename.to_string());
    } else {
        let _ = aliases.insert(String::from("std"));
    }
}

impl<'ast> Visit<'ast> for StdFsImportVisitor {
    fn visit_item_mod(&mut self, item_mod: &'ast syn::ItemMod) {
        let was = self.save_and_apply_test_context(&item_mod.attrs);
        syn::visit::visit_item_mod(self, item_mod);
        self.restore_test_context(was);
    }

    fn visit_item_use(&mut self, item_use: &'ast syn::ItemUse) {
        let was = self.save_and_apply_test_context(&item_use.attrs);
        if !self.in_test_context && use_tree_matches_std_fs(&item_use.tree) {
            self.out.push(span_line(item_use.span()));
        }
        syn::visit::visit_item_use(self, item_use);
        self.restore_test_context(was);
    }
}

impl<'ast> Visit<'ast> for InlineStdFsVisitor {
    fn visit_item_mod(&mut self, item_mod: &'ast syn::ItemMod) {
        let was = self.save_and_apply_test_context(&item_mod.attrs);
        syn::visit::visit_item_mod(self, item_mod);
        self.restore_test_context(was);
    }

    fn visit_item_fn(&mut self, item_fn: &'ast syn::ItemFn) {
        let was = self.save_and_apply_test_context(&item_fn.attrs);
        syn::visit::visit_item_fn(self, item_fn);
        self.restore_test_context(was);
    }

    fn visit_impl_item_fn(&mut self, item_fn: &'ast syn::ImplItemFn) {
        let was = self.save_and_apply_test_context(&item_fn.attrs);
        syn::visit::visit_impl_item_fn(self, item_fn);
        self.restore_test_context(was);
    }

    fn visit_trait_item_fn(&mut self, item_fn: &'ast syn::TraitItemFn) {
        let was = self.save_and_apply_test_context(&item_fn.attrs);
        syn::visit::visit_trait_item_fn(self, item_fn);
        self.restore_test_context(was);
    }

    fn visit_item_use(&mut self, item_use: &'ast syn::ItemUse) {
        let was = self.save_and_apply_test_context(&item_use.attrs);
        collect_std_aliases(&item_use.tree, &mut self.std_aliases);
        syn::visit::visit_item_use(self, item_use);
        self.restore_test_context(was);
    }

    fn visit_item_extern_crate(&mut self, item: &'ast syn::ItemExternCrate) {
        collect_std_extern_crate_alias(item, &mut self.std_aliases);
        syn::visit::visit_item_extern_crate(self, item);
    }

    fn visit_expr_path(&mut self, expr_path: &'ast syn::ExprPath) {
        if !self.in_test_context
            && Self::path_is_std_fs_call(&expr_path.path, &self.std_aliases)
        {
            self.out.push(span_line(expr_path.span()));
        }
        syn::visit::visit_expr_path(self, expr_path);
    }
}
