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

use syn::spanned::Spanned;
use syn::visit::Visit;

use super::super::helpers::span_line;
use super::support::{
    TestContextAware, extend_scope_aliases_from_block, extend_scope_aliases_from_file,
    extend_scope_aliases_from_items, use_tree_is_std_fs_glob,
};

pub(super) struct StdFsGlobImportVisitor {
    pub(super) out: Vec<usize>,
    /// Field `in_test_context`.
    in_test_context: bool,
    /// Field `std_aliases`.
    std_aliases: BTreeSet<String>,
    /// Field `fs_aliases`.
    fs_aliases: BTreeSet<String>,
}

impl TestContextAware for StdFsGlobImportVisitor {
    fn in_test_context_mut(&mut self) -> &mut bool {
        &mut self.in_test_context
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

impl<'source> Visit<'source> for StdFsGlobImportVisitor {
    fn visit_file(&mut self, file: &'source syn::File) {
        let std_aliases = self.std_aliases.clone();
        let fs_aliases = self.fs_aliases.clone();
        extend_scope_aliases_from_file(
            file,
            self.in_test_context,
            &mut self.std_aliases,
            &mut self.fs_aliases,
        );
        syn::visit::visit_file(self, file);
        self.std_aliases = std_aliases;
        self.fs_aliases = fs_aliases;
    }

    fn visit_block(&mut self, block: &'source syn::Block) {
        let std_aliases = self.std_aliases.clone();
        let fs_aliases = self.fs_aliases.clone();
        extend_scope_aliases_from_block(
            block,
            self.in_test_context,
            &mut self.std_aliases,
            &mut self.fs_aliases,
        );
        syn::visit::visit_block(self, block);
        self.std_aliases = std_aliases;
        self.fs_aliases = fs_aliases;
    }

    fn visit_item_mod(&mut self, item_mod: &'source syn::ItemMod) {
        let was = self.save_and_apply_test_context(&item_mod.attrs);
        let std_aliases = self.std_aliases.clone();
        let fs_aliases = self.fs_aliases.clone();
        if let Some((_, items)) = &item_mod.content {
            extend_scope_aliases_from_items(
                items,
                self.in_test_context,
                &mut self.std_aliases,
                &mut self.fs_aliases,
            );
        }
        syn::visit::visit_item_mod(self, item_mod);
        self.std_aliases = std_aliases;
        self.fs_aliases = fs_aliases;
        self.restore_test_context(was);
    }

    fn visit_item_fn(&mut self, item_fn: &'source syn::ItemFn) {
        let was = self.save_and_apply_test_context(&item_fn.attrs);
        let std_aliases = self.std_aliases.clone();
        let fs_aliases = self.fs_aliases.clone();
        extend_scope_aliases_from_block(
            &item_fn.block,
            self.in_test_context,
            &mut self.std_aliases,
            &mut self.fs_aliases,
        );
        syn::visit::visit_item_fn(self, item_fn);
        self.std_aliases = std_aliases;
        self.fs_aliases = fs_aliases;
        self.restore_test_context(was);
    }

    fn visit_impl_item_fn(&mut self, item_fn: &'source syn::ImplItemFn) {
        let was = self.save_and_apply_test_context(&item_fn.attrs);
        let std_aliases = self.std_aliases.clone();
        let fs_aliases = self.fs_aliases.clone();
        extend_scope_aliases_from_block(
            &item_fn.block,
            self.in_test_context,
            &mut self.std_aliases,
            &mut self.fs_aliases,
        );
        syn::visit::visit_impl_item_fn(self, item_fn);
        self.std_aliases = std_aliases;
        self.fs_aliases = fs_aliases;
        self.restore_test_context(was);
    }

    fn visit_trait_item_fn(&mut self, item_fn: &'source syn::TraitItemFn) {
        let was = self.save_and_apply_test_context(&item_fn.attrs);
        let std_aliases = self.std_aliases.clone();
        let fs_aliases = self.fs_aliases.clone();
        if let Some(block) = &item_fn.default {
            extend_scope_aliases_from_block(
                block,
                self.in_test_context,
                &mut self.std_aliases,
                &mut self.fs_aliases,
            );
        }
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
        if !self.in_test_context
            && use_tree_is_std_fs_glob(&use_item.tree, &self.std_aliases, &self.fs_aliases)
        {
            self.out.push(span_line(use_item.tree.span()));
        }
        syn::visit::visit_item_use(self, use_item);
        self.restore_test_context(was);
    }

    fn visit_item_extern_crate(&mut self, item: &'source syn::ItemExternCrate) {
        let was = self.save_and_apply_test_context(&item.attrs);
        syn::visit::visit_item_extern_crate(self, item);
        self.restore_test_context(was);
    }
}
