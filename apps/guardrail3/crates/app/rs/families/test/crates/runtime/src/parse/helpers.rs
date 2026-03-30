use std::collections::BTreeSet;

use syn::parse::{Parse, Parser};
use syn::visit::Visit;

use super::{TestBodyVisitor, UseBinding};

pub(super) fn is_test_attr(attr: &syn::Attribute) -> bool {
    let path = attr.path();
    path.is_ident("test")
        || path
            .segments
            .last()
            .is_some_and(|segment| segment.ident == "test")
}

pub(super) fn is_tokio_test_attr(attr: &syn::Attribute) -> bool {
    let path = attr.path();
    path.segments.len() == 2
        && path.segments[0].ident == "tokio"
        && path.segments[1].ident == "test"
}

pub(super) fn is_cfg_test_attr(attr: &syn::Attribute) -> bool {
    if !attr.path().is_ident("cfg") {
        return false;
    }
    match &attr.meta {
        syn::Meta::List(list) => list.tokens.to_string().replace(' ', "") == "test",
        _ => false,
    }
}

pub(super) fn is_assertion_macro_name(name: &str) -> bool {
    matches!(
        name,
        "assert"
            | "assert_eq"
            | "assert_ne"
            | "assert_matches"
            | "debug_assert"
            | "debug_assert_eq"
            | "debug_assert_ne"
    )
}

pub(super) fn call_path(expr: &syn::Expr) -> Option<Vec<String>> {
    match expr {
        syn::Expr::Path(path) => path
            .path
            .segments
            .iter()
            .map(|segment| segment.ident.to_string())
            .collect::<Vec<_>>()
            .pipe(Some),
        _ => None,
    }
}

pub(super) fn macro_has_literal_comparison(mac: &syn::Macro) -> bool {
    let parser = syn::punctuated::Punctuated::<syn::Expr, syn::Token![,]>::parse_terminated;
    let Ok(args) = parser.parse2(mac.tokens.clone()) else {
        return false;
    };
    if args.len() < 2 {
        return false;
    }
    args.iter().take(2).all(expr_is_literal_like)
}

pub(super) fn macro_has_weak_matches(mac: &syn::Macro) -> bool {
    let Ok(expr) = syn::parse2::<syn::Expr>(mac.tokens.clone()) else {
        return false;
    };
    let expr = peel_parens(expr);
    let syn::Expr::Macro(expr_macro) = expr else {
        return false;
    };
    let Some(name) = expr_macro
        .mac
        .path
        .segments
        .last()
        .map(|segment| segment.ident.to_string())
    else {
        return false;
    };
    if name != "matches" {
        return false;
    }
    let Ok(args) = syn::parse2::<MatchesArgs>(expr_macro.mac.tokens.clone()) else {
        return false;
    };
    pattern_contains_wild(&args.pattern)
}

pub(super) fn visit_macro_expr_args(visitor: &mut TestBodyVisitor, mac: &syn::Macro) {
    let parser = syn::punctuated::Punctuated::<syn::Expr, syn::Token![,]>::parse_terminated;
    let Ok(args) = parser.parse2(mac.tokens.clone()) else {
        return;
    };
    for expr in args {
        visitor.visit_expr(&expr);
    }
}

fn pattern_contains_wild(pattern: &syn::Pat) -> bool {
    match pattern {
        syn::Pat::Wild(_) => true,
        syn::Pat::Tuple(tuple) => tuple.elems.iter().any(pattern_contains_wild),
        syn::Pat::TupleStruct(tuple) => tuple.elems.iter().any(pattern_contains_wild),
        syn::Pat::Struct(strct) => strct
            .fields
            .iter()
            .any(|field| pattern_contains_wild(&field.pat)),
        syn::Pat::Slice(slice) => slice.elems.iter().any(pattern_contains_wild),
        syn::Pat::Reference(reference) => pattern_contains_wild(&reference.pat),
        syn::Pat::Or(or) => or.cases.iter().any(pattern_contains_wild),
        syn::Pat::Paren(paren) => pattern_contains_wild(&paren.pat),
        syn::Pat::Ident(_) | syn::Pat::Path(_) | syn::Pat::Lit(_) | syn::Pat::Range(_) => false,
        _ => false,
    }
}

pub(super) fn should_panic_has_expected(attr: &syn::Attribute) -> bool {
    let mut has_expected = false;
    let _ = attr.parse_nested_meta(|meta| {
        if meta.path.is_ident("expected") {
            let value: syn::LitStr = meta.value()?.parse()?;
            has_expected = !value.value().trim().is_empty();
        }
        Ok(())
    });
    has_expected
}

pub(super) fn span_line(span: proc_macro2::Span) -> usize {
    span.start().line
}

pub(super) fn path_attr_value(attr: &syn::Attribute) -> Option<String> {
    if !attr.path().is_ident("path") {
        return None;
    }
    match &attr.meta {
        syn::Meta::NameValue(name_value) => match &name_value.value {
            syn::Expr::Lit(expr_lit) => match &expr_lit.lit {
                syn::Lit::Str(value) => Some(value.value()),
                _ => None,
            },
            _ => None,
        },
        _ => None,
    }
}

pub(super) fn collect_use_bindings(
    tree: &syn::UseTree,
    prefix: &mut Vec<String>,
    line: usize,
    out: &mut Vec<UseBinding>,
) {
    match tree {
        syn::UseTree::Path(path) => {
            prefix.push(path.ident.to_string());
            collect_use_bindings(&path.tree, prefix, line, out);
            let _ = prefix.pop();
        }
        syn::UseTree::Name(name) => {
            let mut path_segments = prefix.clone();
            path_segments.push(name.ident.to_string());
            out.push(UseBinding {
                line,
                path_segments,
                local_name: Some(name.ident.to_string()),
            });
        }
        syn::UseTree::Rename(rename) => {
            let mut path_segments = prefix.clone();
            path_segments.push(rename.ident.to_string());
            out.push(UseBinding {
                line,
                path_segments,
                local_name: Some(rename.rename.to_string()),
            });
        }
        syn::UseTree::Glob(_) => {
            out.push(UseBinding {
                line,
                path_segments: prefix.clone(),
                local_name: None,
            });
        }
        syn::UseTree::Group(group) => {
            for item in &group.items {
                collect_use_bindings(item, prefix, line, out);
            }
        }
    }
}

fn expr_is_literal_like(expr: &syn::Expr) -> bool {
    match expr {
        syn::Expr::Lit(_) => true,
        syn::Expr::Paren(paren) => expr_is_literal_like(&paren.expr),
        syn::Expr::Group(group) => expr_is_literal_like(&group.expr),
        _ => false,
    }
}

fn peel_parens(expr: syn::Expr) -> syn::Expr {
    match expr {
        syn::Expr::Paren(paren) => peel_parens(*paren.expr),
        syn::Expr::Group(group) => peel_parens(*group.expr),
        other => other,
    }
}

pub(super) fn collect_pat_idents(pat: &syn::Pat, out: &mut BTreeSet<String>) {
    match pat {
        syn::Pat::Ident(ident) => {
            let _ = out.insert(ident.ident.to_string());
        }
        syn::Pat::Tuple(tuple) => {
            for element in &tuple.elems {
                collect_pat_idents(element, out);
            }
        }
        syn::Pat::TupleStruct(tuple) => {
            for element in &tuple.elems {
                collect_pat_idents(element, out);
            }
        }
        syn::Pat::Struct(strct) => {
            for field in &strct.fields {
                collect_pat_idents(&field.pat, out);
            }
        }
        syn::Pat::Slice(slice) => {
            for element in &slice.elems {
                collect_pat_idents(element, out);
            }
        }
        syn::Pat::Reference(reference) => collect_pat_idents(&reference.pat, out),
        syn::Pat::Type(typed) => collect_pat_idents(&typed.pat, out),
        syn::Pat::Or(or) => {
            for case in &or.cases {
                collect_pat_idents(case, out);
            }
        }
        syn::Pat::Paren(paren) => collect_pat_idents(&paren.pat, out),
        _ => {}
    }
}

trait Pipe: Sized {
    fn pipe<T>(self, f: impl FnOnce(Self) -> T) -> T {
        f(self)
    }
}

impl<T> Pipe for T {}

struct MatchesArgs {
    #[allow(dead_code)] // reason: only the pattern matters for this rule
    expr: syn::Expr,
    #[allow(dead_code)] // reason: optional guard is accepted but not inspected
    guard: Option<syn::Expr>,
    pattern: syn::Pat,
}

impl Parse for MatchesArgs {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        let expr = input.parse::<syn::Expr>()?;
        let _ = input.parse::<syn::Token![,]>()?;
        let pattern = syn::Pat::parse_multi_with_leading_vert(input)?;
        let guard = if input.peek(syn::Token![if]) {
            let _ = input.parse::<syn::Token![if]>()?;
            Some(input.parse::<syn::Expr>()?)
        } else {
            None
        };
        Ok(Self {
            expr,
            guard,
            pattern,
        })
    }
}
