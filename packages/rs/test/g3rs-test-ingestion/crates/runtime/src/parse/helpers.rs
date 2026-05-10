#![expect(
    clippy::indexing_slicing,
    reason = "structural code pattern (parser/assertion helper) where lint conflicts with module architecture"
)]
#![expect(
    clippy::match_same_arms,
    reason = "structural code pattern (parser/assertion helper) where lint conflicts with module architecture"
)]
#![expect(
    clippy::shadow_unrelated,
    reason = "structural code pattern (parser/assertion helper) where lint conflicts with module architecture"
)]
#![expect(
    clippy::wildcard_enum_match_arm,
    reason = "structural code pattern (parser/assertion helper) where lint conflicts with module architecture"
)]
use std::collections::BTreeSet;

use syn::parse::{Parse, Parser};
use syn::punctuated::Punctuated;
use syn::visit::Visit;

use super::body::TestBodyVisitor;
use g3rs_test_types::ast::UseBinding;

/// `is_test_attr` function.
pub(super) fn is_test_attr(attr: &syn::Attribute) -> bool {
    let predicate = cfg_predicate(attr);
    path_is_test_attr(attr.path())
        || cfg_attr_nested_metas(attr)
            .into_iter()
            .flatten()
            .any(|meta| {
                predicate
                    .as_ref()
                    .is_some_and(cfg_meta_contains_positive_test)
                    && meta_path_is_test(&meta)
            })
}

/// `is_tokio_test_attr` function.
pub(super) fn is_tokio_test_attr(attr: &syn::Attribute) -> bool {
    let predicate = cfg_predicate(attr);
    path_is_tokio_test_attr(attr.path())
        || cfg_attr_nested_metas(attr)
            .into_iter()
            .flatten()
            .any(|meta| {
                predicate
                    .as_ref()
                    .is_some_and(cfg_meta_contains_positive_test)
                    && meta_path_is_tokio_test(&meta)
            })
}

/// `is_cfg_test_attr` function.
pub(super) fn is_cfg_test_attr(attr: &syn::Attribute) -> bool {
    if !attr.path().is_ident("cfg") {
        return false;
    }
    cfg_predicate(attr).is_some_and(|meta| cfg_meta_requires_test(&meta))
}

/// `is_should_panic_attr` function.
pub(super) fn is_should_panic_attr(attr: &syn::Attribute) -> bool {
    let predicate = cfg_predicate(attr);
    attr.path().is_ident("should_panic")
        || cfg_attr_nested_metas(attr)
            .into_iter()
            .flatten()
            .any(|meta| {
                predicate
                    .as_ref()
                    .is_some_and(cfg_meta_contains_positive_test)
                    && meta.path().is_ident("should_panic")
            })
}

/// `is_assertion_macro_name` function.
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

/// `call_path` function.
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

/// `macro_has_literal_comparison` function.
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

/// `macro_has_weak_matches` function.
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
    let Ok(args) = syn::parse2::<MatchesArgs>(expr_macro.mac.tokens) else {
        return false;
    };
    pattern_contains_wild(&args.pattern)
}

/// `macro_has_weak_assert_matches` function.
pub(super) fn macro_has_weak_assert_matches(mac: &syn::Macro) -> bool {
    let Ok(args) = syn::parse2::<MatchesArgs>(mac.tokens.clone()) else {
        return false;
    };
    pattern_contains_wild(&args.pattern)
}

/// `visit_macro_expr_args` function.
pub(super) fn visit_macro_expr_args(visitor: &mut TestBodyVisitor, mac: &syn::Macro) {
    let parser = syn::punctuated::Punctuated::<syn::Expr, syn::Token![,]>::parse_terminated;
    let Ok(args) = parser.parse2(mac.tokens.clone()) else {
        return;
    };
    for expr in args {
        visitor.visit_expr(&expr);
    }
}

/// `pattern_contains_wild` function.
fn pattern_contains_wild(pattern: &syn::Pat) -> bool {
    match pattern {
        syn::Pat::Wild(_) => true,
        syn::Pat::Rest(_) => true,
        syn::Pat::Tuple(tuple) => tuple.elems.iter().any(pattern_contains_wild),
        syn::Pat::TupleStruct(tuple) => tuple.elems.iter().any(pattern_contains_wild),
        syn::Pat::Struct(strct) => {
            strct
                .fields
                .iter()
                .any(|field| pattern_contains_wild(&field.pat))
                || strct.rest.is_some()
        }
        syn::Pat::Slice(slice) => slice.elems.iter().any(pattern_contains_wild),
        syn::Pat::Reference(reference) => pattern_contains_wild(&reference.pat),
        syn::Pat::Or(or) => or.cases.iter().any(pattern_contains_wild),
        syn::Pat::Paren(paren) => pattern_contains_wild(&paren.pat),
        syn::Pat::Ident(_) | syn::Pat::Path(_) | syn::Pat::Lit(_) | syn::Pat::Range(_) => false,
        _ => false,
    }
}

/// `should_panic_has_expected` function.
pub(super) fn should_panic_has_expected(attr: &syn::Attribute) -> bool {
    if attr.path().is_ident("should_panic") {
        return meta_has_should_panic_expected(&attr.meta);
    }
    let predicate = cfg_predicate(attr);
    cfg_attr_nested_metas(attr)
        .into_iter()
        .flatten()
        .filter(|meta| meta.path().is_ident("should_panic"))
        .any(|meta| {
            predicate
                .as_ref()
                .is_some_and(cfg_meta_contains_positive_test)
                && meta_has_should_panic_expected(&meta)
        })
}

/// `meta_has_should_panic_expected` function.
fn meta_has_should_panic_expected(meta: &syn::Meta) -> bool {
    let mut has_expected = false;
    let _ = parse_meta_nested(meta, |meta| {
        if meta.path.is_ident("expected") {
            let value: syn::LitStr = meta.value()?.parse()?;
            has_expected = !value.value().trim().is_empty();
        }
        Ok(())
    });
    has_expected
}

/// `span_line` function.
pub(super) fn span_line(span: proc_macro2::Span) -> usize {
    span.start().line
}

/// `path_attr_value` function.
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

/// `collect_use_bindings` function.
pub(super) fn collect_use_bindings(
    tree: &syn::UseTree,
    prefix: &mut Vec<String>,
    line: usize,
    is_public: bool,
    out: &mut Vec<UseBinding>,
) {
    match tree {
        syn::UseTree::Path(path) => {
            prefix.push(path.ident.to_string());
            collect_use_bindings(&path.tree, prefix, line, is_public, out);
            let _ = prefix.pop();
        }
        syn::UseTree::Name(name) => {
            let mut path_segments = prefix.clone();
            path_segments.push(name.ident.to_string());
            out.push(UseBinding {
                line,
                path_segments,
                local_name: Some(name.ident.to_string()),
                is_public,
            });
        }
        syn::UseTree::Rename(rename) => {
            let mut path_segments = prefix.clone();
            path_segments.push(rename.ident.to_string());
            out.push(UseBinding {
                line,
                path_segments,
                local_name: Some(rename.rename.to_string()),
                is_public,
            });
        }
        syn::UseTree::Glob(_) => {
            out.push(UseBinding {
                line,
                path_segments: prefix.clone(),
                local_name: None,
                is_public,
            });
        }
        syn::UseTree::Group(group) => {
            for item in &group.items {
                collect_use_bindings(item, prefix, line, is_public, out);
            }
        }
    }
}

/// `expr_is_literal_like` function.
fn expr_is_literal_like(expr: &syn::Expr) -> bool {
    match expr {
        syn::Expr::Lit(_) => true,
        syn::Expr::Paren(paren) => expr_is_literal_like(&paren.expr),
        syn::Expr::Group(group) => expr_is_literal_like(&group.expr),
        _ => false,
    }
}

/// `path_is_test_attr` function.
fn path_is_test_attr(path: &syn::Path) -> bool {
    path.is_ident("test")
        || path
            .segments
            .last()
            .is_some_and(|segment| segment.ident == "test")
}

/// `meta_path_is_test` function.
fn meta_path_is_test(meta: &syn::Meta) -> bool {
    path_is_test_attr(meta.path())
}

/// `path_is_tokio_test_attr` function.
fn path_is_tokio_test_attr(path: &syn::Path) -> bool {
    path.segments.len() == 2
        && path.segments[0].ident == "tokio"
        && path.segments[1].ident == "test"
}

/// `meta_path_is_tokio_test` function.
fn meta_path_is_tokio_test(meta: &syn::Meta) -> bool {
    path_is_tokio_test_attr(meta.path())
}

/// `cfg_attr_nested_metas` function.
fn cfg_attr_nested_metas(attr: &syn::Attribute) -> Option<Vec<syn::Meta>> {
    if !attr.path().is_ident("cfg_attr") {
        return None;
    }
    let syn::Meta::List(list) = &attr.meta else {
        return None;
    };
    let Ok(args) = list.parse_args_with(Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated)
    else {
        return None;
    };
    let mut iter = args.into_iter();
    let _ = iter.next()?;
    Some(iter.collect())
}

/// `cfg_predicate` function.
fn cfg_predicate(attr: &syn::Attribute) -> Option<syn::Meta> {
    let syn::Meta::List(list) = &attr.meta else {
        return None;
    };
    if attr.path().is_ident("cfg") {
        return syn::parse2::<syn::Meta>(list.tokens.clone()).ok();
    }
    if !attr.path().is_ident("cfg_attr") {
        return None;
    }
    list.parse_args_with(Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated)
        .ok()?
        .into_iter()
        .next()
}

/// `cfg_meta_requires_test` function.
fn cfg_meta_requires_test(meta: &syn::Meta) -> bool {
    cfg_meta_can_be_true(meta, true) && !cfg_meta_can_be_true(meta, false)
}

/// `cfg_meta_contains_positive_test` function.
fn cfg_meta_contains_positive_test(meta: &syn::Meta) -> bool {
    match meta {
        syn::Meta::Path(path) => path.is_ident("test"),
        syn::Meta::NameValue(_) => false,
        syn::Meta::List(list) if list.path.is_ident("not") => false,
        syn::Meta::List(list) => nested_cfg_meta_items(list)
            .iter()
            .any(cfg_meta_contains_positive_test),
    }
}

/// `cfg_meta_can_be_true` function.
fn cfg_meta_can_be_true(meta: &syn::Meta, test_enabled: bool) -> bool {
    match meta {
        syn::Meta::Path(path) => !path.is_ident("test") || test_enabled,
        syn::Meta::NameValue(_) => true,
        syn::Meta::List(list) if list.path.is_ident("all") => nested_cfg_meta_items(list)
            .iter()
            .all(|meta| cfg_meta_can_be_true(meta, test_enabled)),
        syn::Meta::List(list) if list.path.is_ident("any") => nested_cfg_meta_items(list)
            .iter()
            .any(|meta| cfg_meta_can_be_true(meta, test_enabled)),
        syn::Meta::List(list) if list.path.is_ident("not") => nested_cfg_meta_items(list)
            .first()
            .is_some_and(|meta| cfg_meta_can_be_false(meta, test_enabled)),
        syn::Meta::List(_) => true,
    }
}

/// `cfg_meta_can_be_false` function.
fn cfg_meta_can_be_false(meta: &syn::Meta, test_enabled: bool) -> bool {
    match meta {
        syn::Meta::Path(path) => !path.is_ident("test") || !test_enabled,
        syn::Meta::NameValue(_) => true,
        syn::Meta::List(list) if list.path.is_ident("all") => nested_cfg_meta_items(list)
            .iter()
            .any(|meta| cfg_meta_can_be_false(meta, test_enabled)),
        syn::Meta::List(list) if list.path.is_ident("any") => nested_cfg_meta_items(list)
            .iter()
            .all(|meta| cfg_meta_can_be_false(meta, test_enabled)),
        syn::Meta::List(list) if list.path.is_ident("not") => nested_cfg_meta_items(list)
            .first()
            .is_some_and(|meta| cfg_meta_can_be_true(meta, test_enabled)),
        syn::Meta::List(_) => true,
    }
}

/// `nested_cfg_meta_items` function.
fn nested_cfg_meta_items(list: &syn::MetaList) -> Vec<syn::Meta> {
    list.parse_args_with(Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated)
        .map(|items| items.into_iter().collect())
        .unwrap_or_default()
}

/// `parse_meta_nested` function.
fn parse_meta_nested(
    meta: &syn::Meta,
    logic: impl FnMut(syn::meta::ParseNestedMeta<'_>) -> syn::Result<()>,
) -> syn::Result<()> {
    match meta {
        syn::Meta::List(list) => list.parse_nested_meta(logic),
        _ => Ok(()),
    }
}

/// `peel_parens` function.
fn peel_parens(expr: syn::Expr) -> syn::Expr {
    match expr {
        syn::Expr::Paren(paren) => peel_parens(*paren.expr),
        syn::Expr::Group(group) => peel_parens(*group.expr),
        other => other,
    }
}

/// `collect_pat_idents` function.
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

/// `single_pat_ident` function.
pub(super) fn single_pat_ident(pat: &syn::Pat) -> Option<String> {
    match pat {
        syn::Pat::Ident(ident) => Some(ident.ident.to_string()),
        syn::Pat::Type(typed) => single_pat_ident(&typed.pat),
        syn::Pat::Reference(reference) => single_pat_ident(&reference.pat),
        syn::Pat::Paren(paren) => single_pat_ident(&paren.pat),
        _ => None,
    }
}

/// `Pipe` trait.
trait Pipe: Sized {
    /// `pipe` method.
    fn pipe<T>(self, f: impl FnOnce(Self) -> T) -> T {
        f(self)
    }
}

impl<T> Pipe for T {}

/// `MatchesArgs` struct.
struct MatchesArgs {
    /// `_expr` item.
    _expr: syn::Expr,
    /// `_guard` item.
    _guard: Option<syn::Expr>,
    /// `pattern` item.
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
            _expr: expr,
            _guard: guard,
            pattern,
        })
    }
}
