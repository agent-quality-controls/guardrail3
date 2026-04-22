use std::collections::BTreeSet;

use syn::spanned::Spanned;
use syn::visit::Visit;

use super::helpers::{attrs_enter_test_context, span_line};

pub(crate) fn find_std_fs_import_lines(source: &syn::File) -> Vec<usize> {
    let mut visitor = StdFsImportVisitor::default();
    visitor.visit_file(source);
    visitor.out
}

pub(crate) fn find_inline_std_fs_call_lines(source: &syn::File) -> Vec<usize> {
    let mut visitor = InlineStdFsVisitor::default();
    visitor.visit_file(source);
    visitor.out
}

pub(crate) fn find_std_fs_glob_import_lines(source: &syn::File) -> Vec<usize> {
    let mut visitor = StdFsGlobImportVisitor::default();
    visitor.visit_file(source);
    visitor.out
}

fn use_tree_matches_std_fs(
    tree: &syn::UseTree,
    std_aliases: &BTreeSet<String>,
    fs_aliases: &BTreeSet<String>,
) -> bool {
    match tree {
        syn::UseTree::Path(path)
            if path.ident == "std" || std_aliases.contains(path.ident.to_string().as_str()) =>
        {
            match &*path.tree {
                syn::UseTree::Path(inner) => {
                    inner.ident == "fs" || fs_aliases.contains(inner.ident.to_string().as_str())
                }
                syn::UseTree::Name(name) => {
                    name.ident == "fs" || fs_aliases.contains(name.ident.to_string().as_str())
                }
                syn::UseTree::Rename(rename) => {
                    rename.ident == "fs" || fs_aliases.contains(rename.ident.to_string().as_str())
                }
                syn::UseTree::Group(group) => group
                    .items
                    .iter()
                    .any(|item| use_tree_matches_std_fs_with_std_prefix(item, fs_aliases)),
                _ => false,
            }
        }
        syn::UseTree::Path(path)
            if fs_aliases.contains(path.ident.to_string().as_str()) =>
        {
            true
        }
        syn::UseTree::Group(group) => group
            .items
            .iter()
            .any(|item| use_tree_matches_std_fs(item, std_aliases, fs_aliases)),
        _ => false,
    }
}

fn use_tree_matches_std_fs_with_std_prefix(
    tree: &syn::UseTree,
    fs_aliases: &BTreeSet<String>,
) -> bool {
    match tree {
        syn::UseTree::Path(path) => {
            path.ident == "fs" || fs_aliases.contains(path.ident.to_string().as_str())
        }
        syn::UseTree::Name(name) => {
            name.ident == "fs" || fs_aliases.contains(name.ident.to_string().as_str())
        }
        syn::UseTree::Rename(rename) => {
            rename.ident == "fs" || fs_aliases.contains(rename.ident.to_string().as_str())
        }
        _ => false,
    }
}

fn use_tree_is_std_fs_glob(
    tree: &syn::UseTree,
    std_aliases: &BTreeSet<String>,
    fs_aliases: &BTreeSet<String>,
) -> bool {
    match tree {
        syn::UseTree::Path(std_path)
            if std_path.ident == "std"
                || std_aliases.contains(std_path.ident.to_string().as_str()) =>
        {
            match &*std_path.tree {
                syn::UseTree::Path(fs_path)
                    if fs_path.ident == "fs"
                        || fs_aliases.contains(fs_path.ident.to_string().as_str()) =>
                {
                    fs_subtree_contains_glob(&fs_path.tree)
                }
                syn::UseTree::Group(group) => group
                    .items
                    .iter()
                    .any(|item| use_tree_is_std_fs_glob_with_std_prefix(item, fs_aliases)),
                _ => false,
            }
        }
        syn::UseTree::Path(fs_path)
            if fs_aliases.contains(fs_path.ident.to_string().as_str()) =>
        {
            fs_subtree_contains_glob(&fs_path.tree)
        }
        syn::UseTree::Group(group) => group
            .items
            .iter()
            .any(|item| use_tree_is_std_fs_glob(item, std_aliases, fs_aliases)),
        _ => false,
    }
}

fn use_tree_is_std_fs_glob_with_std_prefix(
    tree: &syn::UseTree,
    fs_aliases: &BTreeSet<String>,
) -> bool {
    match tree {
        syn::UseTree::Path(fs_path)
            if fs_path.ident == "fs" || fs_aliases.contains(fs_path.ident.to_string().as_str()) =>
        {
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

struct StdFsImportVisitor {
    out: Vec<usize>,
    in_test_context: bool,
    std_aliases: BTreeSet<String>,
    fs_aliases: BTreeSet<String>,
}

struct StdFsGlobImportVisitor {
    out: Vec<usize>,
    in_test_context: bool,
    std_aliases: BTreeSet<String>,
    fs_aliases: BTreeSet<String>,
}

struct InlineStdFsVisitor {
    out: Vec<usize>,
    in_test_context: bool,
    std_aliases: BTreeSet<String>,
    fs_aliases: BTreeSet<String>,
}

impl TestContextAware for StdFsImportVisitor {
    fn in_test_context_mut(&mut self) -> &mut bool {
        &mut self.in_test_context
    }
}

impl TestContextAware for StdFsGlobImportVisitor {
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
    fn path_is_std_fs_call(
        path: &syn::Path,
        std_aliases: &BTreeSet<String>,
        fs_aliases: &BTreeSet<String>,
    ) -> bool {
        let segments = path
            .segments
            .iter()
            .map(|segment| segment.ident.to_string())
            .collect::<Vec<_>>();
        match segments.as_slice() {
            [first, second, ..] if second == "fs" && std_aliases.contains(first) => true,
            [first, ..] if fs_aliases.contains(first) && segments.len() > 1 => true,
            _ => false,
        }
    }
}

fn collect_std_aliases(
    tree: &syn::UseTree,
    std_aliases: &mut BTreeSet<String>,
    fs_aliases: &mut BTreeSet<String>,
) {
    match tree {
        syn::UseTree::Rename(rename)
            if rename.ident == "std"
                || std_aliases.contains(rename.ident.to_string().as_str()) =>
        {
            let _ = std_aliases.insert(rename.rename.to_string());
        }
        syn::UseTree::Rename(rename)
            if rename.ident == "fs"
                || fs_aliases.contains(rename.ident.to_string().as_str()) =>
        {
            let _ = fs_aliases.insert(rename.rename.to_string());
        }
        syn::UseTree::Path(path)
            if path.ident == "std" || std_aliases.contains(path.ident.to_string().as_str()) =>
        {
            collect_std_aliases_under_std(&path.tree, std_aliases, fs_aliases);
        }
        syn::UseTree::Path(path)
            if path.ident == "fs" || fs_aliases.contains(path.ident.to_string().as_str()) =>
        {
            collect_std_aliases_under_fs(&path.tree, fs_aliases);
        }
        syn::UseTree::Group(group) => {
            for item in &group.items {
                collect_std_aliases(item, std_aliases, fs_aliases);
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

fn collect_std_aliases_under_std(
    tree: &syn::UseTree,
    std_aliases: &mut BTreeSet<String>,
    fs_aliases: &mut BTreeSet<String>,
) {
    match tree {
        syn::UseTree::Rename(rename)
            if rename.ident == "self"
                || std_aliases.contains(rename.ident.to_string().as_str()) =>
        {
            let _ = std_aliases.insert(rename.rename.to_string());
        }
        syn::UseTree::Rename(rename)
            if rename.ident == "fs"
                || fs_aliases.contains(rename.ident.to_string().as_str()) =>
        {
            let _ = fs_aliases.insert(rename.rename.to_string());
        }
        syn::UseTree::Path(path)
            if path.ident == "self" || std_aliases.contains(path.ident.to_string().as_str()) =>
        {
            collect_std_aliases_under_std(&path.tree, std_aliases, fs_aliases);
        }
        syn::UseTree::Path(path)
            if path.ident == "fs" || fs_aliases.contains(path.ident.to_string().as_str()) =>
        {
            collect_std_aliases_under_fs(&path.tree, fs_aliases);
        }
        syn::UseTree::Group(group) => {
            for item in &group.items {
                collect_std_aliases_under_std(item, std_aliases, fs_aliases);
            }
        }
        _ => {}
    }
}

fn collect_std_aliases_under_fs(tree: &syn::UseTree, fs_aliases: &mut BTreeSet<String>) {
    match tree {
        syn::UseTree::Rename(rename)
            if rename.ident == "fs" || fs_aliases.contains(rename.ident.to_string().as_str()) =>
        {
            let _ = fs_aliases.insert(rename.rename.to_string());
        }
        syn::UseTree::Path(path)
            if path.ident == "fs" || fs_aliases.contains(path.ident.to_string().as_str()) =>
        {
            collect_std_aliases_under_fs(&path.tree, fs_aliases);
        }
        syn::UseTree::Group(group) => {
            for item in &group.items {
                collect_std_aliases_under_fs(item, fs_aliases);
            }
        }
        _ => {}
    }
}

impl Default for InlineStdFsVisitor {
    fn default() -> Self {
        Self {
            out: Vec::new(),
            in_test_context: false,
            std_aliases: BTreeSet::from([String::from("std")]),
            fs_aliases: BTreeSet::new(),
        }
    }
}

impl Default for StdFsImportVisitor {
    fn default() -> Self {
        Self {
            out: Vec::new(),
            in_test_context: false,
            std_aliases: BTreeSet::from([String::from("std")]),
            fs_aliases: BTreeSet::new(),
        }
    }
}

impl Default for StdFsGlobImportVisitor {
    fn default() -> Self {
        Self {
            out: Vec::new(),
            in_test_context: false,
            std_aliases: BTreeSet::from([String::from("std")]),
            fs_aliases: BTreeSet::new(),
        }
    }
}

impl<'source> Visit<'source> for InlineStdFsVisitor {
    fn visit_item_mod(&mut self, item_mod: &'source syn::ItemMod) {
        let was = self.save_and_apply_test_context(&item_mod.attrs);
        let std_aliases = self.std_aliases.clone();
        let fs_aliases = self.fs_aliases.clone();
        syn::visit::visit_item_mod(self, item_mod);
        self.std_aliases = std_aliases;
        self.fs_aliases = fs_aliases;
        self.restore_test_context(was);
    }

    fn visit_item_fn(&mut self, item_fn: &'source syn::ItemFn) {
        let was = self.save_and_apply_test_context(&item_fn.attrs);
        syn::visit::visit_item_fn(self, item_fn);
        self.restore_test_context(was);
    }

    fn visit_impl_item_fn(&mut self, item_fn: &'source syn::ImplItemFn) {
        let was = self.save_and_apply_test_context(&item_fn.attrs);
        syn::visit::visit_impl_item_fn(self, item_fn);
        self.restore_test_context(was);
    }

    fn visit_trait_item_fn(&mut self, item_fn: &'source syn::TraitItemFn) {
        let was = self.save_and_apply_test_context(&item_fn.attrs);
        syn::visit::visit_trait_item_fn(self, item_fn);
        self.restore_test_context(was);
    }

    fn visit_local(&mut self, local: &'source syn::Local) {
        let was = self.save_and_apply_test_context(&local.attrs);
        syn::visit::visit_local(self, local);
        self.restore_test_context(was);
    }

    fn visit_item_use(&mut self, use_item: &'source syn::ItemUse) {
        let was = self.save_and_apply_test_context(&use_item.attrs);
        collect_std_aliases(&use_item.tree, &mut self.std_aliases, &mut self.fs_aliases);
        syn::visit::visit_item_use(self, use_item);
        self.restore_test_context(was);
    }

    fn visit_item_extern_crate(&mut self, item: &'source syn::ItemExternCrate) {
        collect_std_extern_crate_alias(item, &mut self.std_aliases);
        syn::visit::visit_item_extern_crate(self, item);
    }

    fn visit_expr_call(&mut self, expr_call: &'source syn::ExprCall) {
        if !self.in_test_context {
            if let syn::Expr::Path(expr_path) = &*expr_call.func {
                if Self::path_is_std_fs_call(
                    &expr_path.path,
                    &self.std_aliases,
                    &self.fs_aliases,
                ) {
                    self.out.push(span_line(expr_path.path.span()));
                }
            }
        }
        syn::visit::visit_expr_call(self, expr_call);
    }
}

impl<'source> Visit<'source> for StdFsGlobImportVisitor {
    fn visit_item_mod(&mut self, item_mod: &'source syn::ItemMod) {
        let was = self.save_and_apply_test_context(&item_mod.attrs);
        let std_aliases = self.std_aliases.clone();
        let fs_aliases = self.fs_aliases.clone();
        syn::visit::visit_item_mod(self, item_mod);
        self.std_aliases = std_aliases;
        self.fs_aliases = fs_aliases;
        self.restore_test_context(was);
    }

    fn visit_item_fn(&mut self, item_fn: &'source syn::ItemFn) {
        let was = self.save_and_apply_test_context(&item_fn.attrs);
        syn::visit::visit_item_fn(self, item_fn);
        self.restore_test_context(was);
    }

    fn visit_impl_item_fn(&mut self, item_fn: &'source syn::ImplItemFn) {
        let was = self.save_and_apply_test_context(&item_fn.attrs);
        syn::visit::visit_impl_item_fn(self, item_fn);
        self.restore_test_context(was);
    }

    fn visit_trait_item_fn(&mut self, item_fn: &'source syn::TraitItemFn) {
        let was = self.save_and_apply_test_context(&item_fn.attrs);
        syn::visit::visit_trait_item_fn(self, item_fn);
        self.restore_test_context(was);
    }

    fn visit_local(&mut self, local: &'source syn::Local) {
        let was = self.save_and_apply_test_context(&local.attrs);
        syn::visit::visit_local(self, local);
        self.restore_test_context(was);
    }

    fn visit_item_use(&mut self, use_item: &'source syn::ItemUse) {
        let was = self.save_and_apply_test_context(&use_item.attrs);
        collect_std_aliases(&use_item.tree, &mut self.std_aliases, &mut self.fs_aliases);
        if !self.in_test_context
            && use_tree_is_std_fs_glob(&use_item.tree, &self.std_aliases, &self.fs_aliases)
        {
            self.out.push(span_line(use_item.span()));
        }
        syn::visit::visit_item_use(self, use_item);
        self.restore_test_context(was);
    }

    fn visit_item_extern_crate(&mut self, item: &'source syn::ItemExternCrate) {
        collect_std_extern_crate_alias(item, &mut self.std_aliases);
        syn::visit::visit_item_extern_crate(self, item);
    }
}

impl<'source> Visit<'source> for StdFsImportVisitor {
    fn visit_item_mod(&mut self, item_mod: &'source syn::ItemMod) {
        let was = self.save_and_apply_test_context(&item_mod.attrs);
        let std_aliases = self.std_aliases.clone();
        let fs_aliases = self.fs_aliases.clone();
        syn::visit::visit_item_mod(self, item_mod);
        self.std_aliases = std_aliases;
        self.fs_aliases = fs_aliases;
        self.restore_test_context(was);
    }

    fn visit_item_fn(&mut self, item_fn: &'source syn::ItemFn) {
        let was = self.save_and_apply_test_context(&item_fn.attrs);
        syn::visit::visit_item_fn(self, item_fn);
        self.restore_test_context(was);
    }

    fn visit_impl_item_fn(&mut self, item_fn: &'source syn::ImplItemFn) {
        let was = self.save_and_apply_test_context(&item_fn.attrs);
        syn::visit::visit_impl_item_fn(self, item_fn);
        self.restore_test_context(was);
    }

    fn visit_trait_item_fn(&mut self, item_fn: &'source syn::TraitItemFn) {
        let was = self.save_and_apply_test_context(&item_fn.attrs);
        syn::visit::visit_trait_item_fn(self, item_fn);
        self.restore_test_context(was);
    }

    fn visit_local(&mut self, local: &'source syn::Local) {
        let was = self.save_and_apply_test_context(&local.attrs);
        syn::visit::visit_local(self, local);
        self.restore_test_context(was);
    }

    fn visit_item_use(&mut self, use_item: &'source syn::ItemUse) {
        let was = self.save_and_apply_test_context(&use_item.attrs);
        collect_std_aliases(&use_item.tree, &mut self.std_aliases, &mut self.fs_aliases);
        if !self.in_test_context
            && use_tree_matches_std_fs(&use_item.tree, &self.std_aliases, &self.fs_aliases)
        {
            self.out.push(span_line(use_item.span()));
        }
        syn::visit::visit_item_use(self, use_item);
        self.restore_test_context(was);
    }

    fn visit_item_extern_crate(&mut self, item: &'source syn::ItemExternCrate) {
        collect_std_extern_crate_alias(item, &mut self.std_aliases);
        syn::visit::visit_item_extern_crate(self, item);
    }
}
