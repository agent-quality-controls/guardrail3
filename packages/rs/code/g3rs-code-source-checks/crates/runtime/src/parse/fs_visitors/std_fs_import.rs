use std::collections::BTreeSet;

use syn::spanned::Spanned;
use syn::visit::Visit;

use super::super::helpers::span_line;
use super::support::{
    TestContextAware, collect_std_aliases, collect_std_extern_crate_alias, use_tree_matches_std_fs,
};

pub(super) struct StdFsImportVisitor {
    pub(super) out: Vec<usize>,
    in_test_context: bool,
    std_aliases: BTreeSet<String>,
    fs_aliases: BTreeSet<String>,
}

impl TestContextAware for StdFsImportVisitor {
    fn in_test_context_mut(&mut self) -> &mut bool {
        &mut self.in_test_context
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

impl<'source> Visit<'source> for StdFsImportVisitor {
    fn visit_block(&mut self, block: &'source syn::Block) {
        let std_aliases = self.std_aliases.clone();
        let fs_aliases = self.fs_aliases.clone();
        syn::visit::visit_block(self, block);
        self.std_aliases = std_aliases;
        self.fs_aliases = fs_aliases;
    }

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
        let std_aliases = self.std_aliases.clone();
        let fs_aliases = self.fs_aliases.clone();
        syn::visit::visit_item_fn(self, item_fn);
        self.std_aliases = std_aliases;
        self.fs_aliases = fs_aliases;
        self.restore_test_context(was);
    }

    fn visit_impl_item_fn(&mut self, item_fn: &'source syn::ImplItemFn) {
        let was = self.save_and_apply_test_context(&item_fn.attrs);
        let std_aliases = self.std_aliases.clone();
        let fs_aliases = self.fs_aliases.clone();
        syn::visit::visit_impl_item_fn(self, item_fn);
        self.std_aliases = std_aliases;
        self.fs_aliases = fs_aliases;
        self.restore_test_context(was);
    }

    fn visit_trait_item_fn(&mut self, item_fn: &'source syn::TraitItemFn) {
        let was = self.save_and_apply_test_context(&item_fn.attrs);
        let std_aliases = self.std_aliases.clone();
        let fs_aliases = self.fs_aliases.clone();
        syn::visit::visit_trait_item_fn(self, item_fn);
        self.std_aliases = std_aliases;
        self.fs_aliases = fs_aliases;
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
