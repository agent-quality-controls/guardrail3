use std::collections::BTreeSet;

use syn::spanned::Spanned;
use syn::visit::Visit;

use super::super::helpers::span_line;
use super::support::{TestContextAware, collect_std_aliases, collect_std_extern_crate_alias};

pub(super) struct InlineStdFsVisitor {
    pub(super) out: Vec<usize>,
    in_test_context: bool,
    std_aliases: BTreeSet<String>,
    fs_aliases: BTreeSet<String>,
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

impl<'source> Visit<'source> for InlineStdFsVisitor {
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
                if Self::path_is_std_fs_call(&expr_path.path, &self.std_aliases, &self.fs_aliases)
                {
                    self.out.push(span_line(expr_path.path.span()));
                }
            }
        }
        syn::visit::visit_expr_call(self, expr_call);
    }
}
