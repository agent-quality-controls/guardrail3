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
    fn path_is_std_fs_call(path: &syn::Path) -> bool {
        let mut segs = path
            .segments
            .iter()
            .map(|segment| segment.ident.to_string());
        matches!(
            (segs.next().as_deref(), segs.next().as_deref(), segs.next()),
            (Some("std"), Some("fs"), Some(_))
        )
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
    pub findings: Vec<IgnoreReasonInfo>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IgnoreReasonInfo {
    pub line: usize,
    pub reason: Option<String>,
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
            if attr.path().is_ident("ignore") {
                self.record_ignore_meta(&attr.meta, span_line(attr.span()));
                continue;
            }
            if !attr.path().is_ident("cfg_attr") {
                continue;
            }
            let syn::Meta::List(list) = &attr.meta else {
                continue;
            };
            let Ok(args) = list.parse_args_with(
                syn::punctuated::Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated,
            ) else {
                continue;
            };
            let mut iter = args.into_iter();
            let Some(condition) = iter.next() else {
                continue;
            };
            if !cfg_meta_contains_positive_test(&condition) {
                continue;
            }
            let line = span_line(attr.span());
            for meta in iter {
                if meta.path().is_ident("ignore") {
                    self.record_ignore_meta(&meta, line);
                }
            }
        }
    }

    fn record_ignore_meta(&mut self, meta: &syn::Meta, line: usize) {
        if let Some(reason) = reason_from_ignore_meta(meta) {
            self.findings.push(IgnoreReasonInfo {
                line,
                reason: Some(reason),
            });
            return;
        }

        if !matches!(meta, syn::Meta::Path(_)) {
            return;
        }

        let idx = line.saturating_sub(1);
        if let Some(same_line) = self.lines.get(idx) {
            if let Some(reason) = extract_comment_reason(same_line) {
                self.findings.push(IgnoreReasonInfo {
                    line,
                    reason: Some(reason),
                });
                return;
            }
        }
        if idx > 0 {
            if let Some(prev_line) = self.lines.get(idx.saturating_sub(1)) {
                if let Some(reason) = extract_comment_reason(prev_line) {
                    self.findings.push(IgnoreReasonInfo {
                        line,
                        reason: Some(reason),
                    });
                    return;
                }
            }
        }

        self.findings.push(IgnoreReasonInfo { line, reason: None });
    }
}

fn reason_from_ignore_meta(meta: &syn::Meta) -> Option<String> {
    match meta {
        syn::Meta::NameValue(name_value) => match &name_value.value {
            syn::Expr::Lit(expr_lit) => match &expr_lit.lit {
                syn::Lit::Str(value) => Some(value.value()),
                _ => None,
            },
            _ => None,
        },
        syn::Meta::List(_) | syn::Meta::Path(_) => None,
    }
}

fn extract_comment_reason(line: &str) -> Option<String> {
    const TOKENS: [&str; 2] = ["// reason:", "//reason:"];

    TOKENS.iter().find_map(|token| {
        line.find(token)
            .map(|index| line[index + token.len()..].trim().to_owned())
    })
}

fn cfg_meta_contains_positive_test(meta: &syn::Meta) -> bool {
    match meta {
        syn::Meta::Path(path) => path.is_ident("test"),
        syn::Meta::NameValue(_) => false,
        syn::Meta::List(list) if list.path.is_ident("not") => false,
        syn::Meta::List(list) => list
            .parse_args_with(
                syn::punctuated::Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated,
            )
            .map(|items| items.iter().any(cfg_meta_contains_positive_test))
            .unwrap_or(false),
    }
}

#[cfg(test)]

mod extra_visitors_tests;
