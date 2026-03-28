//! AST visitors extracted from `ast_visitors.rs` to keep files under 500 lines.
//!
//! Contains [`InlineStdFsVisitor`] (R58 direct `std::fs` detection) and
//! [`IgnoreVisitor`] (`#[ignore]` without reason detection).

use syn::spanned::Spanned;
use syn::visit::Visit;

use super::ast_helpers::{attrs_have_allow_lint, is_cfg_test_attr, span_line};

#[derive(Debug)]
pub struct InlineStdFsVisitor {
    pub out: Vec<usize>,
    pub in_cfg_test: bool,
    pub in_allowed_scope: bool,
}

impl InlineStdFsVisitor {
    /// Check if a path is a direct `std::fs` usage like `std::fs::read_to_string`
    /// or `std::fs::File::open`. Accepts ALL paths with 3+ segments starting with
    /// `std::fs`. Type-path concerns are moot because `visit_expr_path` and
    /// `visit_expr_call` only fire on expression paths, not type paths.
    #[allow(clippy::indexing_slicing)] // reason: length checked >= 3 before indexing [0], [1]
    fn path_is_std_fs_call(path: &syn::Path) -> bool {
        let segs: Vec<_> = path.segments.iter().map(|s| s.ident.to_string()).collect();
        segs.len() >= 3 && segs[0] == "std" && segs[1] == "fs"
    }
}

impl<'ast> Visit<'ast> for InlineStdFsVisitor {
    fn visit_item_mod(&mut self, n: &'ast syn::ItemMod) {
        let (was_test, was_allow) = (self.in_cfg_test, self.in_allowed_scope);
        self.in_cfg_test |= n.attrs.iter().any(is_cfg_test_attr);
        self.in_allowed_scope |= attrs_have_allow_lint(&n.attrs, "disallowed_methods");
        syn::visit::visit_item_mod(self, n);
        (self.in_cfg_test, self.in_allowed_scope) = (was_test, was_allow);
    }
    fn visit_item_fn(&mut self, n: &'ast syn::ItemFn) {
        let (was_test, was_allow) = (self.in_cfg_test, self.in_allowed_scope);
        self.in_cfg_test |= n.attrs.iter().any(is_cfg_test_attr);
        self.in_allowed_scope |= attrs_have_allow_lint(&n.attrs, "disallowed_methods");
        syn::visit::visit_item_fn(self, n);
        (self.in_cfg_test, self.in_allowed_scope) = (was_test, was_allow);
    }
    fn visit_impl_item_fn(&mut self, n: &'ast syn::ImplItemFn) {
        let was = self.in_allowed_scope;
        self.in_allowed_scope |= attrs_have_allow_lint(&n.attrs, "disallowed_methods");
        syn::visit::visit_impl_item_fn(self, n);
        self.in_allowed_scope = was;
    }
    fn visit_expr_call(&mut self, n: &'ast syn::ExprCall) {
        if !self.in_cfg_test && !self.in_allowed_scope {
            if let syn::Expr::Path(ep) = &*n.func {
                if Self::path_is_std_fs_call(&ep.path) {
                    self.out.push(span_line(ep.path.span()));
                }
            }
        }
        syn::visit::visit_expr_call(self, n);
    }

    fn visit_expr_path(&mut self, n: &'ast syn::ExprPath) {
        // Catch function pointers: `let f = std::fs::read_to_string;`
        if !self.in_cfg_test && !self.in_allowed_scope && Self::path_is_std_fs_call(&n.path) {
            let line = span_line(n.path.span());
            if !self.out.contains(&line) {
                self.out.push(line);
            }
        }
        syn::visit::visit_expr_path(self, n);
    }
}

// ---------------------------------------------------------------------------
// IgnoreVisitor — detects `#[ignore]` without reason comment
// ---------------------------------------------------------------------------

#[derive(Debug)]
pub struct IgnoreVisitor<'s> {
    pub lines: Vec<&'s str>,
    pub violations: Vec<usize>,
}
impl<'ast> Visit<'ast> for IgnoreVisitor<'_> {
    fn visit_item_fn(&mut self, n: &'ast syn::ItemFn) {
        self.check_ignore_attrs(&n.attrs);
        syn::visit::visit_item_fn(self, n);
    }
    fn visit_impl_item_fn(&mut self, n: &'ast syn::ImplItemFn) {
        self.check_ignore_attrs(&n.attrs);
        syn::visit::visit_impl_item_fn(self, n);
    }
}
impl IgnoreVisitor<'_> {
    fn check_ignore_attrs(&mut self, attrs: &[syn::Attribute]) {
        for attr in attrs {
            if !attr.path().is_ident("ignore") {
                continue;
            }
            // #[ignore = "reason"] (NameValue) and #[ignore(...)] (List) provide
            // the reason inline — only bare #[ignore] (Path) needs a comment.
            if !matches!(attr.meta, syn::Meta::Path(_)) {
                continue;
            }
            let line = span_line(attr.span());
            // 1-based to 0-based index
            let idx = line.saturating_sub(1);
            // Check same line for reason comment
            if let Some(same_line) = self.lines.get(idx) {
                if same_line.contains("// reason:") || same_line.contains("//reason:") {
                    continue;
                }
            }
            // Check previous line for reason comment
            if idx > 0 {
                if let Some(prev_line) = self.lines.get(idx.saturating_sub(1)) {
                    if prev_line.contains("// reason:") || prev_line.contains("//reason:") {
                        continue;
                    }
                }
            }
            self.violations.push(line);
        }
    }
}

#[cfg(test)]
#[path = "extra_visitors_tests.rs"]
mod tests;
