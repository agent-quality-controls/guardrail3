//! AST-based source analysis helpers using syn.
//!
//! These functions parse Rust source into an AST and inspect it
//! structurally — no grep, no false positives from strings/comments.

use super::{ast_visitors, extra_visitors};
use proc_macro2 as _; // reason: span-locations feature needed for syn span.start()

pub use super::ast_visitors::{
    GardeSkipInfo, struct_has_non_exempt_fields, struct_has_non_primitive_fields,
};
pub use super::extra_visitors::IgnoreReasonInfo;

/// A source location (1-based line number) paired with a descriptive string (lint name, method name, etc.).
pub(super) type Located = (usize, String);

/// Information about a `#[cfg_attr(..., allow(...))]` attribute.
#[derive(Debug)]
pub struct CfgAttrAllowInfo {
    /// 1-based line number.
    pub line: usize,
    /// Lint name (e.g. `dead_code`, `clippy::unwrap_used`).
    pub lint: String,
    /// Whether the cfg condition is always true (e.g. `all()` with no args).
    pub is_always_true: bool,
}

/// Information about a `#[derive(...)]` attribute on an item.
#[derive(Debug)]
pub struct DeriveInfo {
    /// 1-based line number of the derive attribute.
    pub line: usize,
    /// Names of the derive macros (e.g. `["Deserialize", "Validate"]`).
    pub macros: Vec<String>,
    /// Whether the struct has at least one non-primitive field (String, Vec, custom types, etc.).
    /// `false` for enums, unit structs, or structs with only primitive fields.
    pub has_non_primitive_fields: bool,
    /// Name of the struct/enum (if identifiable from the item).
    pub name: Option<String>,
}

/// Parse a Rust source file. Returns `None` if parsing fails.
/// Strips UTF-8 BOM if present.
pub fn parse_file(source: &str) -> Option<syn::File> {
    let source = source.strip_prefix('\u{feff}').unwrap_or(source);
    syn::parse_file(source).ok()
}

/// Find `#![allow(...)]` crate-level attributes. Returns `(line, lint_name)`.
pub fn find_crate_level_allows(file: &syn::File) -> Vec<Located> {
    let mut out = Vec::new();
    for attr in &file.attrs {
        if matches!(attr.style, syn::AttrStyle::Inner(_)) {
            extract_allow_lints(attr, &mut out);
        }
    }
    out
}

/// Information about a `#![allow(...)]` inside an inline module.
#[derive(Debug)]
pub struct InlineModAllow {
    /// 1-based line number of the inner attribute.
    pub line: usize,
    /// The lint name (e.g. `clippy::all`).
    pub lint: String,
    /// The module path (e.g. `foo` or `foo::bar` for nested modules).
    pub module_path: String,
}

/// Find `#![allow(...)]` inner attributes inside inline `mod` blocks.
/// These are module-wide suppressions that apply to everything inside the module.
pub fn find_inline_mod_allows(file: &syn::File) -> Vec<InlineModAllow> {
    let mut out = Vec::new();
    for item in &file.items {
        if let syn::Item::Mod(m) = item {
            collect_mod_inner_allows(m, &m.ident.to_string(), &mut out);
        }
    }
    out
}

/// Recursively collect `#![allow(...)]` from inline module bodies.
fn collect_mod_inner_allows(item_mod: &syn::ItemMod, path: &str, out: &mut Vec<InlineModAllow>) {
    // Only inline modules (with a body) can have inner attributes
    let Some((_, items)) = &item_mod.content else {
        return;
    };

    // Check this module's attributes for inner #![allow(...)]
    for attr in &item_mod.attrs {
        if matches!(attr.style, syn::AttrStyle::Inner(_)) {
            let mut lints = Vec::new();
            extract_allow_lints(attr, &mut lints);
            for (line, lint) in lints {
                out.push(InlineModAllow {
                    line,
                    lint,
                    module_path: path.to_owned(),
                });
            }
        }
    }

    // Recurse into nested inline modules
    for item in items {
        if let syn::Item::Mod(nested) = item {
            let nested_path = format!("{path}::{}", nested.ident);
            collect_mod_inner_allows(nested, &nested_path, out);
        }
    }
}

/// Find `#[allow(...)]` item-level attributes. Returns `(line, lint_name)`.
pub fn find_item_allows(file: &syn::File) -> Vec<Located> {
    let mut v = ast_visitors::ItemAllowVisitor { out: Vec::new() };
    syn::visit::Visit::visit_file(&mut v, file);
    v.out
}

/// Find `#[cfg_attr(..., allow(...))]` attributes.
/// Returns rich info including whether the cfg condition is always true.
pub fn find_cfg_attr_allows(file: &syn::File) -> Vec<CfgAttrAllowInfo> {
    let mut out = Vec::new();
    for attr in &file.attrs {
        extract_cfg_attr_allow_lints(attr, &mut out);
    }
    let mut v = ast_visitors::CfgAttrAllowVisitor { out: &mut out };
    syn::visit::Visit::visit_file(&mut v, file);
    out
}

/// Snapshot the first cfg_attr allow into plain values for test assertions.
pub fn cfg_attr_allow_snapshot(
    allows: &[CfgAttrAllowInfo],
) -> (usize, Option<(usize, String, bool)>) {
    (
        allows.len(),
        allows
            .first()
            .map(|allow| (allow.line, allow.lint.clone(), allow.is_always_true)),
    )
}

/// Find lines with `#[garde(skip)]`.
pub fn find_garde_skips(file: &syn::File) -> Vec<usize> {
    let mut v = ast_visitors::GardeSkipVisitor { out: Vec::new() };
    syn::visit::Visit::visit_file(&mut v, file);
    v.out
}

/// Find `#[garde(skip)]` fields/types with explicit exemption classification.
pub fn find_garde_skips_with_types(file: &syn::File) -> Vec<GardeSkipInfo> {
    let mut v = ast_visitors::GardeSkipTypedVisitor { out: Vec::new() };
    syn::visit::Visit::visit_file(&mut v, file);
    v.out
}

/// Find lines with `unsafe` blocks or `unsafe fn` declarations.
pub fn find_unsafe_usage(file: &syn::File) -> Vec<usize> {
    let mut v = ast_visitors::UnsafeVisitor { out: Vec::new() };
    syn::visit::Visit::visit_file(&mut v, file);
    v.out
}

/// Find `todo!()`, `unimplemented!()`, `panic!()`. Returns `(line, macro_name)`.
pub fn find_forbidden_macros(file: &syn::File) -> Vec<Located> {
    let mut v = ast_visitors::ForbiddenMacroVisitor { out: Vec::new() };
    syn::visit::Visit::visit_file(&mut v, file);
    v.out
}

/// Find `.unwrap()` and `.expect()` calls. Returns `(line, method_name)`.
/// Skips calls in functions/modules annotated with `#[allow(clippy::unwrap_used)]`
/// or `#[allow(clippy::expect_used)]`.
pub fn find_unwrap_expect(file: &syn::File) -> Vec<Located> {
    let mut v = ast_visitors::UnwrapExpectVisitor::default();
    syn::visit::Visit::visit_file(&mut v, file);
    v.out
}

/// Find all `#[derive(...)]` attributes in a parsed file.
/// Returns one `DeriveInfo` per derive attribute found, with its line and macro names.
pub fn find_derive_attributes(file: &syn::File) -> Vec<DeriveInfo> {
    let mut v = ast_visitors::DeriveVisitor { out: Vec::new() };
    syn::visit::Visit::visit_file(&mut v, file);
    v.out
}

/// Find `use std::fs` import lines. Skips imports gated by `#[cfg(test)]`.
pub fn find_std_fs_imports(file: &syn::File) -> Vec<usize> {
    file.items
        .iter()
        .filter_map(|item| {
            if let syn::Item::Use(u) = item {
                // Skip cfg(test)-gated imports
                if u.attrs.iter().any(is_cfg_test_attr) {
                    return None;
                }
                if use_tree_matches_std_fs(&u.tree) {
                    return Some(syn::spanned::Spanned::span(u).start().line);
                }
            }
            None
        })
        .collect()
}

/// Find inline `std::fs::*` calls (e.g., `std::fs::read_to_string(...)`).
///
/// Skips calls inside `#[cfg(test)]` functions/modules and functions/modules
/// annotated with `#[allow(clippy::disallowed_methods)]`.
pub fn find_inline_std_fs_calls(file: &syn::File) -> Vec<usize> {
    let mut v = extra_visitors::InlineStdFsVisitor {
        out: Vec::new(),
        in_cfg_test: false,
        in_allowed_scope: false,
    };
    syn::visit::Visit::visit_file(&mut v, file);
    v.out
}

/// Check if the file contains at least one `#[test]` or `#[tokio::test]` attribute.
pub fn has_test_attribute(file: &syn::File) -> bool {
    let mut v = ast_visitors::TestAttrVisitor { found: false };
    syn::visit::Visit::visit_file(&mut v, file);
    v.found
}

/// Count `pub fn` declarations (including in impl blocks and traits).
pub fn count_pub_fn_decls(file: &syn::File) -> usize {
    let mut v = ast_visitors::PubFnVisitor { count: 0 };
    syn::visit::Visit::visit_file(&mut v, file);
    v.count
}

/// Count `#[test]` and `#[tokio::test]` attributes.
pub fn count_test_attrs(file: &syn::File) -> usize {
    let mut v = ast_visitors::TestCountVisitor { count: 0 };
    syn::visit::Visit::visit_file(&mut v, file);
    v.count
}

/// Find `#[ignore]` attributes and any associated reason text.
/// Requires the original source to extract same-line and previous-line reason comments.
pub fn find_ignore_reasons(file: &syn::File, source: &str) -> Vec<IgnoreReasonInfo> {
    let mut v = extra_visitors::IgnoreVisitor {
        lines: source.lines().collect(),
        findings: Vec::new(),
    };
    syn::visit::Visit::visit_file(&mut v, file);
    v.findings
}

/// Find `#[ignore]` attributes without a reason.
/// Returns 1-based line numbers of violations.
pub fn find_ignore_without_reason(file: &syn::File, source: &str) -> Vec<usize> {
    find_ignore_reasons(file, source)
        .into_iter()
        .filter(|finding| finding.reason.is_none())
        .map(|finding| finding.line)
        .collect()
}

/// Count top-level `use` statements.
pub fn count_use_statements(file: &syn::File) -> usize {
    file.items
        .iter()
        .filter(|item| matches!(item, syn::Item::Use(_)))
        .count()
}

// ---------------------------------------------------------------------------
// Internal helpers (shared with ast_visitors via pub(super))
// ---------------------------------------------------------------------------

pub(super) fn span_line(span: proc_macro2::Span) -> usize {
    span.start().line
}

#[cfg(test)]
#[path = "ast_helpers_tests/mod.rs"]
mod ast_helpers_tests;

/// Check if an attribute is `#[allow(clippy::X)]` where X matches the given lint name.
fn has_allow_lint(attr: &syn::Attribute, lint_name: &str) -> bool {
    if !attr.path().is_ident("allow") {
        return false;
    }
    let Ok(nested) = attr.parse_args_with(
        syn::punctuated::Punctuated::<syn::Path, syn::Token![,]>::parse_terminated,
    ) else {
        return false;
    };
    nested.iter().any(|path| {
        // Match clippy::lint_name (exactly 2 segments)
        let mut iter = path.segments.iter();
        let Some(first) = iter.next() else {
            return false;
        };
        let Some(second) = iter.next() else {
            return false;
        };
        iter.next().is_none() && first.ident == "clippy" && second.ident == lint_name
    })
}

/// Check if any attribute in a slice has `#[allow(clippy::X)]` for a given lint name.
pub(super) fn attrs_have_allow_lint(attrs: &[syn::Attribute], lint_name: &str) -> bool {
    attrs.iter().any(|a| has_allow_lint(a, lint_name))
}

pub(super) fn extract_allow_lints(attr: &syn::Attribute, out: &mut Vec<Located>) {
    if !attr.path().is_ident("allow") {
        return;
    }
    let line = span_line(syn::spanned::Spanned::span(attr));
    if let Ok(nested) = attr
        .parse_args_with(syn::punctuated::Punctuated::<syn::Path, syn::Token![,]>::parse_terminated)
    {
        for path in &nested {
            out.push((line, path_to_string(path)));
        }
    }
}

pub(super) fn extract_cfg_attr_allow_lints(attr: &syn::Attribute, out: &mut Vec<CfgAttrAllowInfo>) {
    if !attr.path().is_ident("cfg_attr") {
        return;
    }
    let line = span_line(syn::spanned::Spanned::span(attr));
    let syn::Meta::List(meta_list) = &attr.meta else {
        return;
    };
    let Ok(args) = meta_list.parse_args_with(
        syn::punctuated::Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated,
    ) else {
        return;
    };
    let mut args = args.into_iter();
    let Some(condition) = args.next() else {
        return;
    };
    let always_true = is_cfg_meta_always_true(&condition);
    for meta in args {
        collect_cfg_attr_allow_lints_from_meta(&meta, line, always_true, out);
    }
}

/// Detect if a `cfg_attr` condition is always true.
/// Currently detects `all()` with no arguments.
fn is_cfg_meta_always_true(meta: &syn::Meta) -> bool {
    let syn::Meta::List(list) = meta else {
        return false;
    };
    list.path.is_ident("all") && list.tokens.is_empty()
}

fn collect_cfg_attr_allow_lints_from_meta(
    meta: &syn::Meta,
    line: usize,
    always_true: bool,
    out: &mut Vec<CfgAttrAllowInfo>,
) {
    let syn::Meta::List(inner) = meta else {
        return;
    };
    if inner.path.is_ident("allow") {
        if let Ok(paths) = inner.parse_args_with(
            syn::punctuated::Punctuated::<syn::Path, syn::Token![,]>::parse_terminated,
        ) {
            for path in &paths {
                out.push(CfgAttrAllowInfo {
                    line,
                    lint: path_to_string(path),
                    is_always_true: always_true,
                });
            }
        }
        return;
    }
    if !inner.path.is_ident("cfg_attr") {
        return;
    }
    let Ok(args) = inner.parse_args_with(
        syn::punctuated::Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated,
    ) else {
        return;
    };
    let mut args = args.into_iter();
    let Some(condition) = args.next() else {
        return;
    };
    let nested_always_true = always_true && is_cfg_meta_always_true(&condition);
    for nested in args {
        collect_cfg_attr_allow_lints_from_meta(&nested, line, nested_always_true, out);
    }
}

fn use_tree_matches_std_fs(tree: &syn::UseTree) -> bool {
    if let syn::UseTree::Path(p) = tree {
        if p.ident == "std" {
            return use_subtree_is_fs(&p.tree);
        }
    }
    false
}

fn use_subtree_is_fs(tree: &syn::UseTree) -> bool {
    match tree {
        syn::UseTree::Name(n) => n.ident == "fs",
        syn::UseTree::Path(p) => p.ident == "fs",
        syn::UseTree::Rename(r) => r.ident == "fs",
        syn::UseTree::Group(g) => g.items.iter().any(use_subtree_is_fs),
        syn::UseTree::Glob(_) => true, // `use std::*` — glob imports everything including fs
    }
}

pub(super) fn path_to_string(path: &syn::Path) -> String {
    path.segments
        .iter()
        .map(|seg| seg.ident.to_string())
        .collect::<Vec<_>>()
        .join("::")
}

/// Check if an attribute is `#[cfg(test)]`.
pub fn is_cfg_test_attr(attr: &syn::Attribute) -> bool {
    if !attr.path().is_ident("cfg") {
        return false;
    }
    let Ok(nested) = attr.parse_args::<syn::Ident>() else {
        return false;
    };
    nested == "test"
}

/// Extract the identifier (name) from a syn Item, if it has one.
pub(super) const fn item_ident(item: &syn::Item) -> Option<&syn::Ident> {
    match item {
        syn::Item::Fn(f) => Some(&f.sig.ident),
        syn::Item::Struct(s) => Some(&s.ident),
        syn::Item::Enum(e) => Some(&e.ident),
        syn::Item::Mod(m) => Some(&m.ident),
        syn::Item::Trait(t) => Some(&t.ident),
        syn::Item::Type(t) => Some(&t.ident),
        syn::Item::Const(c) => Some(&c.ident),
        syn::Item::Static(s) => Some(&s.ident),
        syn::Item::ExternCrate(e) => Some(&e.ident),
        syn::Item::TraitAlias(t) => Some(&t.ident),
        syn::Item::Union(u) => Some(&u.ident),
        syn::Item::ForeignMod(_)
        | syn::Item::Impl(_)
        | syn::Item::Macro(_)
        | syn::Item::Use(_)
        | syn::Item::Verbatim(_) => None,
        _ => None,
    }
}

pub(super) fn item_attrs(item: &syn::Item) -> &[syn::Attribute] {
    match item {
        syn::Item::Fn(f) => &f.attrs,
        syn::Item::Struct(s) => &s.attrs,
        syn::Item::Enum(e) => &e.attrs,
        syn::Item::Impl(i) => &i.attrs,
        syn::Item::Mod(m) => &m.attrs,
        syn::Item::Trait(t) => &t.attrs,
        syn::Item::Type(t) => &t.attrs,
        syn::Item::Const(c) => &c.attrs,
        syn::Item::Static(s) => &s.attrs,
        syn::Item::Use(u) => &u.attrs,
        syn::Item::ForeignMod(f) => &f.attrs,
        syn::Item::ExternCrate(e) => &e.attrs,
        syn::Item::Macro(m) => &m.attrs,
        syn::Item::TraitAlias(t) => &t.attrs,
        syn::Item::Union(u) => &u.attrs,
        syn::Item::Verbatim(_) => &[],
        _ => &[],
    }
}

pub(super) fn impl_item_attrs(item: &syn::ImplItem) -> &[syn::Attribute] {
    match item {
        syn::ImplItem::Fn(f) => &f.attrs,
        syn::ImplItem::Type(t) => &t.attrs,
        syn::ImplItem::Const(c) => &c.attrs,
        syn::ImplItem::Macro(m) => &m.attrs,
        syn::ImplItem::Verbatim(_) => &[],
        _ => &[],
    }
}

/// Extract attributes from an expression (if the variant carries them).
pub(super) fn expr_attrs(expr: &syn::Expr) -> &[syn::Attribute] {
    match expr {
        syn::Expr::Array(e) => &e.attrs,
        syn::Expr::Assign(e) => &e.attrs,
        syn::Expr::Async(e) => &e.attrs,
        syn::Expr::Await(e) => &e.attrs,
        syn::Expr::Binary(e) => &e.attrs,
        syn::Expr::Block(e) => &e.attrs,
        syn::Expr::Break(e) => &e.attrs,
        syn::Expr::Call(e) => &e.attrs,
        syn::Expr::Cast(e) => &e.attrs,
        syn::Expr::Closure(e) => &e.attrs,
        syn::Expr::Const(e) => &e.attrs,
        syn::Expr::Continue(e) => &e.attrs,
        syn::Expr::Field(e) => &e.attrs,
        syn::Expr::ForLoop(e) => &e.attrs,
        syn::Expr::Group(e) => &e.attrs,
        syn::Expr::If(e) => &e.attrs,
        syn::Expr::Index(e) => &e.attrs,
        syn::Expr::Let(e) => &e.attrs,
        syn::Expr::Lit(e) => &e.attrs,
        syn::Expr::Loop(e) => &e.attrs,
        syn::Expr::Macro(e) => &e.attrs,
        syn::Expr::Match(e) => &e.attrs,
        syn::Expr::MethodCall(e) => &e.attrs,
        syn::Expr::Paren(e) => &e.attrs,
        syn::Expr::Path(e) => &e.attrs,
        syn::Expr::Range(e) => &e.attrs,
        syn::Expr::Reference(e) => &e.attrs,
        syn::Expr::Repeat(e) => &e.attrs,
        syn::Expr::Return(e) => &e.attrs,
        syn::Expr::Struct(e) => &e.attrs,
        syn::Expr::Try(e) => &e.attrs,
        syn::Expr::TryBlock(e) => &e.attrs,
        syn::Expr::Tuple(e) => &e.attrs,
        syn::Expr::Unary(e) => &e.attrs,
        syn::Expr::Unsafe(e) => &e.attrs,
        syn::Expr::Verbatim(_) => &[],
        syn::Expr::While(e) => &e.attrs,
        syn::Expr::Yield(e) => &e.attrs,
        syn::Expr::Infer(e) => &e.attrs,
        _ => &[],
    }
}
