use quote::ToTokens;
use syn::spanned::Spanned;
use syn::visit::Visit;

use super::super::helpers::span_line;
use super::super::types::StringDispatchInfo;
use super::core::TestContextAware;

pub(super) fn find_string_dispatch_sites(
    source: &syn::File,
    file_is_test_root: bool,
) -> Vec<StringDispatchInfo> {
    let mut visitor = StringDispatchVisitor {
        out: Vec::new(),
        in_test_context: file_is_test_root,
    };
    visitor.visit_file(source);
    visitor.out
}

struct StringDispatchVisitor {
    out: Vec<StringDispatchInfo>,
    in_test_context: bool,
}

impl TestContextAware for StringDispatchVisitor {
    fn in_test_context_mut(&mut self) -> &mut bool {
        &mut self.in_test_context
    }
}

impl<'source> Visit<'source> for StringDispatchVisitor {
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

    fn visit_item_mod(&mut self, item_mod: &'source syn::ItemMod) {
        let was = self.save_and_apply_test_context(&item_mod.attrs);
        syn::visit::visit_item_mod(self, item_mod);
        self.restore_test_context(was);
    }

    fn visit_local(&mut self, local: &'source syn::Local) {
        let was = self.save_and_apply_test_context(&local.attrs);
        syn::visit::visit_local(self, local);
        self.restore_test_context(was);
    }

    fn visit_expr_match(&mut self, expr_match: &'source syn::ExprMatch) {
        if !self.in_test_context {
            let string_literal_branch_count = expr_match
                .arms
                .iter()
                .filter(|arm| arm.guard.is_none())
                .map(|arm| string_literal_patterns(&arm.pat))
                .sum::<usize>();
            if string_literal_branch_count > 10 {
                self.out.push(StringDispatchInfo {
                    line: span_line(expr_match.match_token.span()),
                    site_kind: "match",
                    string_literal_branch_count,
                });
            }
        }
        syn::visit::visit_expr_match(self, expr_match);
    }

    fn visit_expr_if(&mut self, expr_if: &'source syn::ExprIf) {
        if !self.in_test_context {
            if let Some(string_literal_branch_count) =
                if_else_chain_string_dispatch_count(expr_if).filter(|count| *count > 10)
            {
                self.out.push(StringDispatchInfo {
                    line: span_line(expr_if.if_token.span()),
                    site_kind: "if/else if chain",
                    string_literal_branch_count,
                });
            }
        }
        syn::visit::visit_expr_if(self, expr_if);
    }
}

fn string_literal_patterns(pat: &syn::Pat) -> usize {
    match pat {
        syn::Pat::Lit(expr_lit) => usize::from(matches!(expr_lit.lit, syn::Lit::Str(_))),
        syn::Pat::Or(pat_or) => pat_or.cases.iter().map(string_literal_patterns).sum(),
        syn::Pat::Reference(reference) => string_literal_patterns(&reference.pat),
        syn::Pat::Paren(paren) => string_literal_patterns(&paren.pat),
        _ => 0,
    }
}

fn if_else_chain_string_dispatch_count(expr_if: &syn::ExprIf) -> Option<usize> {
    let (base_expr, first_matches) = string_equality_branch(&expr_if.cond)?;
    let mut count = first_matches;
    let mut current_else = expr_if.else_branch.as_ref().map(|(_, expr)| expr.as_ref());
    while let Some(expr) = current_else {
        match expr {
            syn::Expr::If(next_if) => {
                let (next_expr, matches) = string_equality_branch(&next_if.cond)?;
                if next_expr != base_expr {
                    return None;
                }
                count += matches;
                current_else = next_if
                    .else_branch
                    .as_ref()
                    .map(|(_, nested)| nested.as_ref());
            }
            _ => break,
        }
    }
    Some(count)
}

fn string_equality_branch(expr: &syn::Expr) -> Option<(String, usize)> {
    let syn::Expr::Binary(binary) = expr else {
        return None;
    };
    if !matches!(binary.op, syn::BinOp::Eq(_)) {
        return None;
    }
    if let Some(value_expr) = normalize_expr_without_strings(&binary.left) {
        if is_string_literal_expr(&binary.right) {
            return Some((value_expr, 1));
        }
    }
    if let Some(value_expr) = normalize_expr_without_strings(&binary.right) {
        if is_string_literal_expr(&binary.left) {
            return Some((value_expr, 1));
        }
    }
    None
}

fn is_string_literal_expr(expr: &syn::Expr) -> bool {
    matches!(
        expr,
        syn::Expr::Lit(syn::ExprLit {
            lit: syn::Lit::Str(_),
            ..
        })
    )
}

fn normalize_expr_without_strings(expr: &syn::Expr) -> Option<String> {
    if is_string_literal_expr(expr) {
        return None;
    }
    Some(expr.to_token_stream().to_string())
}
