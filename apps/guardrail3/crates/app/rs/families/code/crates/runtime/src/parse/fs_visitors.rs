use std::collections::BTreeSet;

use syn::spanned::Spanned;
use syn::visit::Visit;

use super::helpers::{is_cfg_test_attr, span_line};

pub fn find_std_fs_import_lines(ast: &syn::File) -> Vec<usize> {
    let mut visitor = StdFsImportVisitor::default();
    visitor.visit_file(ast);
    visitor.out
}

pub fn find_inline_std_fs_call_lines(ast: &syn::File) -> Vec<usize> {
    let mut visitor = InlineStdFsVisitor::default();
    visitor.visit_file(ast);
    visitor.out
}

pub fn find_std_fs_glob_import_lines(ast: &syn::File) -> Vec<usize> {
    let mut visitor = StdFsGlobImportVisitor::default();
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

fn use_tree_is_std_fs_glob(tree: &syn::UseTree, std_aliases: &BTreeSet<String>) -> bool {
    match tree {
        syn::UseTree::Path(std_path) if std_aliases.contains(&std_path.ident.to_string()) => {
            match &*std_path.tree {
                syn::UseTree::Path(fs_path) if fs_path.ident == "fs" => {
                    fs_subtree_contains_glob(&fs_path.tree)
                }
                syn::UseTree::Group(group) => group
                    .items
                    .iter()
                    .any(use_tree_is_std_fs_glob_with_std_prefix),
                _ => false,
            }
        }
        syn::UseTree::Group(group) => group
            .items
            .iter()
            .any(|item| use_tree_is_std_fs_glob(item, std_aliases)),
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

struct StdFsImportVisitor {
    out: Vec<usize>,
    in_cfg_test: bool,
    std_aliases: BTreeSet<String>,
}

struct StdFsGlobImportVisitor {
    out: Vec<usize>,
    in_cfg_test: bool,
    std_aliases: BTreeSet<String>,
}

struct InlineStdFsVisitor {
    out: Vec<usize>,
    in_cfg_test: bool,
    std_aliases: BTreeSet<String>,
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

impl Default for InlineStdFsVisitor {
    fn default() -> Self {
        Self {
            out: Vec::new(),
            in_cfg_test: false,
            std_aliases: BTreeSet::from([String::from("std")]),
        }
    }
}

impl Default for StdFsImportVisitor {
    fn default() -> Self {
        Self {
            out: Vec::new(),
            in_cfg_test: false,
            std_aliases: BTreeSet::from([String::from("std")]),
        }
    }
}

impl Default for StdFsGlobImportVisitor {
    fn default() -> Self {
        Self {
            out: Vec::new(),
            in_cfg_test: false,
            std_aliases: BTreeSet::from([String::from("std")]),
        }
    }
}

impl<'ast> Visit<'ast> for InlineStdFsVisitor {
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

    fn visit_local(&mut self, local: &'ast syn::Local) {
        let was = self.in_cfg_test;
        self.in_cfg_test |= local.attrs.iter().any(is_cfg_test_attr);
        syn::visit::visit_local(self, local);
        self.in_cfg_test = was;
    }

    fn visit_item_use(&mut self, use_item: &'ast syn::ItemUse) {
        collect_std_aliases(&use_item.tree, &mut self.std_aliases);
        syn::visit::visit_item_use(self, use_item);
    }

    fn visit_item_extern_crate(&mut self, item: &'ast syn::ItemExternCrate) {
        collect_std_extern_crate_alias(item, &mut self.std_aliases);
        syn::visit::visit_item_extern_crate(self, item);
    }

    fn visit_expr_call(&mut self, expr_call: &'ast syn::ExprCall) {
        if !self.in_cfg_test {
            if let syn::Expr::Path(expr_path) = &*expr_call.func {
                if Self::path_is_std_fs_call(&expr_path.path, &self.std_aliases) {
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
        collect_std_aliases(&use_item.tree, &mut self.std_aliases);
        if !self.in_cfg_test
            && !use_item.attrs.iter().any(is_cfg_test_attr)
            && use_tree_is_std_fs_glob(&use_item.tree, &self.std_aliases)
        {
            self.out.push(span_line(use_item.span()));
        }
        syn::visit::visit_item_use(self, use_item);
    }

    fn visit_item_extern_crate(&mut self, item: &'ast syn::ItemExternCrate) {
        collect_std_extern_crate_alias(item, &mut self.std_aliases);
        syn::visit::visit_item_extern_crate(self, item);
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
        collect_std_aliases(&use_item.tree, &mut self.std_aliases);
        if !self.in_cfg_test
            && !use_item.attrs.iter().any(is_cfg_test_attr)
            && use_tree_matches_std_fs(&use_item.tree)
        {
            self.out.push(span_line(use_item.span()));
        }
        syn::visit::visit_item_use(self, use_item);
    }

    fn visit_item_extern_crate(&mut self, item: &'ast syn::ItemExternCrate) {
        collect_std_extern_crate_alias(item, &mut self.std_aliases);
        syn::visit::visit_item_extern_crate(self, item);
    }
}
