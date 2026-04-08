use syn::spanned::Spanned;
use syn::visit::Visit;

use super::helpers::{attrs_enter_test_context, path_to_string, span_line};
use super::types::ForbiddenMacroInfo;

pub(crate) fn find_forbidden_macros(
    ast: &syn::File,
    file_is_test_root: bool,
) -> Vec<ForbiddenMacroInfo> {
    let mut visitor = ForbiddenMacroVisitor {
        out: Vec::new(),
        in_test_context: file_is_test_root,
    };
    visitor.visit_file(ast);
    visitor.out
}

struct ForbiddenMacroVisitor {
    out: Vec<ForbiddenMacroInfo>,
    in_test_context: bool,
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

impl TestContextAware for ForbiddenMacroVisitor {
    fn in_test_context_mut(&mut self) -> &mut bool {
        &mut self.in_test_context
    }
}

impl<'ast> Visit<'ast> for ForbiddenMacroVisitor {
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

    fn visit_item_mod(&mut self, item_mod: &'ast syn::ItemMod) {
        let was = self.save_and_apply_test_context(&item_mod.attrs);
        syn::visit::visit_item_mod(self, item_mod);
        self.restore_test_context(was);
    }

    fn visit_local(&mut self, local: &'ast syn::Local) {
        let was = self.save_and_apply_test_context(&local.attrs);
        syn::visit::visit_local(self, local);
        self.restore_test_context(was);
    }

    fn visit_macro(&mut self, macro_call: &'ast syn::Macro) {
        let name = path_to_string(&macro_call.path);
        let base = name.rsplit("::").next().unwrap_or(&name);
        if matches!(base, "todo" | "unimplemented" | "unreachable" | "panic") {
            self.out.push(ForbiddenMacroInfo {
                line: span_line(macro_call.path.span()),
                macro_name: name,
                in_test_context: self.in_test_context,
            });
        }
        syn::visit::visit_macro(self, macro_call);
    }
}
